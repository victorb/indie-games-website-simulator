use crate::prelude::*;
use avian2d::prelude::*;

pub struct RequestsPlugin;

impl Plugin for RequestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity(Vec2::NEG_Y * 100.0))
            .add_systems(
                FixedUpdate,
                (
                    assign_requests_to_closest_load_balancer,
                    move_requests_to_destination,
                    move_dropped_requests,
                    increment_request_elapsed_time,
                )
                    .run_if(in_state(GameState::Running)),
            );
        //.add_systems(Update, draw_children_ui);
        // app.register_component_as::<dyn Request, RequestPageView>()
        //     .register_component_as::<dyn Request, RequestPurchase>()
        //     .register_component_as::<dyn Request, RequestUpload>()
        //     .register_component_as::<dyn Request, RequestDownload>()
        //     // .add_systems(Update, show_tooltips)
        //     ;
    }
}

#[derive(Component)]
pub struct Request {
    pub destination: Option<Entity>,
    pub age: f32,
    pub size: usize,
}

impl Default for Request {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(1..32);
        Self {
            destination: None,
            age: 0.0,
            size,
        }
    }
}

#[derive(Component)]
pub struct DroppedRequest;

// If any Entity currently "holds"/"own" this request, and sets this to "false" when it should be
#[derive(Component)]
pub struct Owned {
    pub by: Entity,
}

#[derive(Component)]
struct RequestPageView;

// #[derive(Component)]
// struct RequestPurchase;
//
// #[derive(Component)]
// struct RequestUpload;
//
// #[derive(Component)]
// struct RequestDownload;

// impl Request for RequestPageView {
//     fn asset_path(&self) -> &str {
//         "request.png"
//     }
// }

// impl Request for RequestPurchase {
//     fn asset_path(&self) -> &str {
//         "request.png"
//     }
// }

// impl Request for RequestUpload {
//     fn asset_path(&self) -> &str {
//         "request.png"
//     }
// }

// impl Request for RequestDownload {
//     fn asset_path(&self) -> &str {
//         "request.png"
//     }
// }

#[derive(Component)]
pub struct RequestInfoText;

pub struct SpawnRequest {}

impl Command for SpawnRequest {
    fn apply(self, world: &mut World) {
        let handle = world.resource_scope(|_world, ass: Mut<ImageAssets>| {
            let handle: Handle<Image> = ass.request.clone();
            handle
        });

        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(-250.0..250.0);
        // let offset_y = rng.gen_range(-10.0..10.0);

        let component = RequestPageView {};

        println!("Spawning RequestPageView");

        world.spawn((
            LevelOwned,
            Name::new("RequestPageView"),
            SpriteBundle {
                texture: handle,
                transform: Transform::from_xyz(offset_x, 300.0, 10.0).with_scale(Vec3::splat(0.1)),
                ..default()
            },
            component,
            Request::default(),
            Pickable::IGNORE,
        ));
        // .with_children(|subcommands| {
        //     subcommands.spawn((
        //         Text2dBundle {
        //             transform: Transform::from_translation(Vec3::new(150.0, 100.0, 10.0)),
        //             text: Text::from_section(
        //                 "Info",
        //                 TextStyle {
        //                     font_size: 64.0,
        //                     ..default()
        //                 },
        //             )
        //             .with_justify(JustifyText::Left),
        //             text_anchor: bevy::sprite::Anchor::TopLeft,
        //             ..default()
        //         },
        //         RequestInfoText,
        //     ));
        // });
    }
}

fn draw_children_ui(
    q_requests: Query<(Entity, &Request, &Children)>,
    mut q_child: Query<&mut Text>,
) {
    for (entity, request, children) in q_requests.iter() {
        // Extract
        let size = request.size;
        let age = request.age;

        // Draw
        for &child in children.iter() {
            let mut text = q_child.get_mut(child).unwrap();
            text.sections[0].value =
                format!("Request\nEntity: {entity}\nSize: {size}\nAge: {age:.1}");
        }
    }
}

fn find_closest(origin: Vec3, items: Vec<(Entity, Transform)>) -> Vec<(Entity, Vec3, f32)> {
    let mut closest: Vec<(Entity, Vec3, f32)> = items
        .into_iter()
        .map(|(entity, transform)| {
            let distance = transform.translation.distance(origin);
            (entity, transform.translation, distance)
        })
        .collect();

    closest.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    closest.truncate(10);
    closest
}

fn assign_requests_to_closest_load_balancer(
    mut q_requests: Query<(&Transform, &mut Request)>,
    q_servers: Query<(Entity, &Transform), With<Server>>,
) {
    for (transform, mut request) in q_requests.iter_mut() {
        if request.destination.is_some() {
            continue; // No need to adjust destination
        }
        let items: Vec<(Entity, Transform)> = q_servers.iter().map(|(e, t)| (e, *t)).collect();

        let closest_n = find_closest(transform.translation, items);

        let (e_server, _t_server, _distance) = closest_n.first().unwrap();

        request.destination = Some(*e_server);
    }
}

fn move_requests_to_destination(
    mut commands: Commands,
    time: Res<Time>,
    mut q_requests: Query<
        (Entity, &mut Transform, &mut Request),
        (Without<Owned>, Without<DroppedRequest>),
    >,
    mut q_target: Query<(Entity, &Transform, &mut Server), Without<Request>>,
    mut stats: ResMut<GameStats>,
    mut evs: EventWriter<PlaySound>,
) {
    for (e_request, mut t_request, request) in q_requests.iter_mut() {
        if request.destination.is_none() {
            continue; // We don't have any destination ?!
        }
        let speed = 128.0;

        let (e_target, t_target, mut server) =
            q_target.get_mut(request.destination.unwrap()).unwrap();

        if t_request.translation.distance(t_target.translation) > 1.0 {
            let direction = (t_target.translation - t_request.translation).normalize();

            t_request.translation += direction * speed * time.delta_seconds();
        } else {
            println!("Move done!");
            if server.is_busy() {
                // Drop request
                commands.entity(e_request).insert(DroppedRequest);
                stats.dropped_requests += 1;
                evs.send(PlaySound(Sound::DroppedRequest));
            } else {
                t_request.translation = t_target.translation.with_z(10.0);
                commands.entity(e_request).insert(Owned { by: e_target });
                server.add_request(e_request);
            }
        }
    }
}

fn move_dropped_requests(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, Option<&RigidBody>), With<DroppedRequest>>,
    mut dropped_timers: Local<HashMap<Entity, Timer>>,
) {
    // let destination = Vec3::new(1000.0, 1000.0, 10.0);
    // let drop_speed = 512.0;

    // RigidBody::Dynamic

    let mut timers_to_drop = vec![];

    for (entity, timer) in dropped_timers.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            timers_to_drop.push(entity.clone());
        }
    }
    for entity in timers_to_drop.iter() {
        dropped_timers.remove_entry(entity);
        commands.entity(*entity).insert(ToRemove);
    }

    for (e_request, t_request, rigid_body) in query.iter_mut() {
        match rigid_body {
            Some(_rigid_body) => {
                if t_request.translation.y < -500.0 {
                    commands.entity(e_request).insert(ToRemove);
                }
            }
            None => {
                let mut rng = rand::thread_rng();
                let torque = rng.gen_range(-100.0..100.0);
                let mut up_angle = Vec2::ZERO;
                up_angle.y = 1000.0;
                up_angle.x = rng.gen_range(-500.0..500.0);
                commands
                    .entity(e_request)
                    .insert((
                        RigidBody::Dynamic,
                        Collider::circle(16.0),
                        ExternalAngularImpulse::new(torque).with_persistence(false),
                        ExternalImpulse::new(up_angle).with_persistence(false),
                        // ExternalTorque::new(torque).with_persistence(false),
                    ))
                    .despawn_descendants();
                dropped_timers.insert(e_request, Timer::from_seconds(2.5, TimerMode::Once));

                let tween = Tween::new(
                    EaseFunction::ExponentialIn,
                    Duration::from_millis(5000),
                    // Duration::from_millis(1000),
                    TransformScaleLens {
                        start: Vec3::splat(0.1),
                        end: Vec3::splat(0.001),
                    },
                )
                .with_completed_event(0);

                commands.entity(e_request).try_insert(Animator::new(tween));
            }
        }
        // let current_pos = t_request.translation;
        // let distance = current_pos.distance(destination);
        // if distance > 5.0 {
        //     let direction = (destination - t_request.translation).normalize();
        //     t_request.translation += direction * (drop_speed * 1.5) * time.delta_seconds();
        // } else {
        //     // println!("Deleting now then");
        //     commands.entity(e_request).insert(ToRemove);
        // }
    }
}

fn increment_request_elapsed_time(time: Res<Time>, mut query: Query<&mut Request>) {
    for mut request in query.iter_mut() {
        request.age += time.delta_seconds();
    }
}

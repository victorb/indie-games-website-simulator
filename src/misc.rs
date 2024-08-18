use avian2d::prelude::*;
use bevy_tweening::{component_animator_system, AnimationSystem, TweenCompleted, TweeningPlugin};

use crate::prelude::*;

pub struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_systems(FixedLast, handle_removals)
            .add_systems(OnEnter(GameState::GameCompleted), clear_transforms)
            .add_systems(
                Update,
                (
                    component_animator_system::<Transform>.in_set(AnimationSystem::AnimationUpdate),
                    handle_tween_complete.before(handle_removals),
                    (
                        game_completed_stuff,
                        despawn_rigid_bodies_below_screen
                            .run_if(on_timer(Duration::from_millis(100))),
                    )
                        .run_if(in_state(GameState::GameCompleted)),
                ),
            );
    }
}

#[derive(Component)]
pub struct ToRemove;

fn handle_removals(mut commands: Commands, query: Query<Entity, With<ToRemove>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn handle_tween_complete(
    mut evs: EventReader<TweenCompleted>,
    mut commands: Commands,
    query: Query<Entity, With<Animator<Transform>>>,
) {
    for ev in evs.read() {
        // println!("Remove animator!\n{:#?}", ev.entity);
        // commands.entity(ev.entity).remove::<Animator<Transform>>();
        if query.get(ev.entity).is_ok() {
            // It's safe to remove the component
            commands.entity(ev.entity).remove::<Animator<Transform>>();
        } else {
            // Handle the case where the entity may not have the component or might not be valid
            eprintln!(
                "Entity ({:?}) does not have Animator<Transform> or is no longer valid",
                ev.entity
            );
        }
    }
}

fn clear_transforms(
    mut commands: Commands,
    query: Query<Entity, (With<Transform>, Without<Camera>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(ToRemove);
    }
}

fn game_completed_stuff(
    time: Res<Time>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut timer: Local<Timer>,
    q_windows: Query<&Window>,
) {
    if timer.tick(time.delta()).just_finished() {
        let bottom_y = -q_windows.get_single().unwrap().height() / 2.0;

        timer.set_duration(Duration::from_millis(250));
        timer.reset();
        // Spawn servers and requests that are flying everwhere
        let mut rng = rand::thread_rng();
        let torque = rng.gen_range(-100.0..100.0);
        let mut up_angle = Vec2::ZERO;
        let rand_x = rng.gen_range(-500.0..500.0);

        up_angle.y = rng.gen_range(2500.0..3500.0);

        up_angle.x = rng.gen_range(-500.0..500.0);

        let handle = if rand::random() {
            image_assets.request.clone()
        } else {
            image_assets.server.clone()
        };
        commands
            .spawn_empty()
            .insert((
                SpriteBundle {
                    texture: handle,
                    transform: Transform::from_xyz(rand_x, bottom_y, -1.0)
                        .with_scale(Vec3::splat(0.1)),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::rectangle(256.0, 256.0),
                ColliderDensity(0.013),
                ExternalAngularImpulse::new(torque).with_persistence(false),
                ExternalImpulse::new(up_angle).with_persistence(false),
                // ExternalTorque::new(torque).with_persistence(false),
            ))
            .despawn_descendants();
    }
}

fn despawn_rigid_bodies_below_screen(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &RigidBody)>,
) {
    println!("Found {} thingies", query.iter().len());
    for (entity, transform, _) in query.iter() {
        if transform.translation.y < -500.0 {
            commands.entity(entity).insert(ToRemove);
        }
    }
}

#[macro_export]
macro_rules! create_markers {
    ($($name:ident),*) => {
        $(
            #[derive(Component)]
            pub struct $name;
        )*
    };
}

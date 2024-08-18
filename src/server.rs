use crate::prelude::*;

// const SERVER_SPACING: f32 = 100.0;
const BASELINE_MS_PROCESSING: u64 = 2000;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpgradeServerCPUEvent>()
            .add_event::<UpgradeServerQueueEvent>()
            .add_event::<AddNewServer>()
            .add_event::<ChangeServerModeEvent>()
            .add_event::<SetServerOutputEvent>()
            .add_event::<AlignServersEvent>()
            .add_event::<ResetUpgradesEvent>()
            .init_resource::<SelectedServerForOutputs>()
            .init_resource::<UpgradePoints>()
            .add_systems(Update, draw_children_ui)
            .add_systems(
                Update,
                (
                    handle_upgrade_server_cpu,
                    handle_upgrade_queue_size,
                    handle_change_server_mode,
                    handle_reset_upgrades,
                    handle_select_outputs,
                    add_new_server,
                    // TODO only in dev
                    increment_upgrade_points.run_if(input_just_pressed(KeyCode::NumpadAdd)),
                    decrement_upgrade_points.run_if(input_just_pressed(KeyCode::NumpadSubtract)),
                    align_servers_to_grid.run_if(on_event::<AlignServersEvent>()),
                    align_queued_requests,
                )
                    .run_if(in_state(GameState::Planning).or_else(in_state(GameState::Running))),
            )
            .add_systems(FixedUpdate, process_requests);
    }
}

fn increment_upgrade_points(mut res: ResMut<UpgradePoints>) {
    res.total += 1;
}

fn decrement_upgrade_points(mut res: ResMut<UpgradePoints>) {
    res.total -= 1;
}

#[derive(Resource, Default)]
pub struct SelectedServerForOutputs(pub Option<Entity>);

#[derive(Component)]
struct ServerInfoText;

#[derive(Event)]
pub struct AddNewServer(pub usize);

#[derive(Resource, Default)]
pub struct UpgradePoints {
    pub total: usize,
    pub assigned: usize,
}

impl UpgradePoints {
    pub fn can_spend_points(&self, to_spend: usize) -> bool {
        self.assigned + to_spend <= self.total
    }
}

#[derive(Debug)]
pub enum ServerMode {
    Process,
    Proxy,
}

#[derive(Debug)]
pub enum Filter {
    SizeGreater(usize),
    AgeGreater(f32),
}

#[derive(Component, Debug)]
pub struct Server {
    pub mode: ServerMode,
    pub processing_power: usize,
    pub queue_size: usize,
    pub queued_requests: VecDeque<Entity>,
    pub current_request: Option<Entity>,
    // Timer that counts down to zero, and once zero, it has processed the request
    pub current_progress: Timer,
    // Proxy fields
    // Which servers are currently connected
    pub outputs: Vec<Entity>,
    // Which index we're currently on in our round-robin
    current_output_index: usize,
}

impl Server {
    pub fn reset_progress(&mut self) {
        // Proxy mode doubles our processing power
        let duration = match self.mode {
            ServerMode::Process => BASELINE_MS_PROCESSING / self.processing_power as u64,
            ServerMode::Proxy => BASELINE_MS_PROCESSING / (self.processing_power * 2) as u64,
        };
        self.current_progress = Timer::new(Duration::from_millis(duration), TimerMode::Once);
    }
    pub fn add_request(&mut self, request: Entity) {
        match self.current_request {
            Some(_) => {
                // Add to queue if possible
                self.queued_requests.push_back(request);
            }
            None => {
                // No current request, assign this
                self.current_request = Some(request);
            }
        }
    }
    pub fn is_busy(&self) -> bool {
        println!("Current Request: {:?}", self.current_request);
        println!(
            "Queue Length: {}, Queue Size: {}",
            self.queued_requests.len(),
            self.queue_size
        );
        self.current_request.is_some() && self.queued_requests.len() >= self.queue_size
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            mode: ServerMode::Process,
            processing_power: 1,
            queue_size: 0,
            queued_requests: VecDeque::new(),
            current_request: None,
            current_progress: Timer::new(
                Duration::from_millis(BASELINE_MS_PROCESSING),
                TimerMode::Once,
            ),
            // Proxy fields
            outputs: vec![],
            current_output_index: 0,
        }
    }
}

#[derive(Event)]
pub struct UpgradeServerCPUEvent(pub Entity);

impl From<ListenerInput<Pointer<Click>>> for UpgradeServerCPUEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        println!("converting event to UpgradeServerCPUEvent");
        UpgradeServerCPUEvent(event.target)
    }
}

#[derive(Event)]
pub struct UpgradeServerQueueEvent(pub Entity);

impl From<ListenerInput<Pointer<Click>>> for UpgradeServerQueueEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        UpgradeServerQueueEvent(event.target)
    }
}

#[derive(Event)]
pub struct ChangeServerModeEvent(pub Entity);

impl From<ListenerInput<Pointer<Click>>> for ChangeServerModeEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        ChangeServerModeEvent(event.target)
    }
}

#[derive(Event)]
pub struct SetServerOutputEvent(pub Entity);

impl From<ListenerInput<Pointer<Click>>> for SetServerOutputEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        SetServerOutputEvent(event.target)
    }
}

#[derive(Event)]
pub struct ResetUpgradesEvent(pub Entity);

impl From<ListenerInput<Pointer<Click>>> for ResetUpgradesEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        ResetUpgradesEvent(event.target)
    }
}

pub fn handle_upgrade_server_cpu(
    mut evs: EventReader<UpgradeServerCPUEvent>,
    mut query: Query<(&mut Server, &PickSelection)>,
    mut upgrade_points: ResMut<UpgradePoints>,
) {
    for _ev in evs.read() {
        println!("Upgrading processing power");
        for (mut server, pick_selection) in query.iter_mut() {
            if pick_selection.is_selected {
                let to_spend = server.processing_power.max(1);
                if upgrade_points.can_spend_points(to_spend) {
                    // TODO tunable
                    upgrade_points.assigned += to_spend;
                    server.processing_power += 1;
                    server.reset_progress();
                } else {
                    println!(
                        "Couldn't upgrade! Current assigned points: {}, max points: {}",
                        upgrade_points.assigned, upgrade_points.total
                    );
                }
            }
        }
    }
}

pub fn handle_upgrade_queue_size(
    // selection: Res<Selection>,
    mut evs: EventReader<UpgradeServerQueueEvent>,
    mut query: Query<(&mut Server, &PickSelection)>,
    mut upgrade_points: ResMut<UpgradePoints>,
) {
    for _ev in evs.read() {
        println!("Upgrading queue size");
        for (mut server, pick_selection) in query.iter_mut() {
            if pick_selection.is_selected {
                // let to_spend = server.queue_size.max(1);
                let to_spend = 1;
                if upgrade_points.can_spend_points(to_spend) {
                    // TODO tunable
                    upgrade_points.assigned += to_spend;
                    server.queue_size += 1;
                    server.reset_progress();
                } else {
                    println!(
                        "Couldn't upgrade! Current assigned points: {}, max points: {}",
                        upgrade_points.assigned, upgrade_points.total
                    );
                }
            }
        }
    }
}

pub fn handle_reset_upgrades(
    mut evs: EventReader<ResetUpgradesEvent>,
    mut query: Query<(&mut Server, &PickSelection)>,
    mut upgrade_points: ResMut<UpgradePoints>,
) {
    for _ev in evs.read() {
        println!("Resetting upgrades");
        for (mut server, pick_selection) in query.iter_mut() {
            if pick_selection.is_selected {
                // how many points to refund for cpu
                let cpu_points = if server.processing_power > 1 {
                    ((server.processing_power - 1) * server.processing_power) / 2
                } else {
                    0
                };

                // queue upgrades are easy
                let queue_points = server.queue_size;

                let total_points_to_return = cpu_points + queue_points;

                // reset to default
                server.processing_power = 1;
                server.queue_size = 0;
                server.reset_progress();

                // finally refund, wooo
                upgrade_points.assigned -= total_points_to_return;

                println!(
                    "Reset server. Returned {} upgrade points. Current assigned points: {}, max points: {}",
                    total_points_to_return, upgrade_points.assigned, upgrade_points.total
                );
            }
        }
    }
}

pub fn handle_select_outputs(
    mut evs: EventReader<SetServerOutputEvent>,
    mut query: Query<(Entity, &mut Server, &mut PickSelection)>,
    mut next_state: ResMut<NextState<EditMode>>,
    mut selected_server: ResMut<SelectedServerForOutputs>,
) {
    for _ev in evs.read() {
        println!("Setting new server outputs");
        for (entity, mut _server, mut pick_selection) in query.iter_mut() {
            // Now tell the user to select the servers to connect
            next_state.set(EditMode::Outputs);
            if pick_selection.is_selected {
                // This is the picked server, save somewhere
                pick_selection.is_selected = false;
                selected_server.0 = Some(entity);
            }
        }
    }
}

pub fn handle_change_server_mode(
    mut evs: EventReader<ChangeServerModeEvent>,
    mut query: Query<(&mut Server, &PickSelection, &mut Handle<Image>)>,
    image_assets: Res<ImageAssets>,
) {
    for _ev in evs.read() {
        println!("Switching server modes");
        for (mut server, pick_selection, mut handle) in query.iter_mut() {
            if pick_selection.is_selected {
                // TODO duplicated logic
                match server.mode {
                    ServerMode::Process => {
                        *handle = image_assets.server_proxy.clone();
                        server.mode = ServerMode::Proxy;
                    }
                    ServerMode::Proxy => {
                        *handle = image_assets.server.clone();
                        server.mode = ServerMode::Process;
                        // Reset outputs
                        server.outputs = vec![];
                    }
                };
                // server.processing_power += 1;
            }
        }
    }
}

pub fn handle_set_new_server_output() {
    // Get current position
}

fn process_requests(
    time: Res<Time>,
    mut commands: Commands,
    mut q_servers: Query<(Entity, &Transform, &mut Server)>,
    mut q_request: Query<
        (&mut Transform, &mut Request, Option<&Animator<Transform>>),
        Without<Server>,
    >,
    mut stats: ResMut<GameStats>,
    mut evs: EventWriter<PlaySound>,
) {
    for (e_server, t_server, mut server) in q_servers.iter_mut() {
        match server.current_request {
            Some(e_request) => {
                let (mut t_request, mut request, animator) = q_request.get_mut(e_request).unwrap();
                // Process request
                if server.current_progress.tick(time.delta()).just_finished() {
                    match server.mode {
                        ServerMode::Process => {
                            println!("Done processing request!");
                            // Done processing, reset!
                            commands.entity(e_request).insert(ToRemove);

                            // Check if there is more things to process
                            if server.queued_requests.len() > 0 {
                                server.current_request = server.queued_requests.pop_front();
                            } else {
                                server.current_request = None;
                            }
                            server.reset_progress();
                            stats.handled_requests += 1;
                            stats.response_times.push(request.age);
                            stats.update_avg_response_time();

                            evs.send(PlaySound(Sound::ServerProcess));

                            // // Handle purchase requests
                            // // let request = q_request.get(e_request).unwrap();
                            // let commission_rate = stats.purchase_commision;
                            // // let request_value = request.value.max(1.0);
                            // let request_value = 1.0; // TODO hardcoded...
                            // let commission = request_value * commission_rate;
                            // stats.money += commission;
                        }
                        ServerMode::Proxy => {
                            // Dont processing, Proxy it somewhere
                            println!("Proxing this request to other server");
                            if server.outputs.len() == 0 {
                                commands.entity(e_request).insert(DroppedRequest);
                                evs.send(PlaySound(Sound::DroppedRequest));
                                stats.dropped_requests += 1;
                                if server.queued_requests.len() > 0 {
                                    server.current_request = server.queued_requests.pop_front();
                                } else {
                                    server.current_request = None;
                                }
                                server.reset_progress();
                                continue;
                            }
                            // Proxy this request to one of our connections
                            //
                            if server.current_output_index == server.outputs.len() - 1
                                || server.current_output_index > server.outputs.len() - 1
                            {
                                server.current_output_index = 0;
                            } else {
                                server.current_output_index += 1;
                            }
                            let server_to_pass_on_to = server.outputs[server.current_output_index];

                            if server_to_pass_on_to == e_server {
                                // Tryinrg to pass to ourselves? Drop it
                                commands
                                    .entity(e_request)
                                    .insert(DroppedRequest)
                                    .remove::<Owned>();
                                evs.send(PlaySound(Sound::DroppedRequest));
                                stats.dropped_requests += 1;
                                if server.queued_requests.len() > 0 {
                                    server.current_request = server.queued_requests.pop_front();
                                } else {
                                    server.current_request = None;
                                }
                                server.reset_progress();
                                continue;
                            }

                            request.destination = Some(server_to_pass_on_to);
                            commands.entity(e_request).remove::<Owned>();

                            t_request.scale.y = 0.1;
                            // t_request.translation = t_server.translation;

                            if server.queued_requests.len() > 0 {
                                server.current_request = server.queued_requests.pop_front();
                            } else {
                                server.current_request = None;
                            }
                            server.reset_progress();
                            evs.send(PlaySound(Sound::ProxyProcess));
                        }
                    }
                } else {
                    // Currently processing request
                    t_request.scale.y = server.current_progress.fraction_remaining() / 10.0;
                    // In case user drags server around while processing
                    // t_request.translation = t_server.translation;
                    match animator {
                        Some(_) => {
                            // Already moving to position
                        }
                        None => {
                            // While we're processing request
                            let mut rng = rand::thread_rng();
                            let duration = rng.gen_range(500..1000);

                            let new_x = t_server.translation.x - 48.0;

                            let tween = Tween::new(
                                EaseFunction::BounceOut,
                                Duration::from_millis(duration),
                                // Duration::from_millis(1000),
                                TransformPositionLens {
                                    start: t_request.translation,
                                    end: t_server.translation.with_x(new_x),
                                },
                            )
                            .with_completed_event(0);

                            commands.entity(e_request).try_insert(Animator::new(tween));
                        }
                    }
                }
            }
            None => {
                // Nothing to do...
            }
        }
    }
}

fn draw_children_ui(q_servers: Query<(Entity, &Server, &Children)>, mut q_child: Query<&mut Text>) {
    for (entity, server, children) in q_servers.iter() {
        // Extract
        let power = server.processing_power;
        let queue_size = server.queue_size;
        let progress = server.current_progress.fraction() * 100.0;
        let mode = match server.mode {
            ServerMode::Process => "Process",
            ServerMode::Proxy => "Proxy",
        };

        let queued_requests = server.queued_requests.len();
        let connected_servers = server.outputs.len();

        // Draw
        for &child in children.iter() {
            if let Ok(mut text) = q_child.get_mut(child) {
                text.sections[0].value =
                    format!("Power: {power}\nMax Queue: {queue_size}\nPending:{queued_requests}",);
            }
        }
    }
}

#[derive(Component)]
pub struct ServerFilterOutput;

#[derive(Component)]
pub struct ServerOutput;

fn add_new_server(
    mut evs: EventReader<AddNewServer>,
    mut commands: Commands,
    q_servers: Query<Entity, With<Server>>,
    // mut q_load_balancer: Query<(Entity, &mut LoadBalancer)>,
    asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
    // mut evs_select: EventWriter<SelectEvent>,
) {
    for ev in evs.read() {
        #[allow(unused_assignments)]
        let mut x_offset = 0.0;
        for i in 0..ev.0 {
            x_offset = -200.0 + (i as f32 * 100.0);
            // Update load balancer with reference to all existing servers
            let mut servers: Vec<Entity> = q_servers.iter().sort::<Entity>().collect();
            // let font_handle = asset_server.load("fonts/MajorMonoDisplay-Regular.ttf");

            let new_server = commands
                .spawn((
                    // LevelOwned, // TODO don't want to replace these each level...
                    SpriteBundle {
                        texture: image_assets.server.clone(),
                        transform: Transform::from_xyz(x_offset, 0.0, 5.0)
                            .with_scale(Vec3::splat(0.25)),
                        ..default()
                    },
                    Server::default(),
                    PickableBundle::default(),
                    NoDeselect,
                    // On::<Pointer<Click>>::send_event::<SelectEvent>(),
                    On::<Pointer<Drag>>::send_event::<DragServerEvent>(),
                    // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                    //     transform.translation.x += drag.delta.x;
                    //     transform.translation.y -= drag.delta.y;
                    // }),
                    On::<Pointer<DragStart>>::send_event::<DragServerStartEvent>(),
                    On::<Pointer<DragEnd>>::send_event::<DragServerEndEvent>(),
                    // On::<Pointer<DragStart>>::target_commands_mut(|drag_start, target_commands| {
                    //     println!("Start Drag!");
                    // }),
                    // On::<Pointer<Drag>>::target_commands_mut(|drag, target_commands| {
                    //     println!("Dragging!");
                    // }),
                    // On::<Pointer<DragEnd>>::target_commands_mut(|drag_end, target_commands| {
                    //     println!("End Drag!");
                    // }),
                ))
                .with_children(|subcommands| {
                    subcommands.spawn((
                        Text2dBundle {
                            transform: Transform::from_translation(Vec3::new(-150.0, -160.0, 10.0)),
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font_size: 22.0 * 2.0, // Because of parent scale
                                    // font: font_handle,
                                    ..default()
                                },
                            )
                            .with_justify(JustifyText::Left),
                            text_anchor: bevy::sprite::Anchor::TopLeft,
                            ..default()
                        },
                        ServerInfoText,
                        Pickable::IGNORE,
                    ));
                })
                // .with_children(|subcommands| {
                //     let mesh = meshes.add(Circle { radius: 50.0 }).into();
                //     subcommands.spawn((
                //         MaterialMesh2dBundle {
                //             mesh,
                //             material: materials.add(Color::from(BLUE_400)),
                //             transform: Transform::from_xyz(150.0, -150.0, 10.0),
                //             ..default()
                //         },
                //         ServerOutput,
                //         Pickable::IGNORE,
                //     ));
                // })
                // .with_children(|subcommands| {
                //     let mesh = meshes.add(Circle { radius: 50.0 }).into();
                //     subcommands.spawn((
                //         MaterialMesh2dBundle {
                //             mesh,
                //             material: materials.add(Color::from(GREEN_400)),
                //             transform: Transform::from_xyz(-150.0, -150.0, 10.0),
                //             ..default()
                //         },
                //         ServerFilterOutput,
                //     ));
                // })
                .id();

            servers.push(new_server);

            // let (e_load_balancer, mut load_balancer) = q_load_balancer.get_single_mut().unwrap();
            // load_balancer.available_targets = servers;
        }
    }
}

const GRID_SIZE_X: f32 = 100.0;
const GRID_SIZE_Y: f32 = 100.0;

#[derive(Event)]
pub struct AlignServersEvent;

fn align_servers_to_grid(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &Server,
            Option<&Animator<Transform>>,
        ),
        Without<ToRemove>,
    >,
) {
    for (entity, transform, _, animator) in query.iter_mut() {
        // Compute the nearest grid point
        match animator {
            Some(_animator) => {
                // We're already moving it to the position
            }
            None => {
                let new_pos = Vec3::new(
                    (transform.translation.x / GRID_SIZE_X).round() * GRID_SIZE_X,
                    (transform.translation.y / GRID_SIZE_Y).round() * GRID_SIZE_Y,
                    transform.translation.z,
                );
                let mut rng = rand::thread_rng();
                let duration = rng.gen_range(500..1000);

                let tween = Tween::new(
                    EaseFunction::BounceOut,
                    Duration::from_millis(duration),
                    // Duration::from_millis(1000),
                    TransformPositionLens {
                        start: transform.translation,
                        end: new_pos,
                    },
                )
                .with_completed_event(0);

                commands.entity(entity).try_insert(Animator::new(tween));
            }
        }
    }
}

fn align_queued_requests(
    mut commands: Commands,
    q_servers: Query<(&Transform, &Server), Without<Request>>,
    mut q_request: Query<(&mut Transform, &Request, Option<&Animator<Transform>>), Without<Server>>,
) {
    let y_offset = 30.0;
    for (t_server, server) in q_servers.iter() {
        if server.queued_requests.len() > 0 {
            for (index, e_request) in server.queued_requests.clone().iter_mut().enumerate() {
                let (t_request, _request, animator) = q_request.get_mut(*e_request).unwrap();
                match animator {
                    Some(_) => {
                        // Already moving to position...
                    }
                    None => {
                        let new_x = t_server.translation.x + 64.0;
                        let new_y = t_server.translation.y + (index as f32 * y_offset);
                        let new_pos = Vec3::new(new_x, new_y, t_request.translation.z);
                        if t_request.translation != new_pos {
                            println!("Updating positions");
                            // t_request.translation = new_pos;
                            let mut rng = rand::thread_rng();
                            let duration = rng.gen_range(100..500);

                            let tween = Tween::new(
                                EaseFunction::BounceOut,
                                Duration::from_millis(duration),
                                // Duration::from_millis(1000),
                                TransformPositionLens {
                                    start: t_request.translation,
                                    end: new_pos,
                                },
                            )
                            .with_completed_event(0);

                            commands.entity(*e_request).try_insert(Animator::new(tween));
                        }
                    }
                }
            }
        }
    }
}

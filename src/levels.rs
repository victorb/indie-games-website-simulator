use crate::prelude::*;

fn strs(s: Vec<&str>) -> Vec<String> {
    let mut ret = Vec::new();
    for ss in s.iter() {
        ret.push(ss.to_string());
    }
    ret
}

fn levels() -> Vec<Level> {
    vec![
        // PASSABLE
        Level {
            title: "Alpha Test".to_string(),
            schedules: vec![LoadSchedule::new(5.0, 2, 5.0, (1..1).into())],
            intro_text: "For this first test, we don't really care about response times that much, but every single request has to be handled, don't drop any!".to_string(),
            failure_texts: strs(vec![
                "You couldn't even handle the alpha test? :(",
                "I thought you said you've done this before!",
                "If you can't handle this, I don't know...",
                "Erhm, maybe I know some other smart people",
            ]),
            success_text: "Woah, nice! Now we can finally move on to launching the website!".to_string(),
            available_servers: 1,
            upgrade_points: 5,
            required_handled_requests: 1.0,
            // required_handled_requests: 1.0,
            // This is actually 1000ms in the game, but 1.0 is one second actually
            required_avg_response_time: 10.0,
        },
        // PASSABLE
        Level {
            title: "Website Launch".to_string(),
            schedules: vec![LoadSchedule::new(10.0, 5, 10.0, (1..1).into())],
            intro_text: "Time to launch the website! Expect a lot more requests over a longer timeframe. I've gotten you some more servers too, don't forget you can change their mode to Proxy!".to_string(),
            success_text: "Wow, that went great! Only time can tell what will come next...".to_string(),
            failure_texts: strs(vec![
                "This was our only shot and you ruined it...",
                "Not sure how we're supposed to recover from this",
                "But the alpha test went well, and now this?",
                "Sometimes I think you're not even trying",
            ]),
            available_servers: 4,
            upgrade_points: 15,
            required_handled_requests: 0.8,
            required_avg_response_time: 10.0,
        },
        // NOT SURE IF PASSABLE ?!
        Level {
            title: "GMTK Game Jam".to_string(),
            schedules: vec![LoadSchedule::new(30.0, 20, 30.0, (1..1).into())],
            intro_text: "This crazy YouTube person has decided to use our platform for hosting their game jam! It's gonna be a ton of fun, but prepare for an astronomical load! I've given you access to extra hardware of course".to_string(),
            success_text: "Wow, that went great! Only time can tell what will come next...".to_string(),
            failure_texts: strs(vec![
                "We knew it would be difficult, but who knew this hard?",
                "Maybe we should just give up...",
                "Sometimes, the world is not on your side",
                "Where did you learn this stuff anyways?",
                "Not so easy now when you don't have a LLM huh?",
            ]),
            available_servers: 6,
            upgrade_points: 80,
            required_handled_requests: 0.7,
            required_avg_response_time: 20.0,
        },
    ]
}

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<LevelChange>()
            // Resets everything from the beginning
            .add_event::<ResetCurrentLevel>()
            // Restes but keeps upgrades and servers
            .add_event::<ReloadCurrentLevel>()
            .add_event::<LoadNextLevel>()
            .init_resource::<GameLevels>()
            .add_systems(
                Update,
                (
                    handle_level_change,
                    handle_reset.run_if(on_event::<ResetCurrentLevel>()),
                    handle_load_next_level.run_if(on_event::<LoadNextLevel>()),
                    // handle_reset.run_if(input_just_pressed(KeyCode::KeyR)),
                    handle_reload.run_if(input_just_pressed(KeyCode::KeyP)),
                    handle_reload.run_if(on_event::<ReloadCurrentLevel>()),
                )
                    .run_if(in_state(GameState::Planning).or_else(in_state(GameState::Results))),
            );
    }
}

#[derive(Event)]
pub struct LevelChange;

// Used to mark components that belong to a "scene" or level, gets removed when
// we reload level or load a new one
#[derive(Component)]
pub struct LevelOwned;

pub struct Level {
    pub title: String,
    pub schedules: Vec<LoadSchedule>,
    /// How many servers to give the player
    pub available_servers: usize,
    /// How many upgrade points to give the player
    pub upgrade_points: usize,
    pub intro_text: String,
    pub success_text: String,
    pub failure_texts: Vec<String>,
    // 0.0 <> 1.0 how many percent of requests had to be handled
    pub required_handled_requests: f32,
    // in seconds (ms in game), how low the avg response time needs to be
    required_avg_response_time: f32,
}

#[derive(Resource)]
pub struct GameLevels {
    // Index of current level
    pub current: usize,
    pub levels: Vec<Level>,
}

impl GameLevels {
    pub fn active_level(&self) -> &Level {
        &self.levels[self.current]
    }
}

impl Default for GameLevels {
    fn default() -> Self {
        let res = Self {
            current: 0,
            levels: levels(),
        };
        res
    }
}

/// Resets the current level from scratch
#[derive(Event)]
pub struct ResetCurrentLevel;

impl From<ListenerInput<Pointer<Click>>> for ResetCurrentLevel {
    fn from(_event: ListenerInput<Pointer<Click>>) -> Self {
        ResetCurrentLevel
    }
}

/// Reloads the current level (servers + upgrades stay the same)
#[derive(Event)]
pub struct ReloadCurrentLevel;

impl From<ListenerInput<Pointer<Click>>> for ReloadCurrentLevel {
    fn from(_event: ListenerInput<Pointer<Click>>) -> Self {
        ReloadCurrentLevel
    }
}

#[derive(Event)]
pub struct LoadNextLevel;

impl From<ListenerInput<Pointer<Click>>> for LoadNextLevel {
    fn from(_event: ListenerInput<Pointer<Click>>) -> Self {
        LoadNextLevel
    }
}

fn handle_load_next_level(
    mut game_levels: ResMut<GameLevels>,
    mut evs: EventWriter<ResetCurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match game_levels.levels.get(game_levels.current + 1) {
        Some(_) => {
            game_levels.current += 1;
        }
        None => {
            println!("Game Completed!");
            next_state.set(GameState::GameCompleted);
        }
    }
}

fn handle_reload(
    mut commands: Commands,
    q_level_owned: Query<Entity, With<LevelOwned>>,
    mut evs_spawn: EventWriter<AddNewServer>,
    game_levels: Res<GameLevels>,
    mut results: ResMut<LevelResults>,
    mut game_stats: ResMut<GameStats>,
    mut points: ResMut<UpgradePoints>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Reloading");
    // TODO introduce some sort of persistance?

    // This can be our "reload level" function
    // Despawn everything from previous levels
    for owned in &q_level_owned {
        commands.entity(owned).despawn_recursive();
    }
    // Create everything for our new level
    let active_level = game_levels.active_level();
    // TODO DONT add servers
    // println!("Spawning {} servers", active_level.available_servers);
    // evs_spawn.send(AddNewServer(active_level.available_servers));
    // TODO Giving points for upgrades
    // Adding the load scenarios
    let ls = LoadScenario {
        schedules: active_level.schedules.clone(),
    };
    commands.spawn((ls, LevelOwned));
    // Change the required pass percentage
    results.pass_percentage = active_level.required_handled_requests;
    // Change the required pass avg response time
    results.pass_avg_response_time = active_level.required_avg_response_time;
    // Reset our game stats
    *game_stats = GameStats::default();
    // TODO DONT Reset upgrade points
    // points.total = active_level.upgrade_points;
    // points.assigned = 0;
    // Force reset to Planning
    next_state.set(GameState::Planning);
}

fn handle_reset(
    mut commands: Commands,
    q_level_owned: Query<Entity, With<LevelOwned>>,
    q_servers: Query<Entity, With<Server>>,
    mut evs_spawn: EventWriter<AddNewServer>,
    game_levels: Res<GameLevels>,
    mut results: ResMut<LevelResults>,
    mut game_stats: ResMut<GameStats>,
    mut points: ResMut<UpgradePoints>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Resetting");
    // This can be our "reload level" function
    // Despawn everything from previous levels
    for owned in &q_level_owned {
        commands.entity(owned).despawn_recursive();
    }
    // Despawn all previous servers
    for server in q_servers.iter() {
        commands.entity(server).despawn_recursive();
    }
    // Create everything for our new level
    let active_level = game_levels.active_level();
    // Adding servers
    println!("Spawning {} servers", active_level.available_servers);
    evs_spawn.send(AddNewServer(active_level.available_servers));
    // TODO Giving points for upgrades
    // Adding the load scenarios
    let ls = LoadScenario {
        schedules: active_level.schedules.clone(),
    };
    commands.spawn((ls, LevelOwned));
    // Change the required pass percentage
    results.pass_percentage = active_level.required_handled_requests;
    // Change the required pass avg response time
    results.pass_avg_response_time = active_level.required_avg_response_time;
    // Reset our game stats
    *game_stats = GameStats::default();
    // Reset upgrade points
    points.total = active_level.upgrade_points;
    points.assigned = 0;
    // Force reset to Planning
    next_state.set(GameState::Planning);
}

fn handle_level_change(
    mut once_load: Local<bool>,
    mut current_level: Local<usize>,
    game_levels: ResMut<GameLevels>,
    mut evs: EventWriter<ResetCurrentLevel>,
) {
    if *current_level != game_levels.current || !*once_load {
        // Resource value has changed, update our local first
        *current_level = game_levels.current;
        evs.send(ResetCurrentLevel);
    }
    if !*once_load {
        *once_load = true;
    }
}

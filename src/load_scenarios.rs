use crate::prelude::*;

pub struct LoadScenariosPlugin;

impl Plugin for LoadScenariosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_load_scenarios.run_if(input_just_pressed(KeyCode::Space)),
                start_load_scenarios.run_if(on_event::<StartLoadScenarios>()),
            ), // .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            FixedUpdate,
            (
                // start_load_scenarios.run_if(on_timer(Duration::from_millis(1000))),
                spawn_requests_based_on_load_scenario.run_if(on_timer(Duration::from_millis(100))),
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_event::<StartLoadScenarios>();
    }
}

#[derive(Event)]
pub struct StartLoadScenarios;

impl From<ListenerInput<Pointer<Click>>> for StartLoadScenarios {
    fn from(_event: ListenerInput<Pointer<Click>>) -> Self {
        StartLoadScenarios
    }
}

#[derive(Clone)]
// Defines a load of requests that happens
pub struct LoadSchedule {
    pub active: bool,
    pub completed: bool,
    pub start_time: f32,
    pub rampup: Duration,
    pub max_rps: usize,
    pub rampdown: Duration,
    pub request_sizes: Range<usize>,
    // Used internally for RPS calculation
    accumulated_requests: f32,
}

impl LoadSchedule {
    pub fn new(rampup: f32, max_rps: usize, rampdown: f32, sizes: Range<usize>) -> Self {
        Self {
            active: false,
            completed: false,
            start_time: 0.0,
            rampup: Duration::from_secs_f32(rampup),
            max_rps,
            rampdown: Duration::from_secs_f32(rampdown),
            request_sizes: sizes,
            accumulated_requests: 0.0,
        }
    }
    pub fn start(&mut self, current_elapsed_time: f32) {
        self.active = true;
        self.start_time = current_elapsed_time;
    }
}

#[derive(Component)]
pub struct LoadScenario {
    pub schedules: Vec<LoadSchedule>,
}

// TODO this became a bit of a god system for some reason
pub fn spawn_requests_based_on_load_scenario(
    time: Res<Time>,
    mut query: Query<(Entity, &mut LoadScenario)>,
    mut commands: Commands,
    mut requests_gone: Local<bool>,
    q_requests: Query<&Request>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, mut scenario) in query.iter_mut() {
        let mut all_schedules_completed = true;
        for schedule in &mut scenario.schedules {
            if !schedule.completed {
                all_schedules_completed = false;
            }
            if schedule.active && !schedule.completed {
                let elapsed = time.elapsed_seconds() - schedule.start_time;
                let total_duration =
                    schedule.rampup.as_secs_f32() + schedule.rampdown.as_secs_f32();

                if elapsed > total_duration {
                    schedule.completed = true;
                    continue;
                }

                let current_rps = calculate_current_rps(elapsed, schedule);
                schedule.accumulated_requests += current_rps / 10.0;
                let requests_to_spawn = schedule.accumulated_requests.floor() as usize;
                schedule.accumulated_requests -= requests_to_spawn as f32;

                // println!(
                //     "Spawning {} requests at elapsed time {} (rps: {} [divided: {}, rounded: {}])",
                //     requests_to_spawn,
                //     elapsed,
                //     current_rps,
                //     (current_rps / 10.0),
                //     (current_rps / 10.0).round()
                // );

                for _ in 0..requests_to_spawn {
                    commands.add(SpawnRequest {});
                }
            }
        }
        if all_schedules_completed {
            if *requests_gone {
                println!("All schedules done, and no more requests!");
                // send event that we're done!
                commands.entity(entity).despawn_recursive();
                // Show results
                next_state.set(GameState::Results);
            } else {
                let len = q_requests.iter().len();
                if len == 0 {
                    *requests_gone = true;
                }
            }
        } else {
            *requests_gone = false;
        }
    }
}

fn calculate_current_rps(elapsed: f32, schedule: &LoadSchedule) -> f32 {
    let rampup = schedule.rampup.as_secs_f32();
    let rampdown = schedule.rampdown.as_secs_f32();
    let max_rps = schedule.max_rps as f32;

    if elapsed < rampup {
        (elapsed / rampup) * max_rps
    } else if elapsed < rampup + rampdown {
        let rampdown_progress = (elapsed - rampup) / rampdown;
        max_rps * (1.0 - rampdown_progress)
    } else {
        0.0
    }
}

pub fn start_load_scenarios(
    time: Res<Time>,
    mut query: Query<&mut LoadScenario>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Move into GameState::Running as well!
    next_state.set(GameState::Running);

    for mut scenario in query.iter_mut() {
        for schedule in &mut scenario.schedules {
            if !schedule.active && !schedule.completed {
                println!("Starting schedule");
                schedule.start(time.elapsed_seconds());
            }
        }
    }
}

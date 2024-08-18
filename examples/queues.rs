use indie_games_website_simulator::{full_game::FullGamePlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins(FullGamePlugin)
        .add_systems(Update, start_game)
        .run();
}

fn start_game(
    time: Res<Time>,
    mut evs: EventWriter<StartLoadScenarios>,
    mut q_servers: Query<&mut Server>,
) {
    if time.elapsed_seconds() > 0.5 {
        evs.send(StartLoadScenarios);

        for mut server in q_servers.iter_mut() {
            server.queue_size = 5;
        }
    }
}

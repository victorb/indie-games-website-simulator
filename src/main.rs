#![feature(new_range_api)]

use indie_games_website_simulator::{full_game::FullGamePlugin, prelude::*};

fn main() {
    App::new().add_plugins(FullGamePlugin).run();
}

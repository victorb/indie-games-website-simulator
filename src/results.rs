use crate::prelude::*;

pub struct ResultsPlugin;

impl Plugin for ResultsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelResults>()
            .add_systems(OnEnter(GameState::Results), calculate_pass_or_not);
    }
}

#[derive(Resource, Default)]
pub struct LevelResults {
    //
    pub pass_percentage: f32,
    pub current_percentage: f32,
    //
    pub pass_avg_response_time: f32,
    pub current_avg_response_time: f32,
    //
    pub passed: bool,
}

fn calculate_pass_or_not(game_stats: Res<GameStats>, mut level_results: ResMut<LevelResults>) {
    let handled = game_stats.handled_requests;
    let dropped = game_stats.dropped_requests;
    let total_requests = handled + dropped;
    level_results.current_percentage = handled as f32 / total_requests as f32;

    let passed_handled_percentage =
        level_results.current_percentage >= level_results.pass_percentage;

    let passed_avg_response_times =
        game_stats.avg_response_time <= level_results.pass_avg_response_time;

    level_results.passed = passed_handled_percentage && passed_avg_response_times;

    println!(
        "Avg resposne times: achieved/required {:.2}/{:.2}",
        game_stats.avg_response_time, level_results.pass_avg_response_time
    );

    if level_results.passed {
        println!(
            "Level passed! Current percentage: {:.2}%, Required: {:.2}%",
            level_results.current_percentage * 100.0,
            level_results.pass_percentage * 100.0
        );
    } else {
        println!(
            "Level failed. Current percentage: {:.2}%, Required: {:.2}%",
            level_results.current_percentage * 100.0,
            level_results.pass_percentage * 100.0
        );
    }
}

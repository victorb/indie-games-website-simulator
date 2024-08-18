use crate::prelude::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<EditMode>()
            .enable_state_scoped_entities::<EditMode>()
            .add_systems(OnEnter(GameState::Loading), on_enter_loading)
            .add_systems(OnEnter(GameState::Intro), on_enter_intro)
            .add_systems(OnEnter(GameState::LevelSelect), on_enter_level_select)
            .add_systems(OnEnter(GameState::Planning), on_enter_planning);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Intro,
    LevelSelect,
    Planning,
    Running,
    Results,
    GameCompleted,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Planning)]
pub enum EditMode {
    #[default]
    Upgrade,
    Outputs,
}

//

fn on_enter_loading(mut next_state: ResMut<NextState<GameState>>) {
    println!("Entered Loading, next Into, should happen automatically");
    // next_state.set(GameState::Intro);
}

fn on_enter_intro(mut next_state: ResMut<NextState<GameState>>) {
    println!("Entered Intro, next LevelSelect");
    // next_state.set(GameState::LevelSelect);
}

fn on_enter_level_select(mut next_state: ResMut<NextState<GameState>>) {
    // Show selection of levels
    // for level in ALL_LEVELS.iter() {}
    // draw_level_selection_ui();
    println!("Entered LevelSelect, next Planning");
    // next_state.set(GameState::Planning);
}

fn on_enter_planning(/*mut next_state: ResMut<NextState<GameState>>*/) {
    println!("Now in planning mode");
    // next_state.set(GameState::Intro);
    // Stop here for now
}

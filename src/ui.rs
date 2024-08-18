use crate::prelude::*;
use bevy::color::palettes::{
    css::{BLACK, WHITE_SMOKE},
    tailwind::*,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    show_hide_selection_ui.run_if(in_state(EditMode::Upgrade)),
                    update_ui.run_if(in_state(GameState::Running)),
                    update_planning_ui.run_if(in_state(EditMode::Upgrade)),
                    button_interactivity
                ),
            )
            // Show/hide selection UI

            .add_systems(OnEnter(GameState::Running), spawn_running_ui)
            // .add_systems(OnEnter(GameState::Planning), spawn_planning_ui)
            .add_systems(OnEnter(GameState::Results), spawn_results_ui)
            .add_systems(OnEnter(GameState::GameCompleted), spawn_completed_ui)

            // EditMode States
            .add_systems(OnEnter(EditMode::Upgrade), spawn_planning_ui)
            .add_systems(OnExit(EditMode::Upgrade), clear_entity_with::<PlanningUI>)
            //
            .add_systems(OnEnter(EditMode::Outputs), spawn_outputs_ui)
            .add_systems(OnExit(EditMode::Outputs), clear_entity_with::<OutputsUI>)
            // 
            .add_systems(OnExit(GameState::Running), clear_entity_with::<RunningUI>)
            .add_systems(OnExit(GameState::Results), clear_entity_with::<ResultsUI>)

            // .add_systems(Startup, spawn_results_ui)
        // app.add_event::<UpgradeServerCPUEvent>()
        //     .add_event::<AddNewServer>()
        // .add_systems(
        //     Update,
        //     (
        //         handle_upgrade_server_cpu,
        //         draw_children_ui,
        //         add_new_server.run_if(on_event::<AddNewServer>()),
        //         align_servers_to_grid,
        //     ),
        // )
        //     .add_systems(FixedUpdate, process_requests);
        ;
    }
}

fn clear_entity_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    // Delete all the planning UI
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct PlanningUI;

#[derive(Component)]
pub struct RunningUI;

#[derive(Component)]
pub struct ResultsUI;

#[derive(Component)]
pub struct CompletedUI;

#[derive(Component)]
pub struct OutputsUI;

create_markers!(
    CurrentLevelTitleText,
    SelectedLabel,
    SetFilterOutputButton,
    SetOutputsButton,
    SwitchModeButton,
    ResetUpgradesButton,
    UpgradeQueueSizeButton,
    ServerSelectionUI,
    UpgradeCPUButton,
    DroppedRequestsText,
    AverageResponseTimeText,
    RemainingPointsText,
    HandledRequestsText,
    LoadRPSText,
    StartButton,
    ResetButton,
    // Results UI
    ResultsVerdictText,
    ResultsRetryButton,
    ResultsNextButton,
    ResultsPercentageText,
    ResultsPercentageRequirementText
);

fn spawn_text<UI, T>(
    ui_component: UI,
    font_assets: &Res<FontAssets>,
    commands: &mut Commands,
    label: &str,
    offset: usize,
    component: T,
) where
    UI: Component,
    T: Component,
{
    let vertical_spacing = 40.0;
    // let font_handle = asset_server.load("fonts/MajorMonoDisplay-Regular.ttf");
    commands.spawn((
        ui_component,
        TextBundle::from_sections([
            TextSection::new(
                format!("{}: ", label),
                TextStyle {
                    font_size: 30.0,
                    font: font_assets.texts.clone(),
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_assets.texts.clone(),
                font_size: 30.0,
                color: Color::srgba(0.9, 0.9, 1.0, 0.6).into(),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(vertical_spacing * offset as f32),
            ..default()
        }),
        component,
    ));
}

fn spawn_button<C, E>(
    commands: &mut Commands,
    label: &str,
    offset: usize,
    component: C,
    bg_color: Srgba,
) where
    C: Bundle,
    E: Event + From<ListenerInput<Pointer<Click>>>,
{
    let vertical_spacing = 55.0;
    // let font_handle = asset_server.load("fonts/MajorMonoDisplay-Regular.ttf");
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    top: Val::Px((vertical_spacing * offset as f32) + 10.0),
                    width: Val::Px(350.0),
                    height: Val::Px(50.0),
                    right: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    // bottom: Val::Px(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: bg_color.into(),
                ..default()
            },
            component,
            PickableBundle::default(),
            On::<Pointer<Click>>::send_event::<E>(),
            PlanningUI,
            // NoDeselect,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    label,
                    TextStyle {
                        font_size: 22.0,
                        // font: font_handle,
                        color: BLACK.into(),
                        ..default()
                    },
                ),
                Pickable::IGNORE,
            ));
        });
}

fn spawn_child_button<T, U>(
    builder: &mut ChildBuilder,
    label: &str,
    component: U,
    text_color: Srgba,
    bg_color: Srgba,
) where
    U: Bundle,
    T: Event + From<ListenerInput<Pointer<Click>>>,
{
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::axes(Val::Px(0.0), Val::Px(15.0)),
                    width: Val::Px(250.0),
                    height: Val::Px(50.0),
                    padding: UiRect::axes(Val::Px(15.0), Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: bg_color.into(),
                ..default()
            },
            component,
            PickableBundle::default(),
            On::<Pointer<Click>>::send_event::<T>(),
            NoDeselect,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    label,
                    TextStyle {
                        font_size: 22.0,
                        color: text_color.into(),
                        ..default()
                    },
                ),
                Pickable::IGNORE,
            ));
        });
}

fn make_connections(
    mut next_state: ResMut<NextState<EditMode>>,
    // This contains the source server
    selected: Res<SelectedServerForOutputs>,
    mut q_server: Query<(&mut Server, &mut Handle<Image>)>,
    // These are the destination servers
    q_selection: Query<(Entity, &PickSelection), With<Server>>,
    image_assets: Res<ImageAssets>,
) {
    // Figure out what server we initially selected
    match selected.0 {
        Some(current_server) => {
            let (mut server, mut handle) = q_server.get_mut(current_server).unwrap();
            // Make sure selected server is Proxy
            // TODO duplicated logic
            match server.mode {
                ServerMode::Process => {
                    *handle = image_assets.server_proxy.clone();
                    server.mode = ServerMode::Proxy;
                }
                ServerMode::Proxy => {
                    // It's already a proxy
                }
            };

            // Reset outputs
            server.outputs = vec![];
            // Get all currently selected servers
            // Make the neccessary connections
            for (selected_entity, selection) in q_selection.iter() {
                if selection.is_selected {
                    server.outputs.push(selected_entity);
                }
            }
            // Set next state to Upgrade
            next_state.set(EditMode::Upgrade);
        }
        None => {
            // No server selected, do nothing? At least reset to Upgrade mode
            next_state.set(EditMode::Upgrade);
        }
    }
}

pub fn spawn_outputs_ui(mut commands: Commands) {
    commands
        .spawn((
            OutputsUI,
            NodeBundle {
                style: Style {
                    top: Val::Px(150.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    // bottom: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.15, 0.15, 0.15, 0.8).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Select servers to connect",
                    TextStyle {
                        font_size: 22.0,
                        // font: asset_server.load("fonts/MajorMonoDisplay-Regular.ttf"),
                        color: Color::srgba(0.9, 0.9, 0.9, 1.0),
                        ..default()
                    },
                ),
                Pickable::IGNORE,
            ));
        });

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::axes(Val::Px(30.0), Val::Px(30.0)),
                    width: Val::Px(250.0),
                    height: Val::Px(50.0),
                    padding: UiRect::axes(Val::Px(15.0), Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: GREEN_800.into(),
                ..default()
            },
            OutputsUI,
            Name::new("FinishButton"),
            PickableBundle::default(),
            On::<Pointer<Click>>::run(make_connections),
            // On::<Pointer<Click>>::send_event::<T>(),
            NoDeselect,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Finish",
                    TextStyle {
                        font_size: 22.0,
                        // font: font_handle,
                        color: WHITE_SMOKE.into(),
                        ..default()
                    },
                ),
                Pickable::IGNORE,
            ));
        });
}

pub fn spawn_completed_ui(
    mut commands: Commands,
    level_results: Res<LevelResults>,
    game_stats: Res<GameStats>,
) {
    commands
        .spawn((
            CompletedUI,
            NodeBundle {
                style: Style {
                    // top: Val::Px((vertical_spacing * offset as f32) + 10.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    // bottom: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(100.0),
                    ..default()
                },
                background_color: Color::srgba(0., 0., 0., 0.5).into(),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Woah, you really made it! Congrats!",
                    TextStyle {
                        font_size: 48.0,
                        color: BLUE_400.into(),
                        ..default()
                    },
                ),
                // ResultsVerdictText,
                Pickable::IGNORE,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Thank you for playing Indie Games Website Simulator",
                    TextStyle {
                        font_size: 24.0,
                        color: BLUE_400.into(),
                        ..default()
                    },
                ),
                // ResultsVerdictText,
                Pickable::IGNORE,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Made by Victor Bjelkholm for GMTK Game Jam 2024",
                    TextStyle {
                        font_size: 16.0,
                        color: BLUE_400.into(),
                        ..default()
                    },
                ),
                // ResultsVerdictText,
                Pickable::IGNORE,
            ));
        });
    commands
        .spawn((
            CompletedUI,
            NodeBundle {
                style: Style {
                    bottom: Val::Px(10.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "(I didn't knew you could actually win that last level...)",
                    TextStyle {
                        font_size: 16.0,
                        color: BLUE_400.into(),
                        ..default()
                    },
                ),
                // ResultsVerdictText,
                Pickable::IGNORE,
            ));
        });
}

pub fn spawn_results_ui(
    mut commands: Commands,
    level_results: Res<LevelResults>,
    game_stats: Res<GameStats>,
    game_levels: Res<GameLevels>,
    font_assets: Res<FontAssets>,
) {
    let font_handle = &font_assets.texts;
    let (verdict_text, verdict_color) = if level_results.passed {
        ("Success!", GREEN_700)
    } else {
        ("Failure!", RED_700)
    };

    let message = if level_results.passed {
        game_levels.active_level().success_text.clone()
    } else {
        let mut rng = rand::thread_rng();
        game_levels
            .active_level()
            .failure_texts
            .choose(&mut rng)
            .unwrap()
            .clone()
    };
    let message = format!("\n{}", message);

    let total_requests = game_stats.handled_requests + game_stats.dropped_requests;
    let personal_result = format!(
        "\nYou handled {:.0}% of the requests ({}/{})",
        level_results.current_percentage * 100.0,
        game_stats.handled_requests,
        total_requests
    );

    let requirement = if level_results.pass_percentage == 1.0 {
        format!("\nYou were required to handle 100% of the requests")
    } else {
        format!(
            "\nYou were required to handle at least {:.0}% of the requests",
            level_results.pass_percentage * 100.0,
        )
    };

    let word = if level_results.passed { "was" } else { "is" };

    let personal_avg_latency = format!(
        "\nYou managed a average response time of {:.2}ms",
        game_stats.avg_response_time * 10.0
    );
    let avg_requirement = format!(
        "\nMinimum acceptable latency {} {:.2}ms",
        word,
        level_results.pass_avg_response_time * 10.0
    );

    commands
        .spawn((
            ResultsUI,
            NodeBundle {
                style: Style {
                    // top: Val::Px((vertical_spacing * offset as f32) + 10.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    // bottom: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                // background_color: Color::srgba(0., 0., 0., 0.8).into(),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // top: Val::Px((vertical_spacing * offset as f32) + 10.0),
                            width: Val::Percent(70.0),
                            height: Val::Percent(80.0),
                            // bottom: Val::Px(20.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgba(0.1, 0.1, 0.1, 0.99).into(),
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            verdict_text,
                            TextStyle {
                                font_size: 48.0,
                                font: font_handle.clone(),
                                color: verdict_color.into(),
                                ..default()
                            },
                        ),
                        ResultsVerdictText,
                        Pickable::IGNORE,
                    ));
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            Pickable::IGNORE,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    message,
                                    TextStyle {
                                        font_size: 24.0,
                                        font: font_handle.clone(),
                                        color: WHITE_SMOKE.into(),
                                        ..default()
                                    },
                                ),
                                ResultsVerdictText,
                                Pickable::IGNORE,
                            ));
                        });
                    parent.spawn((
                        TextBundle::from_section(
                            personal_result,
                            TextStyle {
                                font_size: 30.0,
                                font: font_handle.clone(),
                                color: WHITE_SMOKE.into(),
                                ..default()
                            },
                        ),
                        ResultsVerdictText,
                        Pickable::IGNORE,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            requirement,
                            TextStyle {
                                font_size: 28.0,
                                font: font_handle.clone(),
                                color: WHITE_SMOKE.into(),
                                ..default()
                            },
                        ),
                        ResultsVerdictText,
                        Pickable::IGNORE,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            personal_avg_latency,
                            TextStyle {
                                font_size: 18.0,
                                font: font_handle.clone(),
                                color: WHITE_SMOKE.into(),
                                ..default()
                            },
                        ),
                        Pickable::IGNORE,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            avg_requirement,
                            TextStyle {
                                font_size: 18.0,
                                font: font_handle.clone(),
                                color: WHITE_SMOKE.into(),
                                ..default()
                            },
                        ),
                        Pickable::IGNORE,
                    ));
                    if level_results.passed {
                        spawn_child_button::<LoadNextLevel, ResultsNextButton>(
                            parent,
                            "Next",
                            ResultsNextButton,
                            WHITE_SMOKE,
                            GREEN_900,
                        );
                    }
                    spawn_child_button::<ReloadCurrentLevel, ResultsRetryButton>(
                        parent,
                        "Retry",
                        ResultsRetryButton,
                        WHITE_SMOKE,
                        if level_results.passed {
                            ORANGE_800
                        } else {
                            GREEN_400
                        },
                    );
                });
        });
}

pub fn spawn_running_ui(mut commands: Commands, asset_server: Res<FontAssets>) {
    println!("Spawning running UI");
    spawn_text(
        RunningUI,
        &asset_server,
        &mut commands,
        "Average Response Time",
        1,
        AverageResponseTimeText,
    );
    spawn_text(
        RunningUI,
        &asset_server,
        &mut commands,
        "Dropped Requests",
        2,
        DroppedRequestsText,
    );
    spawn_text(
        RunningUI,
        &asset_server,
        &mut commands,
        "Handled Requests",
        3,
        HandledRequestsText,
    );
}

pub fn spawn_planning_ui(
    mut commands: Commands,
    game_levels: Res<GameLevels>,
    font_assets: Res<FontAssets>,
) {
    println!("Spawning planning UI");

    let title = game_levels.active_level().title.clone();

    commands.spawn((
        PlanningUI,
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!("Level #{}: {}", game_levels.current + 1, title),
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: font_assets.titles.clone(),
                font_size: 48.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        CurrentLevelTitleText,
    ));
    commands.spawn((
        PlanningUI,
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!("{}", game_levels.active_level().intro_text),
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                // font: font_assets.texts.clone(),
                font_size: 18.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(10.0),
            width: Val::Px(320.0),
            ..default()
        }),
        CurrentLevelTitleText,
    ));

    spawn_text(
        PlanningUI,
        &font_assets,
        &mut commands,
        "Available Upgrade Points",
        6,
        RemainingPointsText,
    );

    spawn_button::<StartButton, StartLoadScenarios>(
        &mut commands,
        "Start",
        0,
        StartButton,
        GREEN_400,
    );
    spawn_button::<ResetButton, ResetCurrentLevel>(
        &mut commands,
        "Reset",
        1,
        ResetButton,
        ORANGE_400,
    );

    // Selection UI
    commands
        .spawn((
            PlanningUI,
            NodeBundle {
                style: Style {
                    // top: Val::Px((vertical_spacing * offset as f32) + 10.0),
                    bottom: Val::Px(10.0),
                    width: Val::Px(350.0),
                    height: Val::Px(330.0),
                    right: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    // bottom: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                background_color: Color::srgba(0.15, 0.15, 0.15, 0.9).into(),
                ..default()
            },
            ServerSelectionUI,
            NoDeselect,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Server",
                    TextStyle {
                        font_size: 22.0,
                        // font: asset_server.load("fonts/MajorMonoDisplay-Regular.ttf"),
                        color: Color::srgba(0.9, 0.9, 0.9, 1.0),
                        ..default()
                    },
                ),
                SelectedLabel,
                Pickable::IGNORE,
            ));
            spawn_child_button::<ChangeServerModeEvent, SwitchModeButton>(
                parent,
                "Change Mode",
                SwitchModeButton,
                BLACK,
                GREEN_400,
            );
            // Only render this if we're a proxy
            spawn_child_button::<SetServerOutputEvent, SetOutputsButton>(
                parent,
                "Set Outputs",
                SetOutputsButton,
                BLACK,
                BLUE_400,
            );
            spawn_child_button::<UpgradeServerCPUEvent, UpgradeCPUButton>(
                parent,
                "Upgrade CPU",
                UpgradeCPUButton,
                BLACK,
                GREEN_400,
            );
            spawn_child_button::<UpgradeServerQueueEvent, UpgradeQueueSizeButton>(
                parent,
                "Upgrade Queue Size",
                UpgradeQueueSizeButton,
                BLACK,
                GREEN_400,
            );
            spawn_child_button::<ResetUpgradesEvent, ResetUpgradesButton>(
                parent,
                "Reset Upgrades",
                ResetUpgradesButton,
                WHITE_SMOKE,
                RED_900,
            );
        });
}

pub fn update_planning_ui(
    points: Res<UpgradePoints>,
    mut texts: ParamSet<(Query<&mut Text, With<RemainingPointsText>>,)>,
) {
    texts.p0().get_single_mut().unwrap().sections[1].value =
        (points.total - points.assigned).to_string();
}

pub fn update_ui(
    stats: Res<GameStats>,
    mut texts: ParamSet<(
        Query<&mut Text, With<AverageResponseTimeText>>,
        Query<&mut Text, With<DroppedRequestsText>>,
        Query<&mut Text, With<HandledRequestsText>>,
    )>,
) {
    if stats.avg_response_time == 0.0 {
        texts.p0().get_single_mut().unwrap().sections[1].value = "n/a".to_string();
    } else {
        texts.p0().get_single_mut().unwrap().sections[1].value =
            format!("{:.2} ms", stats.avg_response_time * 10.0);
    }
    texts.p1().get_single_mut().unwrap().sections[1].value = stats.dropped_requests.to_string();
    texts.p2().get_single_mut().unwrap().sections[1].value = stats.handled_requests.to_string();
}

// Depending on the state in Selection, show/hide the UI related to it
pub fn show_hide_selection_ui(
    selection: Query<&PickSelection>,
    mut query: Query<(&ServerSelectionUI, &mut Visibility)>,
    mut previous: Local<Visibility>,
) {
    let selected: Vec<&PickSelection> = selection.iter().filter(|l| l.is_selected).collect();

    if selected.len() > 0 {
        if *previous != Visibility::Visible {
            *query.get_single_mut().unwrap().1 = Visibility::Visible;
            *previous = Visibility::Visible
        }
    } else {
        if *previous != Visibility::Hidden {
            *query.get_single_mut().unwrap().1 = Visibility::Hidden;
            *previous = Visibility::Hidden
        }
    }
}

// pub fn update_server_selection_ui(
//     selection: Res<Selection>,
//     mut q_server_selection: Query<(&ServerSelectionUI, &mut Visibility)>,
//     mut q_text: Query<&mut Text, With<SelectedLabel>>,
//     mut last_entity: Local<Option<Entity>>,
// ) {
//     if let Some(selected) = selection.selected {
//         if *last_entity != Some(selected) {
//             *last_entity = Some(selected);
//             println!("Update selected element");
//             q_text.get_single_mut().unwrap().sections[0].value = format!("Server: {}", selected);
//         }
//     } else {
//         // Do nothing
//     }
// }

/// Default, Hover, Pressed
#[derive(Component)]
pub struct Hoverable(pub Srgba, pub Srgba, pub Srgba);

fn button_interactivity(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Transform,
            &Hoverable,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut evs_sound: EventWriter<PlaySound>,
) {
    for (interaction, mut color, mut border_color, mut transform, hoverable, children) in
        &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => {
                println!("Doing nothing with button?");
                // text.sections[0].value = "Button".to_string();
                // *color = NORMAL_BUTTON.into();
                *color = hoverable.0.into();
            }
            Interaction::Hovered => {
                println!("Hovering Button");
                // text.sections[0].value = "Hover".to_string();
                // *color = HOVERED_BUTTON.into();
                *color = hoverable.1.into();
            }
            Interaction::Pressed => {
                println!("Pressing button");
                // text.sections[0].value = "Press".to_string();
                // *color = PRESSED_BUTTON.into();
                *color = hoverable.1.into();
                evs_sound.send(PlaySound(Sound::ClickButton));
            }
        }
    }
}

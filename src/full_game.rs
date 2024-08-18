use crate::prelude::*;
use bevy::color::palettes::tailwind::*;
use bevy_framepace::FramepacePlugin;

const SPRITE_OFFSET: f32 = 48.0;

pub struct FullGamePlugin;

impl Plugin for FullGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#game-canvas".into()),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgba(0.263, 0.541, 0.984, 1.0)))
        .add_plugins(FramepacePlugin)
        // Game plugins
        .add_plugins((
            ServerPlugin,
            RequestsPlugin,
            LoadScenariosPlugin,
            MiscPlugin,
            DraggingPlugin,
            SelectionPlugin,
            UIPlugin,
            LevelsPlugin,
            StatesPlugin,
            ResultsPlugin,
            AssetsPlugin,
            SplashPlugin,
            LevelSelectPlugin,
        ))
        .insert_resource(GameStats::default())
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                draw_gizmos
                    .run_if(in_state(GameState::Planning).or_else(in_state(GameState::Running))),
                draw_selected,
            ),
        );
    }
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    q_servers: Query<(Entity, &Server)>,
    q_transform: Query<&Transform>,
) {
    gizmos
        .grid_2d(
            Vec2::ZERO + Vec2::new(SPRITE_OFFSET, 0.0),
            0.0,
            UVec2::new(16, 9),
            Vec2::new(100., 100.),
            // Dark gray
            Srgba::new(0.9, 0.9, 0.9, 0.1),
        )
        .outer_edges();

    // Draw connections for server outputs
    for (e_server, server) in q_servers.iter() {
        let t_server = q_transform.get(e_server).unwrap();
        for output in &server.outputs {
            let t_output = q_transform.get(*output).unwrap();

            gizmos.line_2d(
                t_server.translation.truncate(),
                t_output.translation.truncate(),
                GREEN_200,
            );
        }
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

use crate::prelude::*;

use std::time::Duration;

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, Delay, EaseFunction, Tween,
};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_birds.run_if(in_state(GameState::Intro)))
            .add_systems(
                Update,
                (progress_to_level_select.run_if(on_timer(Duration::from_millis(3000))))
                    .run_if(in_state(GameState::Intro)),
            )
            .add_systems(OnEnter(GameState::Intro), setup)
            .add_systems(OnExit(GameState::Intro), despawn_splash_items);
    }
}

#[derive(Component)]
struct SplashItem;

#[derive(Component)]
struct Bird {
    target_scale: Vec3,
}

fn handle_birds(time: Res<Time>, mut bird_query: Query<(&mut Transform, &mut Bird)>) {
    for (mut transform, bird) in bird_query.iter_mut() {
        transform.scale = transform
            .scale
            .lerp(bird.target_scale, 2.0 * time.delta_seconds());
    }
}

fn progress_to_level_select(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::LevelSelect)
}

fn despawn_splash_items(mut commands: Commands, query: Query<Entity, With<SplashItem>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    let bird_tween = Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_secs(1),
        TransformRotationLens {
            start: Quat::from_axis_angle(Vec3::Z, 180_f32.to_radians()),
            end: Quat::from_axis_angle(Vec3::Z, 0_f32.to_radians()),
        },
    );
    let bird_move_tween = Tween::new(
        EaseFunction::CubicInOut,
        std::time::Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3::new(0., 0., 0.),
            end: Vec3::new(window.width() / 8., 0., 0.),
        },
    );

    let text_tween = Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3::new(window.width(), 0., -2.),
            end: Vec3::new(-window.width() / 8., 0., -2.),
        },
    );

    let background_tween = Tween::new(
        EaseFunction::CubicInOut,
        std::time::Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3::new(window.width() / 4., 0., -1.),
            end: Vec3::new(window.width() / 2., 0., -1.),
        },
    );

    commands.spawn((
        SplashItem,
        SpriteBundle {
            texture: asset_server.load("bevy.png"),
            transform: Transform {
                scale: Vec3::ZERO * 0.7,
                ..default()
            },
            sprite: Sprite { ..default() },
            ..default()
        },
        Bird {
            target_scale: Vec3::ONE * 0.6,
        },
        Animator::new(bird_tween.then(bird_move_tween)),
    ));

    commands.spawn((
        SplashItem,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(window.width() / 2., window.height()))),
            material: materials.add(Color::srgba(0.263, 0.541, 0.984, 1.0)),
            //material: materials.add(Color::rgb_linear(1.0, 0.025, 0.028)),
            transform: Transform {
                translation: Vec3::new(window.width() / 4., 0.0, -1.0),
                ..default()
            },
            ..default()
        },
        Animator::new(Delay::new(Duration::from_secs_f32(1.)).then(background_tween)),
    ));

    commands.spawn((
        SplashItem,
        Text2dBundle {
            text: Text::from_section(
                "Made with Bevy",
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 80.,
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3 {
                    x: window.width(),
                    y: 0.,
                    z: -2.,
                },
                ..default()
            },
            ..default()
        },
        Animator::new(Delay::new(Duration::from_secs_f32(0.75)).then(text_tween)),
    ));
}

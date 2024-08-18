use crate::prelude::*;
use bevy::color::palettes::{css::WHITE_SMOKE, tailwind::*};
use bevy_tweening::Delay;

pub struct LevelSelectPlugin;

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LevelSelect), spawn_ui)
            //.add_systems(
            //    Update,
            //    handle_tween_complete.run_if(in_state(GameState::LevelSelect)),
            //)
            ;
    }
}

// fn handle_tween_complete(
//     mut evs: EventReader<TweenCompleted>,
//     mut commands: Commands,
//     query: Query<Entity, With<Animator<Transform>>>,
// ) {
//     for ev in evs.read() {}
// }

#[derive(Component)]
struct LevelSelectStuff;

fn start_game(
    mut commands: Commands,
    query: Query<Entity, With<LevelSelectStuff>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Starting game");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
    next_state.set(GameState::Planning);
}

fn spawn_ui(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let logo_start = Vec3::new(0.0, 1000.0, 100.0);
    let logo_in_tween = Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_secs(1),
        TransformPositionLens {
            start: logo_start,
            end: Vec3::ZERO,
        },
    );
    let logo_out_tween = Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_secs(2),
        TransformPositionLens {
            start: Vec3::ZERO,
            end: Vec3::new(0.0, 250.0, 10.0),
        },
    );
    commands.spawn((
        LevelSelectStuff,
        SpriteBundle {
            texture: image_assets.logo.clone(),
            transform: Transform::from_translation(logo_start).with_scale(Vec3::splat(0.2)),
            ..default()
        },
        Animator::new(
            logo_in_tween
                .then(Delay::new(Duration::from_secs_f32(2.)))
                .then(logo_out_tween),
        ),
    ));

    let tutorial_start = Vec3::new(0.0, -1000.0, 100.0);
    let tutorial_tween = Tween::new(
        EaseFunction::BounceOut,
        std::time::Duration::from_secs(1),
        TransformPositionLens {
            start: tutorial_start,
            end: Vec3::new(0.0, -50.0, 10.0),
        },
    );
    commands.spawn((
        LevelSelectStuff,
        SpriteBundle {
            texture: image_assets.tutorial.clone(),
            transform: Transform::from_translation(tutorial_start).with_scale(Vec3::splat(0.7)),
            ..default()
        },
        Animator::new(Delay::new(Duration::from_secs_f32(4.)).then(tutorial_tween)),
    ));

    let button_tween = Tween::new(
        EaseFunction::BounceOut,
        std::time::Duration::from_secs(1),
        bevy_tweening::lens::UiPositionLens {
            start: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Px(-300.0)),
            end: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Px(50.0)),
        },
    );

    commands
        .spawn((
            LevelSelectStuff,
            ButtonBundle {
                style: Style {
                    bottom: Val::Px(-200.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    // bottom: Val::Px(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: GREEN_400.into(),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(start_game),
            Hoverable(GREEN_600, GREEN_500, GREEN_400),
            // On::<Pointer<Click>>::send_event::<E>(),
            // PlanningUI,
            // NoDeselect,
        ))
        .insert(Animator::new(
            Delay::new(Duration::from_secs_f32(4.)).then(button_tween),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Start",
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

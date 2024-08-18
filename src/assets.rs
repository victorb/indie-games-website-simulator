use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin;

fn not_condition<Marker>(a: impl Condition<Marker>) -> impl Condition<()> {
    IntoSystem::into_system(a.map(|x| !x))
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Intro) // TODO should be INTRO
                // .continue_to_state(GameState::GameCompleted) // For dev
                // .continue_to_state(GameState::LevelSelect) // For dev
                // .continue_to_state(GameState::Planning) // For dev
                .load_collection::<ImageAssets>()
                .load_collection::<SoundAssets>()
                .load_collection::<FontAssets>(),
        )
        .add_event::<PlaySound>()
        .add_systems(
            Update,
            handle_play_sounds.run_if(not_condition(in_state(GameState::Loading))),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "request_new.png")]
    pub request: Handle<Image>,
    #[asset(path = "server_new.png")]
    pub server: Handle<Image>,
    #[asset(path = "server_proxy.png")]
    pub server_proxy: Handle<Image>,
    #[asset(path = "indiegameslogo.png")]
    pub logo: Handle<Image>,
    #[asset(path = "tutorial_new.png")]
    pub tutorial: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/Titles-HelloBlueprint.ttf")]
    pub titles: Handle<Font>,
    #[asset(path = "fonts/Text-EdgecuttingLiteMedium.ttf")]
    pub texts: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct SoundAssets {
    #[asset(path = "sounds/server_process.ogg")]
    pub process_server: Handle<AudioSource>,
    #[asset(path = "sounds/proxy_process.ogg")]
    pub process_proxy: Handle<AudioSource>,
    #[asset(path = "sounds/click.ogg")]
    pub click_button: Handle<AudioSource>,
    #[asset(path = "sounds/dropped.ogg")]
    pub dropped: Handle<AudioSource>,
}

pub enum Sound {
    ServerProcess,
    ProxyProcess,
    ClickButton,
    DroppedRequest,
}

#[derive(Event)]
pub struct PlaySound(pub Sound);

fn handle_play_sounds(
    mut commands: Commands,
    mut evs: EventReader<PlaySound>,
    sounds: Res<SoundAssets>,
) {
    for ev in evs.read() {
        commands.spawn(AudioBundle {
            source: match ev.0 {
                Sound::ServerProcess => sounds.process_server.clone(),
                Sound::ClickButton => sounds.click_button.clone(),
                Sound::ProxyProcess => sounds.process_proxy.clone(),
                Sound::DroppedRequest => sounds.dropped.clone(),
            },
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
            ..default()
        });
    }
}

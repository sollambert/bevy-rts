#![allow(unused)]
use bevy::{pbr::{ShadowFilteringMethod, ShadowSamplers}, prelude::*};

#[derive(Resource)]
pub struct Settings {
    accessibility: AccessibilitySettings,
    audio: AudioSettings,
    game: GameSettings,
    input: InputSettings,
    video: VideoSettings,
}

#[derive(Resource)]
pub struct AccessibilitySettings;

#[derive(Resource)]
pub struct AudioSettings {
    master_volume: f32,
    music_volume: f32,
    sfx_volume: f32,
    voice_volume: f32,
}

#[derive(Resource)]
pub struct GameSettings;

#[derive(Resource)]
pub struct InputSettings;

#[derive(Resource)]
pub struct VideoSettings {
    anti_aliasing: Msaa,
    shadow_filtering_method: ShadowFilteringMethod,
}
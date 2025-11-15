use bevy::camera::visibility::RenderLayers;

/// In-game resolution width.
pub const RES_WIDTH: u32 = 640;

/// In-game resolution height.
pub const RES_HEIGHT: u32 = 360;

/// Default render layers for pixel-perfect rendering.
pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// Render layers for high-resolution rendering.
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

// Player related constants.
pub const PLAYER_VELOCITY_X: f32 = 200.0;
pub const PLAYER_JUMP_VELOCITY: f32 = 450.0;
pub const MAX_JUMP_HEIGHT: f32 = 115.0;

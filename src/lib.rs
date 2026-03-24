mod render;

use bevy::{
    camera::visibility::{self, NoFrustumCulling, VisibilityClass},
    prelude::*,
    render::sync_world::SyncToRenderWorld,
};

pub struct InfiniteGridPlugin;

impl Plugin for InfiniteGridPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        render::render_app_builder(app);
    }
}

/// The component used to represent an infinite grid.
///
/// This is intended for use as a ground plane in editor-like tools.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
#[require(
    InfiniteGridSettings,
    Transform,
    Visibility,
    VisibilityClass,
    NoFrustumCulling,
    SyncToRenderWorld
)]
#[component(on_add = visibility::add_visibility_class::<InfiniteGrid>)]
pub struct InfiniteGrid;

/// Component to configure the infinite grid
///
/// This component can be applied directly on the grid entity or on a camera that can see the grid
#[derive(Component, Copy, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct InfiniteGridSettings {
    /// The color of the X axis
    pub x_axis_color: Color,
    /// The color of the Z axis
    pub z_axis_color: Color,
    /// The color of the minor lines of the grid
    pub minor_line_color: Color,
    /// The color of the major lines of the grid. Every 10th line is considered major
    pub major_line_color: Color,
    /// How far the grid will be visible relative to the camera
    pub fadeout_distance: f32,
    /// How quickly the grid will fadeout
    pub dot_fadeout_strength: f32,
    /// The scale of the distance between the lines. A smaller value increases the distance between
    /// the lines
    pub scale: f32,
}

impl Default for InfiniteGridSettings {
    fn default() -> Self {
        Self {
            // These colors are copied from bevy_feathers but we don't need to depend on it just
            // for that
            x_axis_color: Color::srgb(1.0, 0.2, 0.2),
            z_axis_color: Color::srgb(0.2, 0.2, 1.0),
            minor_line_color: Color::srgb(0.1, 0.1, 0.1),
            major_line_color: Color::srgb(0.25, 0.25, 0.25),
            fadeout_distance: 100.,
            dot_fadeout_strength: 0.25,
            scale: 1.0,
        }
    }
}

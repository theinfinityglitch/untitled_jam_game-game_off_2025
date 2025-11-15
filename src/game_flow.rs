use crate::*;
use bevy::{
    camera::RenderTarget,
    color::palettes::css::GRAY,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::WindowResized,
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct OuterCamera;

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: constants::RES_WIDTH,
        height: constants::RES_HEIGHT,
        ..default()
    };

    // This Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // Fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // This camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2d,
        Camera {
            // Render before the "main pass" camera
            order: -1,
            target: RenderTarget::Image(image_handle.clone().into()),
            clear_color: ClearColorConfig::Custom(GRAY.into()),
            ..default()
        },
        Transform::from_xyz(
            (constants::RES_WIDTH as f32) / 2.0f32,
            (constants::RES_HEIGHT as f32) / 2.0f32,
            0.0f32,
        ),
        Msaa::Off,
        InGameCamera,
        constants::PIXEL_PERFECT_LAYERS,
    ));

    // Spawn the canvas
    commands.spawn((
        Sprite::from_image(image_handle),
        Canvas,
        constants::HIGH_RES_LAYERS,
    ));

    // The "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2d, Msaa::Off, OuterCamera, constants::HIGH_RES_LAYERS));
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_canvas(
    mut resize_messages: MessageReader<WindowResized>,
    mut projection: Single<&mut Projection, With<OuterCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    for window_resized in resize_messages.read() {
        let h_scale = window_resized.width / constants::RES_WIDTH as f32;
        let v_scale = window_resized.height / constants::RES_HEIGHT as f32;
        projection.scale = 1. / h_scale.min(v_scale);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut rapier_config: Query<&mut RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<(), BevyError> {
    // Spawn a blue square for representating a test player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(16.0f32, 16.0f32))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.0f32, 0.0f32, 1.0f32))),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(16.0f32 / 2.0f32, 16.0f32 / 2.0f32),
        KinematicCharacterController::default(),
        Transform::from_xyz(640.0f32 / 2.0f32, 320.0f32, 3.0f32),
        player::Player,
        constants::PIXEL_PERFECT_LAYERS,
    ));

    // rapier_config.single_mut()?.gravity = Vec2::new(0.0, -2000.0);

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("test_ldtk.ldtk").into(),
            ..Default::default()
        },
        constants::PIXEL_PERFECT_LAYERS,
    ));

    Ok(())
}

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup))
            .add_systems(Update, fit_canvas)
            .add_systems(Update, player::player_system)
            .add_systems(Update, player::jump)
            .add_systems(Update, player::rise)
            .add_systems(Update, player::fall);
    }
}

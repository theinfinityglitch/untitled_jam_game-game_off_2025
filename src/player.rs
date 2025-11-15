use crate::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Jump(f32);

fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) -> Result<(), BevyError> {
    if query.is_empty() {
        return Ok(());
    }

    let (player, output) = query.single()?;

    if input.pressed(KeyCode::Space) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }

    Ok(())
}

fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) -> Result<(), BevyError> {
    if query.is_empty() {
        return Ok(());
    }

    let (entity, mut player, mut jump) = query.single_mut()?;

    let mut movement = time.delta().as_secs_f32() * constants::PLAYER_JUMP_VELOCITY;

    if movement + jump.0 >= constants::MAX_JUMP_HEIGHT {
        movement = constants::MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }

    Ok(())
}

fn fall(
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController, Without<Jump>>,
    config: Query<&RapierConfiguration>,
) -> Result<(), BevyError> {
    if query.is_empty() {
        return Ok(());
    }

    let mut player = query.single_mut()?;

    let gravity: f32 = config.single()?.gravity.y;

    let movement = time.delta().as_secs_f32() * gravity * 0.5;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, vec.y + movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }

    Ok(())
}

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
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
        Transform::from_xyz(340.0f32, 320.0f32, 3.0f32),
        player::Player,
        constants::PIXEL_PERFECT_LAYERS,
    ));

    Ok(())
}

fn player_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
) -> Result<(), BevyError> {
    let mut player = query.single_mut()?;

    let mut movement = 0.0f32;

    if input.pressed(KeyCode::KeyD) {
        movement += time.delta_secs() * constants::PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::KeyA) {
        movement += time.delta_secs() * constants::PLAYER_VELOCITY_X * -1.0f32;
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }

    Ok(())
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, player_system)
            .add_systems(Update, jump)
            .add_systems(Update, rise)
            .add_systems(Update, fall);
    }
}

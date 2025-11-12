use crate::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Jump(f32);

pub fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = match query.single() {
        Ok(result) => result,
        Err(_) => return,
    };

    if input.pressed(KeyCode::Space) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}

pub fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = match query.single_mut() {
        Ok(result) => result,
        Err(_) => return,
    };

    let mut movement = time.delta().as_secs_f32() * constants::PLAYER_VELOCITY_Y;

    if movement + jump.0 >= constants::MAX_JUMP_HEIGHT {
        movement = constants::MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

pub fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    let player = query.single_mut();

    // I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
    let movement = time.delta().as_secs_f32() * (constants::PLAYER_VELOCITY_Y / 1.5) * -1.0;

    match player {
        Ok(mut player) => match player.translation {
            Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
            None => player.translation = Some(Vec2::new(0.0, movement)),
        },
        Err(_) => return,
    }
}

#[derive(Component)]
pub struct Player;

pub fn player_system(
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

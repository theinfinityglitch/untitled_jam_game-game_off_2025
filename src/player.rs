use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn player_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut dir: Vec2 = Vec2::ZERO;

    if input.pressed(KeyCode::KeyA) {
        dir.x -= 1.0f32;
    }

    if input.pressed(KeyCode::KeyD) {
        dir.x += 1.0f32;
    }

    if input.pressed(KeyCode::KeyW) {
        dir.y += 1.0f32;
    }

    if input.pressed(KeyCode::KeyS) {
        dir.y -= 1.0f32;
    }

    if dir != Vec2::ZERO {
        dir = dir.normalize();

        let vel: Vec3 = Vec3::new(
            (dir.x * 50.0f32) * time.delta_secs(),
            (dir.y * 50.0f32) * time.delta_secs(),
            0.0f32,
        );

        for mut player in &mut query {
            player.translation += vel;
        }
    }
}

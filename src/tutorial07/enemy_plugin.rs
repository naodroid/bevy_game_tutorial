use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{ Enemy, GlobalEnemyState, PlayerShip};

// Plugin
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GlobalEnemyState::default())
            .add_system(spawn_enemy_system.system())
            .add_system(move_enemy_system.system());

    }
    fn name(&self) -> &str {
        "EnemyPlugin"
    }
}

fn spawn_enemy_system(
    commands: &mut Commands,
    mut global_state: ResMut<GlobalEnemyState>,
    player: Query<&Transform, With<PlayerShip>>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    if global_state.frames_to_next_enemy > 0 {
        global_state.frames_to_next_enemy -= 1;
        return
    }
    global_state.frames_to_next_enemy = 90; //1.5sec

    let player_tr = player.iter().next().unwrap();
    let window = windows.iter().next().unwrap();
    let win_w = window.width();
    let win_h = window.height();
    //
    commands.spawn(SpriteBundle {
        material: materials.add(asset_server.load("circle.png").into()),
        transform: create_enemy_position(&player_tr, win_w, win_h),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    }).with(
        Enemy::default()
    );
}
fn create_enemy_position(player_transform: &Transform, win_w: f32, win_h: f32) -> Transform {
    let px = player_transform.translation.x;
    let py = player_transform.translation.y;

    loop {
        let x = random::<f32>() * win_w - win_w / 2.;
        let y = random::<f32>() * win_h - win_h / 2.;
        //avoid near place from player, to avoid immediate-collision
        let dx = px - x;
        let dy = py - y;
        if dx * dx + dy * dy >= 400.0 {
            return Transform::from_translation(Vec3 {x: x, y: y, z: 0.});
        }
    }
}

fn move_enemy_system(
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Query<&Transform, With<PlayerShip>>,
) {
    let player_tr = player.iter().next().unwrap();
    let px = player_tr.translation.x;
    let py = player_tr.translation.y;
    for mut enemy in enemies.iter_mut() {
        let ex = enemy.translation.x;
        let ey = enemy.translation.y;
        let mv = Vec2 {x: px - ex, y: py - ey}.normalize() * 0.5;
        enemy.translation.x += mv.x;
        enemy.translation.y += mv.y;
    }
}


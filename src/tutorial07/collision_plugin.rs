use bevy::prelude::*;
use crate::components::{PlayerShip, GunState, Bullet, Enemy};


fn bullet_enemy_collision_system(
    commands: &mut Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>
) {
    let mut hit_enemy_ids: Vec<u32> = Vec::new();

    for (b_entity, b_tr) in bullets.iter() {
        for (e_entity, e_tr) in enemies.iter() {
            //if already the enemy was used, not check
            let enemy_id = e_entity.id();
            if hit_enemy_ids.contains(&enemy_id) {
                continue;
            }
            //use circle collision to make this logic simple
            let diff = b_tr.translation.distance(e_tr.translation);
            if diff < 20.0 {
                //hit! remove the bullet and the enemy.
                commands.despawn(b_entity);
                commands.despawn(e_entity);
                hit_enemy_ids.push(enemy_id);
                break;
            }
        }
    }
}

fn player_enemy_collision_system(
    players: Query<&Transform, With<Bullet>>,
    enemies: Query<&Transform, With<Enemy>>
) {
    if let Some(player) = players.iter().next() {
        for enemy in enemies.iter() {
            let diff = player.translation.distance(enemy.translation);
            if diff < 20.0 {
                //Hit! go to GameOver Scene
            }
        }
    }
}




// Plugin
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(bullet_enemy_collision_system.system())
            .add_system(player_enemy_collision_system.system());
    }

    fn name(&self) -> &str {
        "Collision"
    }
}
use bevy::prelude::*;
use crate::components::{PlayerShip, GunState, Bullet, Enemy};


fn bullet_enemy_collision_system(
    mut commands: Commands,
    queries: QuerySet<(
        Query<(Entity, &Transform), With<Bullet>>,
        Query<(Entity, &Transform), With<Enemy>>
    )>
) {
    let mut hit_enemy_ids: Vec<u32> = Vec::new();
    let bullets = queries.q0();
    let enemies = queries.q1();
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
                commands.entity(b_entity).despawn();
                commands.entity(e_entity).despawn();
                hit_enemy_ids.push(enemy_id);
                break;
            }
        }
    }
}

fn player_enemy_collision_system(
    queries: QuerySet<(
        Query<&Transform, With<PlayerShip>>,
        Query<&Transform, With<Enemy>>
    )>
) {
    let players = queries.q0();
    let enemies = queries.q1();
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
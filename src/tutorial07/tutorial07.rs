use bevy::prelude::*;

mod player_ship_plugin;
mod bullet_plugin;
mod components;
mod enemy_plugin;
mod collision_plugin;

use player_ship_plugin::PlayerShipPlugin;
use bullet_plugin::BulletPlugin;
use crate::enemy_plugin::EnemyPlugin;
use crate::collision_plugin::CollisionPlugin;

//
fn setup(
    mut commands: Commands,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}

//
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Game Title".to_string(),
            width: 480.0,
            height: 320.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerShipPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollisionPlugin)
        .add_startup_system(setup.system())
        .run();
}

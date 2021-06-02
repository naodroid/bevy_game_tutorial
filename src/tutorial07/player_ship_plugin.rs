use bevy::prelude::*;

use crate::components::PlayerShip;

// Plugin
pub struct PlayerShipPlugin;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(asset_server.load("triangle.png").into()),
            transform: Transform::identity(),
            sprite: Sprite::new(Vec2::new(25.0, 25.0)),
            ..Default::default()
        }).insert(
        PlayerShip::default()
    );
}

//
fn follow_cursor_system(
    mut query: Query<&mut PlayerShip>,
    mut events: EventReader<CursorMoved>,
    windows: Res<Windows>,
) {
    //Use unwrap because the app always has a window.
    let window = windows.iter().next().unwrap();
    //But events don't happen every frame. We need to check whether the value exists or not.
    if let Some(value) = events.iter().last() {
        for mut player in query.iter_mut() {
            player.target_x = value.position.x - window.width() / 2.0;
            player.target_y = value.position.y - window.height() / 2.0;
        }
    }
}

fn move_player_system(
    mut query: Query<(&mut Transform, &PlayerShip)>
) {
    for (mut tr, player) in query.iter_mut() {
        let current = Vec2::new(tr.translation.x, tr.translation.y);
        let target = Vec2::new(player.target_x, player.target_y);
        let diff = target - current;
        if diff.length() >= 1.0 {
            //move
            let mv = diff.normalize() * 1.5;
            tr.translation.x += mv.x;
            tr.translation.y += mv.y;
            //rotate
            let y_axis = Vec2::new(0.0, 1.0);
            let angle = y_axis.angle_between(diff);
            tr.rotation = Quat::from_rotation_z(angle);
        }
    }
}

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(follow_cursor_system.system())
            .add_system(move_player_system.system());
    }

    fn name(&self) -> &str {
        "PlayerShip"
    }
}
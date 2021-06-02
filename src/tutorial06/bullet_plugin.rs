use bevy::prelude::*;
use crate::components::{PlayerShip, GunState, Bullet};

//
fn fire_bullet_system(
    mut commands: Commands,
    player: Query<&Transform, With<PlayerShip>>,
    mut gun_state: ResMut<GunState>,
    input: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    if gun_state.frames_to_fire == 0 {
        if input.pressed(MouseButton::Left) {
            let player_transform = player.iter().next().unwrap();
            let tr = *player_transform;
            commands
                .spawn()
                .insert_bundle(
                SpriteBundle {
                    material: materials.add(asset_server.load("elipse.png").into()),
                    transform: tr,
                    sprite: Sprite::new(Vec2::new(10.0, 20.0)),
                    ..Default::default()
                }).insert(
                Bullet
            );
        }
        gun_state.frames_to_fire = 5;
    } else {
        gun_state.frames_to_fire -= 1;
    }
}

fn move_bullet_system(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<Bullet>>,
    windows: Res<Windows>,
) {
    let window = windows.iter().next().unwrap();
    let win_w = window.width() / 2.;
    let win_h = window.height() / 2.;

    for (entity, mut bullet) in bullets.iter_mut() {
        let (v, a) = bullet.rotation.to_axis_angle();
        let angle = if v.z > 0. { a } else { -a };
        let tr = &mut bullet.translation;
        //notice: the angle when the ship toward upside is Zero,
        // left is PI/2, Down: PI, Right: -PI/2
        tr.x -= angle.sin() * 10.;
        tr.y += angle.cos() * 10.;

        //despawn it if outside of the window
        if tr.x < -win_w || tr.x > win_w || tr.y < -win_h || tr.y > win_h {
            commands.entity(entity).despawn();
        }
    }
}

// Plugin
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GunState::default())
            .add_system(fire_bullet_system.system())
            .add_system(move_bullet_system.system());
    }

    fn name(&self) -> &str {
        "BulletSystem"
    }
}

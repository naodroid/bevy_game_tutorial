use bevy::prelude::*;

#[derive(Default)]
struct PlayerShip {
    target_x: f32,
    target_y: f32,
}

struct Bullet;

#[derive(Default)]
struct GunState {
    frames_to_fire: i32,
}


//
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(asset_server.load("triangle.png").into()),
            transform: Transform::identity(),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
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
            let mv = diff.normalize();
            tr.translation.x += mv.x;
            tr.translation.y += mv.y;
            //rotate
            let y_axis = Vec2::new(0.0, 1.0);
            let angle = y_axis.angle_between(diff);
            tr.rotation = Quat::from_rotation_z(angle);
        }
    }
}

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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(GunState::default())
        .add_startup_system(setup.system())
        .add_system(follow_cursor_system.system())
        .add_system(move_player_system.system())
        .add_system(fire_bullet_system.system())
        .add_system(move_bullet_system.system())
        .run();
}

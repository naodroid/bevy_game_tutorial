use bevy::prelude::*;

#[derive(Default)]
struct PlayerShip {
    target_x: f32,
    target_y: f32
}

//
fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(asset_server.load("triangle.png").into()),
            transform: Transform::identity(),
            sprite: Sprite::new(Vec2::new(80.0, 80.0)),
            ..Default::default()
        }).with(
            PlayerShip::default()
        );
}

//
fn follow_cursor_system(
    mut query: Query<&mut PlayerShip>,
    events: Res<Events<CursorMoved>>,
    mut reader: Local<EventReader<CursorMoved>>,
    windows: Res<Windows>,
) {
    //Use unwrap because the app always has a window.
    let window = windows.iter().next().unwrap();
    //But events don't happen every frame. We need to check whether the value exists or not.
    if let Some(value) = reader.iter(&events).last() {
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
        let current = Vec2 { x: tr.translation.x, y: tr.translation.y };
        let target = Vec2 { x: player.target_x, y: player.target_y };
        let diff = target - current;
        if diff.length() >= 1.0 {
            //move
            let mv = diff.normalize();
            tr.translation.x += mv.x;
            tr.translation.y += mv.y;
            //rotate
            let y_axis = Vec2 { x: 0.0, y: 1.0 };
            let angle = y_axis.angle_between(diff);
            tr.rotation = Quat::from_rotation_z(angle);
        }
    }
}

//
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Game Title".to_string(),
            width: 480.0,
            height: 320.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(follow_cursor_system.system())
        .add_system(move_player_system.system())
        .run();
}

use bevy::prelude::*;

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
        });
}

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
        .run();
}

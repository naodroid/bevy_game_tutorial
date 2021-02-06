use bevy::prelude::*;
use bevy::input::keyboard::{KeyboardInputState, KeyboardInput};
use bevy::input::mouse::MouseButtonInput;

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
// Mouse
fn mouse_move_system(
    events: Res<Events<CursorMoved>>,
    mut reader: Local<EventReader<CursorMoved>>
) {
    for ev in reader.iter(&events) {
        println!("CURSOR_MOVED: {:?}", ev.position);
    }
}
fn mouse_click_system(
    events: Res<Events<MouseButtonInput>>,
    mut reader: Local<EventReader<MouseButtonInput>>
) {
    for ev in reader.iter(&events) {
        match (ev.button, ev.state.is_pressed()) {
            (MouseButton::Left, true) => println!("MouseClick:Left"),
            (MouseButton::Left, false) => println!("MouseClick:Left-Released"),
            (MouseButton::Right, true) => println!("MouseClick:Right"),
            (MouseButton::Middle, true) => println!("MouseClick:Middle"),
            (MouseButton::Other(code), true) => println!("MouseClick:Other, {}", code),
            _ => (),
        }
    }
}
fn mouse_state_system(input: Res<Input<MouseButton>>) {
    if input.pressed(MouseButton::Left) {
        //show every frame if the left button keep pressed
        println!("MouseState:Pressed-Left");
    }
    if input.just_pressed(MouseButton::Right) {
        //show every frame if right button is pressed.
        //and don't show again until right button will be released
        println!("MouseState:JustPressed-Right");
    }
}

// Keyboard
fn keyboard_event_system(
    events: Res<Events<KeyboardInput>>,
    mut reader: Local<EventReader<KeyboardInput>>
) {
    for ev in reader.iter(&events) {
        match (ev.key_code, ev.state.is_pressed()) {
            (Some(KeyCode::Space), true) => println!("Pressed"),
            (Some(KeyCode::Space), false) => println!("Releaed"),
            _ => ()
        }
    }
}
fn keyboard_state_system(
    state: Res<Input<KeyCode>>
) {
    if state.pressed(KeyCode::Up) {
        println!("KeyState:Up");
    }
}

// Own event
struct GameOverEvent;

fn send_game_over_event_system(
    input: Res<Input<MouseButton>>,
    mut events: ResMut<Events<GameOverEvent>>,
) {
    //send original event when left button pressed
    if input.just_pressed(MouseButton::Left) {
        //show every frame if the left button keep pressed
        events.send(GameOverEvent)
    }
}
fn game_over_event_system(
    events: Res<Events<GameOverEvent>>,
    mut reader: Local<EventReader<GameOverEvent>>
) {
    for _ in reader.iter(&events) {
        println!("GameOver!")
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
        .add_system(mouse_move_system.system())
        .add_system(mouse_click_system.system())
        .add_system(mouse_state_system.system())
        .add_system(keyboard_event_system.system())
        .add_event::<GameOverEvent>()
        .add_system(keyboard_state_system.system())
        .add_system(send_game_over_event_system.system())
        .add_system(game_over_event_system.system())
        .run();
}

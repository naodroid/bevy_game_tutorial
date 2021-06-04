# Events

Most of applications that have user interface need to act user inputs, such as mouse and keyboard events.

In this chapter, we'll focus on user input event on Bevy.

## Mouse

In Bevy, user inputs are processed in `system` functions, similar to updating values.

### Move

The parameters of functions is `Event`, but a little complex. The following code is to receive mouse cursor moved events from system.

```rust
//add this line at the top of the file
use bevy::window::CursorMoved;

//
fn mouse_event_system(
    mut events: EventReader<CursorMoved>,
) {
    for ev in events.iter() {
        println!("CURSOR_MOVED: {:?}", ev.position);
    }
}

//Don't forget to add this function to Bevy Engine using "add_system"
```

Two objects are required to read events , `Event` and `EventReader`. Both are declared as function parameters and passed to the function from the Engine.

And use `for-loop` to fetch them, because many events can be happened during the run-loop. But in most case you don't need to use all events. Using the last(or first) event is sufficient (and makes the code easy).

> I suppose this structure, using both Event and EventReader, will change in the future version because the code is redundancy.

### Click Event

Click Event can be read in the same way.

```rust
use bevy::input::mouse::MouseButtonInput;

fn mouse_click_system(
    mut events: EventReader<MouseButtonInput>,
) {
    for ev in events.iter() {
        match (ev.button, ev.state.is_pressed()) {
            (MouseButton::Left, true) => println!("LeftClick!"),
            (MouseButton::Left, false) => println!("Left Released"),
            (MouseButton::Right, true) => println!("RightClick!"),
            (MouseButton::Middle, true) => println!("MiddleClick!"),
            (MouseButton::Other(code), true) => println!("OTHER, {}", code),
            _ => (),
        }
    }
}
```

### Check pressing pattern

Sometime you want to check whether the button is pressing or not *at every frame*. In this case, Event Driven pattern is bothering, you need to hold bool value(pressing or not) in yourown.

```rust
fn mouse_state_system(input: EventReader<MouseButton>) {
    if input.pressed(MouseButton::Left) {
        println!("MouseState:Pressed-Left");
    }
    if input.just_pressed(MouseButton::Right) {
        println!("MouseState:JustPressed-Right");
    }
}
```

## Keyboard

Checking keyboard is as same as mouse. Event and State are available. 

```rust
// Keyboard
fn keyboard_event_system(
    mut events: EventReader<KeyboardInput>,
) {
    for ev in events.iter() {
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
```

## Original Event

In Bevy you can send/receive your original event in the same way. It's useful when changing a large global state, such as *GameOver*.

```rust
// Own event
struct GameOverEvent;

fn send_game_over_event_system(
    input: Res<Input<MouseButton>>,
    mut events: EventWriter<GameOverEvent>,
) {
    //send original event when left button pressed
    if input.just_pressed(MouseButton::Left) {
        //show every frame if the left button keep pressed
        events.send(GameOverEvent)
    }
}
fn game_over_event_system(
    mut events: EventReader<GameOverEvent>
) {
    for _ in events.iter() {
        println!("GameOver!")
    }
}

//
fn main() {
    App::build()
        //...
        //Register the original event to engine to use them.
        .add_event::<GameOverEvent>()
        .add_system(send_game_over_event_system.system())
        .add_system(game_over_event_system.system())
        .run();
}
```

> Notice: Register your event structs. If you forget it, the app will crash.

# Summary

* Use `Res<Events<EventType>>` and `Local<EventReader<EventType>>` to input events
* In some pattern `Res<Input<Type>>` is useful. 
* You can declare your original events.

Next chapter will explain how to move tringle-texture following mouse cursor.


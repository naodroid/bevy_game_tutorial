# Plugins

We made two systems, `PlayerShip` and `Bullets`. And the code became larger and hard to read. 

In this chapter, I'll split the code into some files with `Plugin`.


## Split file

First of all let's split the main file. 

* components.rs
  * descrpibe all components
* playership.rs
  * Player system, startup and moving
* bullets.rs
  * Bullets system, spawn(despawn) and moving
* main.rs
  * refer these files and use them

This is a common Rust-way,  butin Bevy we can bundle the system as `Plugin`.

----------------------

## Plugin

`Plugin` is a set of systems and registers them in app build phase.You must have used plugins in the first chapter, `DefaultPlugins`.

Now let's see the source code of bevy's simple plugin to know what `Plugin` is. In this tutorial I'll use `CorePlugin` which is simple and includes basic functions.

[CorePlugin source - 0.4.0](https://docs.rs/bevy_core/0.4.0/src/bevy_core/lib.rs.html#25)

--------------

This is the main part of `CorePlugin`

```rust

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        //
    }
}
```

Some required systems will be added to `AppBuilder` in `build` function. It's very simple. And register it in `main.rs` with `add_plugin`.

```rust
fn main() {
    App::build()
        .add_plugins(CorePlugins);
```

Then *CorePlugin.build* is called when the application's startup phase. 

-----------------------

For example `PlayerShip` plugin will be the following.

```rust
//player_ship.rs
pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(follow_cursor_system.system())
            .add_system(move_player_system.system());
    }
}
fn setup(...) {
    //spawn playership
}
fn follow_cursor_system(...) {}
fn move_player_system(..) {}
```

Systems like `setup` and others don't need to be exposed and this means you can modify player-systems without changing `main.rs`. I suppose this is familier for many programmers who are used to Object-Oriented-Programming, `Encapsulate`.)

## Components

But I didn't add `PlayrShip` component itself into `player_ship.rs`, because `PlayerShip` is also used in `fire_bullet_system`.  In the future `PlayerShip` will be also in a collision system.

So I suppose that all components should be bundled in one file to make the project flexible. All plugins refer `components.rs` only (except Plugins that bundles sub-Plugins).

> How to split the code and what to be `public` is very difficult problem. <br>
> The best solution depends on the code scale and project members' preference.

## Summary 

`Plugin ` is a modular system in Bevy. It can

* modularize bundle startup and system functions.
* conceal detail implementations of the systems.

It's very usuful but you should consider the project-files structure carefully especially when the code scale is to become very large.

In the next chapter, I'll create `Enemy` with `Plugin`.











use bevy::prelude::*;
use bevy::app::*;
use bevy::utils::Duration;

struct Block;

struct Wall;

struct Position(i32);

//
fn setup(
    mut commands: Commands,
) {
    commands.spawn()
        .insert(Position(1))
        .insert(Block);
    commands.spawn()
        .insert(Position(10))
        .insert(Block);
    commands.spawn()
        .insert(Position(5))
        .insert(Wall);
    commands.spawn()
        .insert(Position(20))
        .insert(Wall);
}
fn print_system(
    query: Query<&Position, With<Block>>
) {
    println!("## Print ##");
    for p in query.iter() {
        println!(" Block-{}", p.0);
    }
}
fn move_block_system(
    mut query: Query<&mut Position, With<Block>>
) {
    for mut p in query.iter_mut() {
        p.0 += 1
    }
}
fn collision_system(
    mut commands: Commands,
    queries: QuerySet<(
        Query<(Entity, &Position), With<Block>>,
        Query<&Position, With<Wall>>
    )>,
) {
    for (block_entity, block_pos) in queries.q0().iter() {
        for wall_pos in queries.q1().iter() {
            if block_pos.0 == wall_pos.0 {
                commands.entity(block_entity).despawn();
            }
        }
    }
}

// Main
fn main() {
    App::build()
        //run every second
        .insert_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop { wait: Some(Duration::from_secs(1)) }
        })
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(setup.system())
        .add_system(move_block_system.system())
        .add_system(collision_system.system())
        .add_system(print_system.system())
        .run();
}

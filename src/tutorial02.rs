use bevy::prelude::*;
use bevy::app::*;
use bevy::utils::Duration;

struct Block;

struct Wall;

struct Position(i32);

//
fn setup(
    commands: &mut Commands,
) {
    commands
        .spawn((Block, Position(1)))
        .spawn((Block, Position(10)))
        .spawn((Wall, Position(5)))
        .spawn((Wall, Position(20)));
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
    commands: &mut Commands,
    blocks: Query<(Entity, &Position), With<Block>>,
    walls: Query<&Position, With<Wall>>
) {
    for (block_entity, block_pos) in blocks.iter() {
        for wall_pos in walls.iter() {
            if block_pos.0 == wall_pos.0 {
                commands.despawn(block_entity);
            }
        }
    }
}

// Main
fn main() {
    App::build()
        //run every second
        .add_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop { wait: Some(Duration::from_secs(1)) }
        })
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(setup.system())
        .add_system(move_block_system.system())
        .add_system(collision_system.system())
        .add_system(print_system.system())
        .run();
}

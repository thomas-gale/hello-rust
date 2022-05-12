use bevy::{
    core::Name,
    prelude::{
        App, Commands, Component, Entity, FromWorld, Plugin, Query, Res, ResMut, With, World,
    },
    tasks::{AsyncComputeTaskPool, Task},
};
use std::thread::sleep;
extern crate queues;
use queues::*;

pub struct TerrainGeneratePlugin;

impl Plugin for TerrainGeneratePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GenerateTerrainQueue>()
            .add_startup_system(spawn_terrain);
    }
}

// Following code ideas attribution: https://stackoverflow.com/questions/71504675/how-to-make-async-system-function-in-the-bevy-game-engine

// Basic terrain entity spawn and terrain component type.
// Component to signal that this entity is the terrain - todo check this is the best way.
#[derive(Component)]
struct Terrain;

fn spawn_terrain(mut commands: Commands) {
    commands
        .spawn()
        .insert(Terrain)
        .insert(Name::new("terrain generation"));
}

// Resource for storing a queue of terrain generation tasks.
struct GenerateTerrainQueue(Queue<[i32; 3]>);

impl FromWorld for GenerateTerrainQueue {
    fn from_world(_world: &mut World) -> Self {
        // let mut queue = world.get_resource_mut::<GenerateTerrainQueue>().unwrap();
        let mut queue: Queue<[i32; 3]> = queue![];
        let res = queue.add([1, 2, 3]);
        if res.is_err() {
            panic!("queue is full");
        }
        GenerateTerrainQueue(queue)
    }
}

// A Component to attach to any terrain entity that needs to be generated.
#[derive(Component)]
struct GenerateTerrainTask {
    task: Task<String>,
}

// The expensive terrain generation calculation.
fn generate_chunk(chunk: [i32; 3]) {
    println!(
        "starting chunk calculation for x{} y{} z{}...",
        chunk[0], chunk[1], chunk[2]
    );
    sleep(std::time::Duration::from_secs(2));
    println!("calculate chunk complete!");
}

fn prepare_generate_tasks(
    mut queue: ResMut<GenerateTerrainQueue>,
    mut query: Query<Entity, With<Terrain>>,
    pool: Res<AsyncComputeTaskPool>,
    mut commands: Commands,
) {
    while let chunk_key = queue.0.remove().unwrap() {
        let task = pool.spawn(async move {
            generate_chunk(chunk_key);
            String::from("chunk generated :)")
        });
        let terrain_entity = query.single_mut();
        commands
            .entity(terrain_entity)
            .insert(GenerateTerrainTask { task });
    }
}

fn apply_generate_tasks(pool: Res<AsyncComputeTaskPool>, mut cmds: Commands) {
    todo!();
    return;
}

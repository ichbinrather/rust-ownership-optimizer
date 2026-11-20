mod player;
mod world;

use world::World;

fn main() {
    let mut world = World::demo();
    for _ in 0..3 {
        world.tick();
    }
    println!("{}", world.report());
}

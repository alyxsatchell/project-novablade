pub mod playership;

use crystal_nova::{particle::Particle, physics::Vector, universe::Universe};
use winit::{event_loop::EventLoop, window::WindowBuilder};
use crate::playership::PlayerShip;

fn main() {
    pollster::block_on(run());
}

async fn run(){
    let player: PlayerShip = PlayerShip::new();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut universe = Universe::new(&window, Box::new(player)).await;
    let setup = |uni: &mut Universe| {
        let (phys_col, queue) = uni.get_physics_collection_mut();
        let particle = Particle{
            position: Vector::new(),
            velocity: Vector::new(),
        };
        phys_col.add_particle(particle, queue);
    };
    universe.run(event_loop, Some(Box::new(setup))).await;
}
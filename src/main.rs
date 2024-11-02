pub mod playership;

use crystal_nova::universe::Universe;
use winit::{event_loop::EventLoop, window::WindowBuilder};
use crate::playership::PlayerShip;

fn main() {
    println!("Hello, world!");
    pollster::block_on(run());
}

async fn run(){
    let mut player: PlayerShip = PlayerShip::new();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut universe = Universe::new(&window, Box::new(player)).await;
    universe.run(event_loop).await;
    println!("test");
}
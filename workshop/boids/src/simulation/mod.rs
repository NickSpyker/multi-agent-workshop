mod data;
pub mod math;
mod message;
mod simulator;

pub use data::Boids;
pub use math::Vec2;
pub use message::MessageFromSimulatorToGui;
pub use simulator::BoidsSimulator;

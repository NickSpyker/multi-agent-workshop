mod gui;
mod simulation;

use gui::PhysarumGui;
use simulation::PhysarumSimulator;

use multi_agent::{AppLauncher, Result};

fn main() -> Result<()> {
    AppLauncher::run::<PhysarumSimulator, PhysarumGui>()
}

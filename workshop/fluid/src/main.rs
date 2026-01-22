mod gui;
mod simulation;

use gui::FluidGui;
use simulation::FluidSimulator;

use multi_agent::{AppLauncher, Result};

fn main() -> Result<()> {
    AppLauncher::run::<FluidSimulator, FluidGui>()
}

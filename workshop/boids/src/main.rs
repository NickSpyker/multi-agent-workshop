mod gui;
mod simulation;

use gui::BoidsGui;
use simulation::BoidsSimulator;

use multi_agent::{AppLauncher, Result};

fn main() -> Result<()> {
    AppLauncher::run::<BoidsSimulator, BoidsGui>()
}

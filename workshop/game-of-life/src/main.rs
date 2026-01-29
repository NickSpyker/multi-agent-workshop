mod gui;
mod rle;
mod simulation;

use gui::GameOfLifeGui;
use simulation::GameOfLifeSimulator;

use multi_agent::{AppLauncher, Result};

fn main() -> Result<()> {
    AppLauncher::run::<GameOfLifeSimulator, GameOfLifeGui>()
}

use super::{GameOfLife, MessageFromSimulatorToGui};
use crate::gui::{GameOfLifeConfig, MessageFromGuiToSimulator};
use multi_agent::{MultiAgentSimulation, Result};
use rayon::prelude::*;
use std::{collections::HashSet, time::Duration};

#[derive(Debug)]
pub struct GameOfLifeSimulator {
    data: GameOfLife,
    accumulated_time: Duration,
}

impl MultiAgentSimulation for GameOfLifeSimulator {
    type SimulationData = GameOfLife;
    type GuiData = GameOfLifeConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(_initial_gui_data: Self::GuiData) -> Result<Self> {
        Ok(Self {
            data: GameOfLife::default(),
            accumulated_time: Duration::ZERO,
        })
    }

    fn update<F>(
        &mut self,
        gui_data: Self::GuiData,
        messages: Vec<Self::MessageFromGui>,
        delta_time: Duration,
        _send_message_to_gui: F,
    ) -> Result<&Self::SimulationData> {
        for message in messages {
            match message {
                Self::MessageFromGui::SpawnCells(cells) => self.data.spawn(cells),
                Self::MessageFromGui::RemoveCells(cells) => self.data.remove(cells),
                Self::MessageFromGui::Reset => {
                    self.data.cells.clear();
                    self.data.generation = 0;
                }
                Self::MessageFromGui::PlacePattern(cells) => self.data.spawn(cells),
            }
        }

        let Self::GuiData {
            paused,
            tick_rate_per_second,
        } = gui_data;

        if !paused && tick_rate_per_second > 0.0 {
            let tick_duration = Duration::from_secs_f32(1.0 / tick_rate_per_second);
            self.accumulated_time += delta_time;

            while self.accumulated_time >= tick_duration {
                self.process_tick();
                self.data.generation += 1;
                self.accumulated_time -= tick_duration;
            }
        }

        Ok(&self.data)
    }
}

impl GameOfLifeSimulator {
    fn process_tick(&mut self) {
        let GameOfLife { cells, .. } = &mut self.data;

        let count_neighbors = |x: i64, y: i64| -> u8 {
            let mut count: u8 = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if (dx != 0 || dy != 0) && cells.contains(&(x + dx, y + dy)) {
                        count += 1;
                    }
                }
            }
            count
        };

        // Collect alive cells + their neighbors
        let candidates: HashSet<(i64, i64)> = cells
            .par_iter()
            .flat_map(|&(x, y)| {
                (-1..=1)
                    .flat_map(move |dx: i64| (-1..=1).map(move |dy: i64| (x + dx, y + dy)))
                    .collect::<Vec<(i64, i64)>>()
            })
            .collect();

        // 1. Underpopulation: Any live cell with fewer than two live neighbors dies.
        // 2. Survival: Any live cell with two or three live neighbors lives on to the next generation.
        // 3. Overpopulation: Any live cell with more than three live neighbors dies.
        // 4. Reproduction: Any dead cell with exactly three live neighbors becomes a live cell.
        *cells = candidates
            .into_par_iter()
            .filter(|&(x, y)| {
                let neighbors: u8 = count_neighbors(x, y);
                let is_alive: bool = cells.contains(&(x, y));

                neighbors == 3 || (is_alive && neighbors == 2)
            })
            .collect();
    }
}

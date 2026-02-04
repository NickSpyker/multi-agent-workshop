use super::{Boids, MessageFromSimulatorToGui, Vec2};
use crate::gui::{BoidsConfig, MessageFromGuiToSimulator};
use multi_agent::{MultiAgentSimulation, Result};
use std::time::Duration;

#[derive(Debug)]
pub struct BoidsSimulator {
    data: Boids,
}

impl MultiAgentSimulation for BoidsSimulator {
    type SimulationData = Boids;
    type GuiData = BoidsConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
        let mut data = Boids::default();
        data.spawn_random(initial_gui_data.boid_count, initial_gui_data.max_speed);

        Ok(Self { data })
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
                MessageFromGuiToSimulator::Reset => {
                    self.data.clear();
                    self.data
                        .spawn_random(gui_data.boid_count, gui_data.max_speed);
                }
                MessageFromGuiToSimulator::SpawnBoids(count) => {
                    self.data.spawn_random(count, gui_data.max_speed);
                }
                MessageFromGuiToSimulator::SetBoidCount(target) => {
                    self.data.set_count(target, gui_data.max_speed);
                }
                MessageFromGuiToSimulator::ResizeWorld(width, height) => {
                    self.data.resize(width, height);
                }
            }
        }

        if !gui_data.paused {
            self.process_tick(&gui_data, delta_time.as_secs_f32());
        }

        Ok(&self.data)
    }
}

impl BoidsSimulator {
    fn process_tick(&mut self, config: &BoidsConfig, dt: f32) {
        // TODO
    }
}

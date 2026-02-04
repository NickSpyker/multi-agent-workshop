use super::{MessageFromSimulatorToGui, Physarum};
use crate::gui::{MessageFromGuiToSimulator, PhysarumConfig};
use fastrand::Rng;
use multi_agent::{MultiAgentSimulation, Result};
use std::time::Duration;

#[derive(Debug)]
pub struct PhysarumSimulator {
    data: Physarum,
    rng: Rng,
}

impl MultiAgentSimulation for PhysarumSimulator {
    type SimulationData = Physarum;
    type GuiData = PhysarumConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
        let mut data = Physarum::new(
            initial_gui_data.width,
            initial_gui_data.height,
        );
        data.spawn_agents(initial_gui_data.agent_count, initial_gui_data.spawn_mode);

        Ok(Self {
            data,
            rng: Rng::new(),
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
                MessageFromGuiToSimulator::Reset => {
                    self.data.clear();
                    self.data.spawn_agents(gui_data.agent_count, gui_data.spawn_mode);
                }
                MessageFromGuiToSimulator::SetAgentCount(count) => {
                    self.data.set_agent_count(count, gui_data.spawn_mode);
                }
                MessageFromGuiToSimulator::ResizeWorld(width, height) => {
                    self.data.resize(width, height);
                }
                MessageFromGuiToSimulator::ClearTrails => {
                    self.data.trail_map.clear();
                }
            }
        }

        if !gui_data.paused {
            let dt = delta_time.as_secs_f32();

            for _ in 0..gui_data.steps_per_frame {
                self.process_agents(&gui_data, dt / gui_data.steps_per_frame as f32);
            }

            self.data.trail_map.diffuse_and_decay(
                gui_data.diffuse_rate,
                gui_data.decay_rate,
                dt,
            );
        }

        Ok(&self.data)
    }
}

impl PhysarumSimulator {
    fn process_agents(&mut self, config: &PhysarumConfig, dt: f32) {
        // TODO
    }
}

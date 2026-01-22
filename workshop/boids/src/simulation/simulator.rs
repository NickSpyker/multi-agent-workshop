use super::{Boids, MessageFromSimulatorToGui};
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
        Ok(Self {
            data: Boids::default(),
        })
    }

    fn update<F>(
        &mut self,
        gui_data: Self::GuiData,
        messages: Vec<Self::MessageFromGui>,
        delta_time: Duration,
        send_message_to_gui: F,
    ) -> Result<&Self::SimulationData> {
        Ok(&self.data)
    }
}

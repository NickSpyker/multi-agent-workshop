use super::{MessageFromSimulatorToGui, Physarum};
use crate::gui::{MessageFromGuiToSimulator, PhysarumConfig};
use multi_agent::{MultiAgentSimulation, Result};
use std::time::Duration;

#[derive(Debug)]
pub struct PhysarumSimulator {
    data: Physarum,
}

impl MultiAgentSimulation for PhysarumSimulator {
    type SimulationData = Physarum;
    type GuiData = PhysarumConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
        Ok(Self {
            data: Physarum::default(),
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

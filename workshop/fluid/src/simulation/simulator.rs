use super::{Fluid, MessageFromSimulatorToGui};
use crate::gui::{FluidConfig, MessageFromGuiToSimulator};
use multi_agent::{MultiAgentSimulation, Result};
use std::time::Duration;

#[derive(Debug)]
pub struct FluidSimulator {
    data: Fluid,
}

impl MultiAgentSimulation for FluidSimulator {
    type SimulationData = Fluid;
    type GuiData = FluidConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
        Ok(Self {
            data: Fluid::default(),
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

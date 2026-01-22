use super::{GameOfLife, MessageFromSimulatorToGui};
use crate::gui::{GameOfLifeConfig, MessageFromGuiToSimulator};
use multi_agent::{MultiAgentSimulation, Result};
use std::time::Duration;

#[derive(Debug)]
pub struct GameOfLifeSimulator {
    data: GameOfLife,
}

impl MultiAgentSimulation for GameOfLifeSimulator {
    type SimulationData = GameOfLife;
    type GuiData = GameOfLifeConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
        Ok(Self {
            data: GameOfLife::default(),
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

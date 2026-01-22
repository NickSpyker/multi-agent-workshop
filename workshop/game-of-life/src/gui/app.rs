use super::{GameOfLifeConfig, MessageFromGuiToSimulator};
use crate::simulation::{GameOfLife, MessageFromSimulatorToGui};
use eframe::Frame;
use egui::{Context, Ui};
use multi_agent::{GuardArc, MultiAgentGui};

#[derive(Debug, Default)]
pub struct GameOfLifeGui {}

impl MultiAgentGui for GameOfLifeGui {
    const APP_NAME: &'static str = "Game of Life";

    type GuiData = GameOfLifeConfig;
    type SimulationData = GameOfLife;

    type MessageFromSimulation = MessageFromSimulatorToGui;
    type MessageToSimulation = MessageFromGuiToSimulator;

    fn received_messages_from_simulation(&mut self, messages: Vec<Self::MessageFromSimulation>) {}

    fn sidebar<F>(
        &mut self,
        simulation_data: &GuardArc<Self::SimulationData>,
        ctx: &Context,
        frame: &mut Frame,
        ui: &mut Ui,
        send_message_to_simulation: F,
    ) -> Option<Self::GuiData> {
        None
    }

    fn content<F>(
        &mut self,
        simulation_data: &GuardArc<Self::SimulationData>,
        ctx: &Context,
        frame: &mut Frame,
        ui: &mut Ui,
        send_message_to_simulation: F,
    ) {
    }
}

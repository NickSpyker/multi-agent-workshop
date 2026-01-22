use super::{MessageFromGuiToSimulator, PhysarumConfig};
use crate::simulation::{Physarum, MessageFromSimulatorToGui};
use eframe::Frame;
use egui::{Context, Ui};
use multi_agent::{GuardArc, MultiAgentGui};

#[derive(Debug, Default)]
pub struct PhysarumGui {}

impl MultiAgentGui for PhysarumGui {
    const APP_NAME: &'static str = "Physarum";

    type GuiData = PhysarumConfig;
    type SimulationData = Physarum;

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

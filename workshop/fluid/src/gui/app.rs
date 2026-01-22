use super::{FluidConfig, MessageFromGuiToSimulator};
use crate::simulation::{Fluid, MessageFromSimulatorToGui};
use eframe::Frame;
use egui::{Context, Ui};
use multi_agent::{GuardArc, MultiAgentGui};

#[derive(Debug, Default)]
pub struct FluidGui {}

impl MultiAgentGui for FluidGui {
    const APP_NAME: &'static str = "Fluid";

    type GuiData = FluidConfig;
    type SimulationData = Fluid;

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

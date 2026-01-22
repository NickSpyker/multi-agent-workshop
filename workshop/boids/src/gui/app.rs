use super::{BoidsConfig, MessageFromGuiToSimulator};
use crate::simulation::{Boids, MessageFromSimulatorToGui};
use eframe::Frame;
use egui::{Context, Ui};
use multi_agent::{GuardArc, MultiAgentGui};

#[derive(Debug, Default)]
pub struct BoidsGui {}

impl MultiAgentGui for BoidsGui {
    const APP_NAME: &'static str = "Boids";

    type GuiData = BoidsConfig;
    type SimulationData = Boids;

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

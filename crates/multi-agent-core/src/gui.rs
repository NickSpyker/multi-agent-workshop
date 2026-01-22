/*
 * Copyright 2026 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::GuardArc;
use eframe::{
    egui::{Context, Ui},
    Frame,
};
use std::fmt::Debug;

/// A multi-agent GUI framework trait.
///
/// Implementations of this trait define the GUI layout and interaction logic.
/// The GUI runs on the main thread using the egui immediate-mode framework and
/// communicates with the simulation through shared state and message channels.
///
/// The GUI is split into two main areas:
/// - **Sidebar**: For controls, settings, and information display
/// - **Content**: For the main visualization area
///
/// # Example
///
/// ```rust,ignore
/// use multi_agent::{MultiAgentGui, GuardArc, eframe::Frame, egui::{Context, Ui}};
///
/// #[derive(Debug, Default)]
/// struct MyGui {
///     config: SimulationConfig,
/// }
///
/// impl MultiAgentGui for MyGui {
///     const APP_NAME: &'static str = "My Simulation";
///
///     type GuiData = SimulationConfig;
///     type SimulationData = Vec<Agent>;
///     type MessageFromSimulation = SimulationMessage;
///     type MessageToSimulation = GuiMessage;
///
///     fn received_messages_from_simulation(&mut self, messages: Vec<Self::MessageFromSimulation>) {
///         for message in messages {
///             // Handle simulation messages
///         }
///     }
///
///     fn sidebar<F>(
///         &mut self,
///         simulation_data: &GuardArc<Self::SimulationData>,
///         ctx: &Context,
///         frame: &mut Frame,
///         ui: &mut Ui,
///         send_message_to_simulation: F,
///     ) -> Option<Self::GuiData>
///     where
///         F: Fn(Self::MessageToSimulation),
///     {
///         ui.heading("Controls");
///         if ui.button("Reset").clicked() {
///             send_message_to_simulation(GuiMessage::Reset);
///         }
///         Some(self.config.clone())
///     }
///
///     fn content<F>(
///         &mut self,
///         simulation_data: &GuardArc<Self::SimulationData>,
///         ctx: &Context,
///         frame: &mut Frame,
///         ui: &mut Ui,
///         send_message_to_simulation: F,
///     ) where
///         F: Fn(Self::MessageToSimulation),
///     {
///         // Draw simulation visualization
///         for agent in simulation_data.iter() {
///             // Render agent
///         }
///     }
/// }
/// ```
pub trait MultiAgentGui: Debug + Default {
    /// The name displayed in the window title bar.
    const APP_NAME: &'static str;

    /// Initial window size in pixels [width, height].
    ///
    /// Default: [1280, 720] (720p HD)
    const WINDOW_SIZE_IN_PIXELS: [f32; 2] = [1280.0, 720.0];

    /// Minimum window size in pixels [width, height].
    ///
    /// The window cannot be resized smaller than this.
    ///
    /// Default: [896, 504] (16:9 aspect ratio)
    const MIN_WINDOW_SIZE_IN_PIXELS: [f32; 2] = [896.0, 504.0];

    /// Background color in RGBA format [red, green, blue, alpha].
    ///
    /// Values range from 0-255.
    ///
    /// Default: [12, 12, 12, 180] (dark gray, semi-transparent)
    const BACKGROUND_RGBA_COLOR: [u8; 4] = [12, 12, 12, 180];

    /// Default width of the sidebar in pixels.
    ///
    /// Users can resize the sidebar at runtime.
    ///
    /// Default: 250.0
    const SIDEBAR_DEFAULT_WIDTH_IN_PIXELS: f32 = 250.0;

    /// Configuration data sent from GUI to simulation.
    ///
    /// This should match the `GuiData` type in your `MultiAgentSimulation` implementation.
    type GuiData: Default + Clone + Sync + Send;

    /// Simulation data received from simulation.
    ///
    /// This should match the `SimulationData` type in your `MultiAgentSimulation` implementation.
    type SimulationData: Default + Clone + Sync + Send;

    /// Messages received from simulation.
    ///
    /// This should match the `MessageToGui` type in your `MultiAgentSimulation` implementation.
    type MessageFromSimulation: Clone;

    /// Messages sent to simulation.
    ///
    /// This should match the `MessageFromGui` type in your `MultiAgentSimulation` implementation.
    type MessageToSimulation: Clone;

    /// Process messages received from the simulation.
    ///
    /// This is called once per frame before rendering, with all messages that have
    /// accumulated since the last frame.
    ///
    /// # Arguments
    /// * `messages` - All messages received from the simulation since the last frame
    fn received_messages_from_simulation(&mut self, messages: Vec<Self::MessageFromSimulation>);

    /// Render the sidebar panel.
    ///
    /// This is called once per frame and should render controls, settings,
    /// and information displays in the left panel.
    ///
    /// # Arguments
    /// * `simulation_data` - Current simulation state (read-only reference)
    /// * `ctx` - egui context for global UI state
    /// * `frame` - eframe frame for window control
    /// * `ui` - UI handle for rendering widgets
    /// * `send_message_to_simulation` - Callback to send messages to the simulation
    ///
    /// # Returns
    /// Return `Some(gui_data)` if the GUI configuration has changed and should be
    /// sent to the simulation. Return `None` if no update is needed.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn sidebar<F>(
    ///     &mut self,
    ///     simulation_data: &GuardArc<Self::SimulationData>,
    ///     ctx: &Context,
    ///     frame: &mut Frame,
    ///     ui: &mut Ui,
    ///     send_message_to_simulation: F,
    /// ) -> Option<Self::GuiData>
    /// where
    ///     F: Fn(Self::MessageToSimulation),
    /// {
    ///     ui.heading("Controls");
    ///
    ///     let old_count = self.agent_count;
    ///     ui.add(egui::Slider::new(&mut self.agent_count, 0..=1000).text("Agents"));
    ///
    ///     if self.agent_count != old_count {
    ///         Some(self.get_config())
    ///     } else {
    ///         None
    ///     }
    /// }
    /// ```
    fn sidebar<F>(
        &mut self,
        simulation_data: &GuardArc<Self::SimulationData>,
        ctx: &Context,
        frame: &mut Frame,
        ui: &mut Ui,
        send_message_to_simulation: F,
    ) -> Option<Self::GuiData>
    where
        F: Fn(Self::MessageToSimulation);

    /// Render the main content panel.
    ///
    /// This is called once per frame and should render the main visualization
    /// of the simulation in the central panel.
    ///
    /// # Arguments
    /// * `simulation_data` - Current simulation state (read-only reference)
    /// * `ctx` - egui context for global UI state
    /// * `frame` - eframe frame for window control
    /// * `ui` - UI handle for rendering widgets
    /// * `send_message_to_simulation` - Callback to send messages to the simulation
    ///
    /// # Example
    /// ```rust,ignore
    /// fn content<F>(
    ///     &mut self,
    ///     simulation_data: &GuardArc<Self::SimulationData>,
    ///     ctx: &Context,
    ///     frame: &mut Frame,
    ///     ui: &mut Ui,
    ///     send_message_to_simulation: F,
    /// ) where
    ///     F: Fn(Self::MessageToSimulation),
    /// {
    ///     let painter = ui.painter();
    ///     for agent in simulation_data.iter() {
    ///         painter.circle_filled(
    ///             egui::Pos2::new(agent.x, agent.y),
    ///             agent.radius,
    ///             egui::Color32::WHITE,
    ///         );
    ///     }
    /// }
    /// ```
    fn content<F>(
        &mut self,
        simulation_data: &GuardArc<Self::SimulationData>,
        ctx: &Context,
        frame: &mut Frame,
        ui: &mut Ui,
        send_message_to_simulation: F,
    ) where
        F: Fn(Self::MessageToSimulation);
}

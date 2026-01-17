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

/// A graphical user interface for visualizing and controlling a multi-agent simulation.
///
/// This trait defines how to render the GUI and handle user interactions. Implementations
/// should focus on presentation logic while the simulation handles the business logic.
///
/// The GUI runs on the main thread at approximately 60 FPS, independently from the
/// simulation thread. It can read simulation data without blocking and send configuration
/// changes or messages to the simulation.
///
/// # Optional Implementation
///
/// This trait is **optional** for workshop students. A default GUI implementation is provided.
/// Students can implement this trait as a bonus exercise to customize the visualization.
///
/// # Type Parameters
///
/// - `GuiData`: Configuration data sent to the simulation (e.g., slider values)
/// - `SimulationData`: Read-only data from the simulation for visualization
/// - `MessageToSimulation`: Discrete events sent to simulation (e.g., button clicks)
/// - `MessageFromSimulation`: Discrete events received from simulation (e.g., notifications)
pub trait MultiAgentGui: Debug + Default {
    /// The application window title displayed in the title bar.
    const APP_NAME: &'static str;

    /// The initial window size in pixels `[width, height]`.
    ///
    /// Default: `[1280.0, 720.0]` (720p resolution)
    const WINDOW_SIZE_IN_PIXELS: [f32; 2] = [1280.0, 720.0];

    /// The minimum window size in pixels `[width, height]`.
    ///
    /// Default: `[896.0, 504.0]` (70% of default size)
    const MIN_WINDOW_SIZE_IN_PIXELS: [f32; 2] = [896.0, 504.0];

    /// The background color in RGBA format `[red, green, blue, alpha]`.
    ///
    /// Default: `[12, 12, 12, 180]` (dark gray with transparency)
    const BACKGROUND_RGBA_COLOR: [u8; 4] = [12, 12, 12, 180];

    /// The default width of the sidebar in pixels.
    ///
    /// Default: `250.0` pixels
    const SIDEBAR_DEFAULT_WIDTH_IN_PIXELS: f32 = 250.0;

    /// Configuration data sent to the simulation.
    ///
    /// Must match the `GuiData` type in the paired `MultiAgentSimulation`.
    type GuiData: Default + Clone + Sync + Send;

    /// Simulation state data for visualization.
    ///
    /// Must match the `SimulationData` type in the paired `MultiAgentSimulation`.
    type SimulationData: Default + Clone + Sync + Send;

    /// Messages received from the simulation.
    ///
    /// Must match the `MessageToGui` type in the paired `MultiAgentSimulation`.
    type MessageFromSimulation: Clone;

    /// Messages sent to the simulation.
    ///
    /// Must match the `MessageFromGui` type in the paired `MultiAgentSimulation`.
    type MessageToSimulation: Clone;

    /// Handles messages received from the simulation thread.
    ///
    /// This method is called once per frame with all messages that have been received
    /// from the simulation since the last frame. Use it to update GUI state based on
    /// simulation events.
    ///
    /// # Arguments
    ///
    /// * `messages` - All messages received from the simulation since the last frame
    fn received_messages_from_simulation(&mut self, messages: Vec<Self::MessageFromSimulation>);

    /// Renders the sidebar panel and returns updated GUI configuration.
    ///
    /// The sidebar typically contains controls (buttons, sliders, checkboxes) for
    /// configuring the simulation. Return `Some(gui_data)` to send updated configuration
    /// to the simulation, or `None` if no changes were made.
    ///
    /// # Arguments
    ///
    /// * `simulation_data` - Read-only access to current simulation state
    /// * `ctx` - egui context for requesting repaints and accessing global state
    /// * `frame` - Frame information (can be used to close the window, etc.)
    /// * `ui` - The UI builder for adding widgets to the sidebar
    /// * `send_message_to_simulation` - Callback to send discrete messages to simulation
    ///
    /// # Returns
    ///
    /// `Some(GuiData)` if configuration changed, `None` otherwise
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

    /// Renders the main content area for visualization.
    ///
    /// This is the central panel where the simulation is visualized. Use egui's
    /// painting API to draw shapes, or add interactive widgets.
    ///
    /// # Arguments
    ///
    /// * `simulation_data` - Read-only access to current simulation state
    /// * `ctx` - egui context for requesting repaints and accessing global state
    /// * `frame` - Frame information
    /// * `ui` - The UI builder for the main content area
    /// * `send_message_to_simulation` - Callback to send discrete messages to simulation
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

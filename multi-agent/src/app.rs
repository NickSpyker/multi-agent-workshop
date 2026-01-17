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

use multi_agent_runtime::MultiAgentRuntimeManager;

/// The main entry point for launching multi-agent simulations.
///
/// `AppLauncher` orchestrates the entire application lifecycle:
/// - Creates separate threads for simulation and GUI
/// - Sets up lock-free communication channels
/// - Manages the simulation loop at the configured frequency
/// - Handles graceful shutdown when the window closes
///
/// # Example
///
/// ```no_run
/// use multi_agent::{
///     AppLauncher,
///     Result, GuardArc,
///     MultiAgentSimulation, MultiAgentGui,
///     egui::{Context, Ui}, eframe::Frame
/// };
/// use std::time::Duration;
///
/// #[derive(Debug, Default)]
/// struct MySimulation;
///
/// impl MultiAgentSimulation for MySimulation {
///     type SimulationData = ();
///     type GuiData = ();
///
///     type MessageFromGui = ();
///     type MessageToGui = ();
///
///     fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
///         Ok(Self)
///     }
///
///     fn update<F>(
///         &mut self,
///         gui_data: Self::GuiData,
///         messages: Vec<Self::MessageFromGui>,
///         delta_time: Duration,
///         send_message_to_gui: F,
///     ) -> Result<&Self::SimulationData> {
///         Ok(&())
///     }
/// }
///
/// #[derive(Debug, Default)]
/// struct MyGui;
///
/// impl MultiAgentGui for MyGui {
///     const APP_NAME: &'static str = "My Gui";
///
///     type GuiData = ();
///     type SimulationData = ();
///
///     type MessageFromSimulation = ();
///     type MessageToSimulation = ();
///
///     fn received_messages_from_simulation(
///         &mut self,
///         messages: Vec<Self::MessageFromSimulation>,
///     ) {}
///
///     fn sidebar<F>(
///         &mut self,
///         simulation_data: &GuardArc<Self::SimulationData>,
///         ctx: &Context,
///         frame: &mut Frame,
///         ui: &mut Ui,
///         send_message_to_simulation: F,
///     ) -> Option<Self::GuiData> {
///         None
///     }
///
///     fn content<F>(
///         &mut self,
///         simulation_data: &GuardArc<Self::SimulationData>,
///         ctx: &Context,
///         frame: &mut Frame,
///         ui: &mut Ui,
///         send_message_to_simulation: F,
///     ) {}
/// }
///
/// fn main() -> Result<()> {
///     AppLauncher::run::<MySimulation, MyGui>()
/// }
/// ```
pub type AppLauncher = MultiAgentRuntimeManager;

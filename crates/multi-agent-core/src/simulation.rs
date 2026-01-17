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

use crate::Result;
use std::{fmt::Debug, time::Duration};

/// A multi-agent simulation that runs at a fixed frequency on a dedicated thread.
///
/// This trait defines the core interface that all simulations must implement. The simulation
/// loop runs independently of the GUI thread, updating at the configured frequency and
/// communicating with the GUI through shared state and message passing.
///
/// # Thread Model
///
/// - The simulation runs on its own thread at `UPDATE_FREQUENCY_HZ` (default: 30 Hz)
/// - The GUI runs on the main thread at approximately 60 FPS
/// - Data flows in both directions without locks using lock-free data structures
///
/// # Type Parameters
///
/// - `SimulationData`: The state produced by the simulation and read by the GUI
/// - `GuiData`: Configuration/parameters sent from GUI to simulation
/// - `MessageFromGui`: Discrete events sent from GUI to simulation (e.g., button clicks)
/// - `MessageToGui`: Discrete events sent from simulation to GUI (e.g., notifications)
pub trait MultiAgentSimulation: Debug + Send + 'static {
    /// The simulation update frequency in Hertz (updates per second).
    ///
    /// Default is 30 Hz, which means the `update()` method will be called approximately
    /// 30 times per second. Override this constant to change the simulation speed.
    const UPDATE_FREQUENCY_HZ: u64 = 30;

    /// The data produced by the simulation and consumed by the GUI.
    ///
    /// This type is returned from `update()` and read by the GUI for visualization.
    /// Must implement `Default + Clone + Sync + Send` for thread-safe sharing.
    type SimulationData: Default + Clone + Sync + Send;

    /// Configuration data sent from the GUI to the simulation.
    ///
    /// This type is typically updated by GUI controls (sliders, checkboxes, etc.)
    /// and read by the simulation to adjust behavior.
    /// Must implement `Default + Clone + Sync + Send` for thread-safe sharing.
    type GuiData: Default + Clone + Sync + Send;

    /// Discrete messages sent from the GUI to the simulation.
    ///
    /// Use this for one-time events like button clicks, rather than continuous
    /// configuration (which should go in `GuiData`).
    type MessageFromGui: Clone + Send + 'static;

    /// Discrete messages sent from the simulation to the GUI.
    ///
    /// Use this for notifications or events that the GUI should react to,
    /// such as simulation completion or error conditions.
    type MessageToGui: Clone + Send + 'static;

    /// Creates a new simulation instance with initial GUI data.
    ///
    /// This method is called once at startup, before the simulation thread begins.
    /// Use it to initialize your simulation state based on the initial GUI configuration.
    ///
    /// # Arguments
    ///
    /// * `initial_gui_data` - The initial configuration from the GUI
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` on successful initialization, or `Err` if initialization fails.
    fn new(initial_gui_data: Self::GuiData) -> Result<Self>
    where
        Self: Sized;

    /// Updates the simulation state for one time step.
    ///
    /// This method is called repeatedly at the frequency specified by `UPDATE_FREQUENCY_HZ`.
    /// It receives the current GUI configuration, any messages from the GUI, and the time
    /// elapsed since the last update.
    ///
    /// # Arguments
    ///
    /// * `gui_data` - Current configuration from the GUI (read-only snapshot)
    /// * `messages` - All messages received from the GUI since the last update
    /// * `delta_time` - Time elapsed since the last update (actual time, may vary slightly)
    /// * `send_message_to_gui` - Callback to send messages to the GUI thread
    ///
    /// # Returns
    ///
    /// Returns a reference to the updated simulation data, which will be cloned and sent
    /// to the GUI thread for visualization. Returning a reference avoids unnecessary copies
    /// within the simulation loop.
    fn update<F>(
        &mut self,
        gui_data: Self::GuiData,
        messages: Vec<Self::MessageFromGui>,
        delta_time: Duration,
        send_message_to_gui: F,
    ) -> Result<&Self::SimulationData>
    where
        F: Fn(Self::MessageToGui);
}

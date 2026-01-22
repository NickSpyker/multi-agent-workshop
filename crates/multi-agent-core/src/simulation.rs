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

/// A multi-agent simulation framework trait.
///
/// Implementations of this trait define the core simulation logic, including agent behavior,
/// physics updates, and message handling. The simulation runs on a dedicated thread at the
/// specified frequency and communicates with the GUI through shared state and message channels.
///
/// # Example
///
/// ```rust,ignore
/// use multi_agent::{MultiAgentSimulation, Result};
/// use std::time::Duration;
/// # #[derive(Clone, Default, Debug)] struct Agent;
/// # impl Agent { fn update(&mut self, _dt: f32) {} }
/// # #[derive(Clone, Default)] struct SimulationConfig;
/// # #[derive(Clone)] enum GuiMessage {}
/// # #[derive(Clone)] enum SimulationMessage {}
///
/// #[derive(Debug)]
/// struct MySimulation {
///     agents: Vec<Agent>,
/// }
///
/// impl MultiAgentSimulation for MySimulation {
///     const FREQUENCY_IN_HZ: u64 = 60;
///
///     type SimulationData = Vec<Agent>;
///     type GuiData = SimulationConfig;
///     type MessageFromGui = GuiMessage;
///     type MessageToGui = SimulationMessage;
///
///     fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
///         Ok(Self {
///             agents: Vec::new(),
///         })
///     }
///
///     fn update<F>(
///         &mut self,
///         gui_data: Self::GuiData,
///         messages: Vec<Self::MessageFromGui>,
///         delta_time: Duration,
///         send_message_to_gui: F,
///     ) -> Result<&Self::SimulationData>
///     where
///         F: Fn(Self::MessageToGui),
///     {
///         // Process messages from GUI
///         for message in messages {
///             // Handle message
///         }
///
///         // Update simulation state
///         let dt = delta_time.as_secs_f32();
///         for agent in &mut self.agents {
///             agent.update(dt);
///         }
///
///         Ok(&self.agents)
///     }
/// }
/// ```
pub trait MultiAgentSimulation: Debug + Send + 'static {
    /// The target update frequency in Hertz (updates per second).
    ///
    /// The runtime will attempt to call `update()` this many times per second.
    /// Common values are 30 Hz for typical simulations or 60 Hz for higher precision.
    ///
    /// Default: 30 Hz (33.3ms per frame)
    const FREQUENCY_IN_HZ: u64 = 30;

    /// Data shared from simulation to GUI.
    ///
    /// This type should be cheap to clone as it will be cloned each frame.
    /// Prefer small structs or types with internal `Arc` for large data.
    ///
    /// # Requirements
    /// - `Default`: Initial state before simulation starts
    /// - `Clone`: Shared via RCU (Read-Copy-Update) pattern
    /// - `Sync + Send`: Thread-safe for cross-thread sharing
    type SimulationData: Default + Clone + Sync + Send;

    /// Configuration data shared from GUI to simulation.
    ///
    /// This type represents parameters that the GUI can modify, such as simulation
    /// speed, agent count, or other runtime configuration.
    ///
    /// Note: `gui_data` is cloned each simulation frame. Keep this type small and
    /// cheap to clone for best performance.
    ///
    /// # Requirements
    /// - `Default`: Initial configuration
    /// - `Clone`: Shared via RCU (Read-Copy-Update) pattern
    /// - `Sync + Send`: Thread-safe for cross-thread sharing
    type GuiData: Default + Clone + Sync + Send;

    /// Messages that can be sent from GUI to simulation.
    ///
    /// Use this to send commands or events from user interactions to the simulation,
    /// such as "pause", "reset", "add agent", etc.
    ///
    /// # Requirements
    /// - `Clone`: Messages may be cloned when sent
    /// - `Send`: Messages cross thread boundaries
    type MessageFromGui: Clone + Send + 'static;

    /// Messages that can be sent from simulation to GUI.
    ///
    /// Use this to notify the GUI of simulation events, such as "agent died",
    /// "milestone reached", etc.
    ///
    /// # Requirements
    /// - `Clone`: Messages may be cloned when sent
    /// - `Send`: Messages cross thread boundaries
    type MessageToGui: Clone + Send + 'static;

    /// Create a new simulation instance with initial GUI data.
    ///
    /// This is called once when the simulation starts, before the first `update()` call.
    ///
    /// # Arguments
    /// * `initial_gui_data` - The initial configuration from the GUI
    ///
    /// # Errors
    /// Return an error if the simulation cannot be initialized with the given configuration.
    fn new(initial_gui_data: Self::GuiData) -> Result<Self>
    where
        Self: Sized;

    /// Update the simulation state for one frame.
    ///
    /// This method is called at the frequency specified by `FREQUENCY_IN_HZ`.
    /// It should update the simulation state and return a reference to the data
    /// that should be displayed in the GUI.
    ///
    /// # Arguments
    /// * `gui_data` - Current configuration from the GUI (cloned each frame)
    /// * `messages` - All messages received from the GUI since the last update
    /// * `delta_time` - Time elapsed since the last update call
    /// * `send_message_to_gui` - Callback to send messages to the GUI
    ///
    /// # Returns
    /// A reference to the simulation data that will be shared with the GUI.
    /// The data will be cloned and stored in shared state.
    ///
    /// # Errors
    /// Return an error if the simulation encounters an unrecoverable error.
    /// This will cause the simulation thread to terminate.
    ///
    /// # Performance Tips
    /// - Message sending is lossy: messages may be dropped if the channel is full
    /// - `gui_data` is cloned every frame, keep it small
    /// - Return `Ok(&self.data)` to avoid extra allocations
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

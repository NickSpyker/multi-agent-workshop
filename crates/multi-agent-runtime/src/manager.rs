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

use multi_agent_core::{Error, MultiAgentGui, MultiAgentSimulation, Result};
use multi_agent_gui::AppGui;
use multi_agent_sync::message::MessageChannel;
use multi_agent_sync::Shared;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

/// Runtime manager that orchestrates the multi-agent simulation and GUI.
///
/// This struct manages the lifecycle of both the simulation and GUI threads,
/// including thread spawning, communication setup, and graceful shutdown.
///
/// # Architecture
///
/// The manager creates two threads:
/// - **Main thread**: Runs the GUI using egui/eframe
/// - **Simulation thread**: Runs the simulation at the specified frequency
///
/// Communication between threads uses:
/// - **Shared state**: Lock-free RCU for `SimulationData` and `GuiData`
/// - **Message channels**: Bounded channels for bidirectional messaging
///
/// # Example
///
/// ```rust,ignore
/// use multi_agent::AppLauncher;
///
/// fn main() -> multi_agent::Result<()> {
///     AppLauncher::run::<MySimulation, MyGui>()
/// }
/// ```
pub struct MultiAgentRuntimeManager;

impl MultiAgentRuntimeManager {
    /// Run the multi-agent application with the given simulation and GUI implementations.
    ///
    /// This method:
    /// 1. Creates shared state for simulation and GUI data
    /// 2. Sets up bidirectional message channels (capacity: 100)
    /// 3. Spawns the simulation thread
    /// 4. Runs the GUI on the main thread
    /// 5. Performs graceful shutdown when the GUI closes
    ///
    /// # Type Parameters
    /// * `Simulation` - Your simulation implementation
    /// * `Gui` - Your GUI implementation
    ///
    /// The types must be compatible (same data and message types).
    ///
    /// # Returns
    /// - `Ok(())` if the application runs and closes successfully
    /// - `Err(Error::SimulationPanic)` if the simulation thread panics
    /// - `Err(Error::ShutdownTimeout)` if the simulation thread doesn't stop within 5 seconds
    /// - `Err(Error::Gui)` if the GUI framework returns an error
    ///
    /// # Example
    /// ```rust,ignore
    /// use multi_agent::AppLauncher;
    ///
    /// fn main() -> multi_agent::Result<()> {
    ///     AppLauncher::run::<MySimulation, MyGui>()
    /// }
    /// ```
    #[inline]
    pub fn run<Simulation, Gui>() -> Result<()>
    where
        Simulation: MultiAgentSimulation,
        Gui: MultiAgentGui<
                GuiData = Simulation::GuiData,
                SimulationData = Simulation::SimulationData,
                MessageFromSimulation = Simulation::MessageToGui,
                MessageToSimulation = Simulation::MessageFromGui,
            >,
        <Simulation as MultiAgentSimulation>::SimulationData: Send,
    {
        let simulation_data = Shared::new(Simulation::SimulationData::default());
        let gui_data = Shared::new(Gui::GuiData::default());

        let (sim_sender, gui_receiver) = MessageChannel::new(100).split();
        let (gui_sender, sim_receiver) = MessageChannel::new(100).split();

        let gui: AppGui<Gui> = AppGui::new(
            gui_sender,
            gui_receiver,
            gui_data.clone(),
            simulation_data.clone(),
        );

        let mut simulation = Simulation::new(Gui::GuiData::default())?;

        let stop_gui = Arc::new(AtomicBool::new(false));
        let stop_simulator = Arc::clone(&stop_gui);

        let simulation_thread = thread::spawn(move || {
            let frequency = Duration::from_millis(1000 / Simulation::FREQUENCY_IN_HZ);

            let mut delta = Instant::now();
            loop {
                if stop_simulator.load(Ordering::Relaxed) {
                    break;
                }

                let now = Instant::now();
                let delta_time = now.duration_since(delta);
                delta = now;

                let new_simulation_data = simulation.update(
                    (**gui_data.load()).clone(),
                    sim_receiver.drain(),
                    delta_time,
                    |message| {
                        let _ = sim_sender.send(message);
                    },
                )?;
                simulation_data.store(new_simulation_data.clone());

                let now = Instant::now();
                let duration = now.duration_since(delta);
                if duration < frequency {
                    thread::sleep(frequency - duration);
                }
            }

            Ok(())
        });

        gui.run()?;
        stop_gui.store(true, Ordering::Relaxed);

        let timeout = Duration::from_secs(5);
        let start = Instant::now();
        loop {
            if simulation_thread.is_finished() {
                return simulation_thread.join().map_err(|e| {
                    Error::SimulationPanic(format!("{:?}", e))
                })?;
            }
            if start.elapsed() >= timeout {
                return Err(Error::ShutdownTimeout { timeout });
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_calculation() {
        // Test that FREQUENCY_IN_HZ correctly translates to duration
        let freq_30hz: u64 = 30;
        let expected_duration_30hz: Duration = Duration::from_millis(1000 / freq_30hz);
        assert_eq!(expected_duration_30hz, Duration::from_millis(33));

        let freq_60hz: u64 = 60;
        let expected_duration_60hz: Duration = Duration::from_millis(1000 / freq_60hz);
        assert_eq!(expected_duration_60hz, Duration::from_millis(16));

        let freq_10hz: u64 = 10;
        let expected_duration_10hz: Duration = Duration::from_millis(1000 / freq_10hz);
        assert_eq!(expected_duration_10hz, Duration::from_millis(100));
    }
}

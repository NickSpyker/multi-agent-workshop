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
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

/// The runtime manager that orchestrates simulation and GUI threads.
///
/// This is the internal implementation used by `AppLauncher`. Most users should
/// use `AppLauncher::run()` instead of this type directly.
pub struct MultiAgentRuntimeManager;

impl MultiAgentRuntimeManager {
    /// Launches the application with the given simulation and GUI implementations.
    ///
    /// This method:
    /// 1. Creates lock-free shared state containers for bidirectional communication
    /// 2. Sets up message channels for discrete events
    /// 3. Spawns the simulation thread running at `Simulation::UPDATE_FREQUENCY_HZ`
    /// 4. Runs the GUI on the main thread at approximately 60 FPS
    /// 5. Waits for graceful shutdown when the window closes
    ///
    /// # Type Parameters
    ///
    /// * `Simulation` - Your simulation implementation
    /// * `Gui` - Your GUI implementation (types must match the simulation's)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Simulation initialization fails in `Simulation::new()`
    /// - The GUI fails to start
    /// - A simulation update returns an error
    /// - The simulation thread panics
    /// - The simulation thread fails to stop within the timeout (5 seconds)
    ///
    /// # Example
    ///
    /// See `AppLauncher` documentation for usage examples.
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
        let simulation_data: Shared<Simulation::SimulationData> =
            Shared::new(Simulation::SimulationData::default());
        let gui_data: Shared<Gui::GuiData> = Shared::new(Gui::GuiData::default());

        let channel_sim_to_gui: MessageChannel<Simulation::MessageToGui> = MessageChannel::new(10);
        let channel_gui_to_sim: MessageChannel<Gui::MessageToSimulation> = MessageChannel::new(10);
        let (sim_sender, gui_receiver) = channel_sim_to_gui.split();
        let (gui_sender, sim_receiver) = channel_gui_to_sim.split();

        let gui: AppGui<Gui> = AppGui::new(
            gui_sender,
            gui_receiver,
            gui_data.clone(),
            simulation_data.clone(),
        );

        let initial_gui_data_input: Gui::GuiData = Gui::GuiData::default();
        let mut simulation: Simulation = Simulation::new(initial_gui_data_input)?;

        let stop_gui: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let stop_simulator: Arc<AtomicBool> = Arc::clone(&stop_gui);

        let simulation_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            let period: Duration =
                Duration::from_secs_f64(1.0 / Simulation::UPDATE_FREQUENCY_HZ as f64);

            let mut delta: Instant = Instant::now();
            loop {
                if stop_simulator.load(Ordering::Relaxed) {
                    break;
                }

                let now: Instant = Instant::now();
                let delta_time: Duration = now.duration_since(delta);
                delta = now;

                let new_simulation_data: &Simulation::SimulationData = simulation.update(
                    (**gui_data.load()).clone(),
                    sim_receiver.drain(),
                    delta_time,
                    |message| sim_sender.send_lossy(message),
                )?;
                simulation_data.store(new_simulation_data.clone());

                let now: Instant = Instant::now();
                let duration: Duration = now.duration_since(delta);
                if duration < period {
                    thread::sleep(period - duration);
                }
            }

            Ok(())
        });

        gui.run()?;
        stop_gui.store(true, Ordering::Relaxed);

        let timeout: Duration = Duration::from_secs(5);
        let start: Instant = Instant::now();
        loop {
            if simulation_thread.is_finished() {
                return simulation_thread.join().map_err(Error::Thread)?;
            }
            if start.elapsed() >= timeout {
                return Err(Error::ThreadStopTimeout);
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

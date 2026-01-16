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
use multi_agent_sync::message::{MessageChannel, MessageReceiver, MessageSender};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct MultiAgentRuntimeManager<Simulation, Gui>
where
    Simulation: MultiAgentSimulation,
    Gui: MultiAgentGui<
            GuiData = Simulation::GuiData,
            SimulationData = Simulation::SimulationData,
            MessageFromSimulation = Simulation::MessageToGui,
            MessageToSimulation = Simulation::MessageFromGui,
        >,
{
    simulation: Simulation,
    gui: AppGui<Gui>,
    sim_sender: MessageSender<Simulation::MessageToGui>,
    sim_receiver: MessageReceiver<Simulation::MessageFromGui>,
}

impl<Simulation, Gui> MultiAgentRuntimeManager<Simulation, Gui>
where
    Simulation: MultiAgentSimulation,
    Gui: MultiAgentGui<
            GuiData = Simulation::GuiData,
            SimulationData = Simulation::SimulationData,
            MessageFromSimulation = Simulation::MessageToGui,
            MessageToSimulation = Simulation::MessageFromGui,
        >,
{
    #[inline]
    pub fn new(message_channel_capacity: usize) -> Result<Self> {
        let channel_sim_to_gui: MessageChannel<Simulation::MessageToGui> =
            MessageChannel::new(message_channel_capacity);

        let channel_gui_to_sim: MessageChannel<Gui::MessageToSimulation> =
            MessageChannel::new(message_channel_capacity);

        let (sim_sender, gui_receiver) = channel_sim_to_gui.split();
        let (gui_sender, sim_receiver) = channel_gui_to_sim.split();

        let gui: AppGui<Gui> = AppGui::new(gui_sender, gui_receiver);

        let initial_gui_data_input: Gui::GuiData = Gui::GuiData::default();
        let simulation: Simulation = Simulation::new(initial_gui_data_input)?;

        Ok(Self {
            gui,
            simulation,
            sim_sender,
            sim_receiver,
        })
    }

    #[inline]
    pub fn run(self) -> Result<()> {
        let Self {
            mut simulation,
            gui,
            sim_sender,
            sim_receiver,
        }: Self = self;

        let stop_gui: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let stop_simulator: Arc<AtomicBool> = Arc::clone(&stop_gui);

        let simulation_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            let frequency: Duration = Duration::from_millis(Simulation::FREQUENCY_IN_HZ);

            let mut delta: Instant = Instant::now();
            loop {
                if stop_simulator.load(Ordering::Relaxed) {
                    break;
                }

                let now: Instant = Instant::now();
                let delta_time: Duration = now.duration_since(delta);
                delta = now;

                let messages: Vec<Simulation::MessageFromGui> = sim_receiver.drain();
                simulation.receive_messages_from_gui(messages)?;

                let simulation_daya: &Simulation::SimulationData =
                    simulation.update(Simulation::GuiData::default(), delta_time, |messages| {
                        for message in messages {
                            sim_sender.send(message)?;
                        }
                        Ok(())
                    })?;

                let now: Instant = Instant::now();
                let duration: Duration = now.duration_since(delta);
                if duration < frequency {
                    thread::sleep(frequency - duration);
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

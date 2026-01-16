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

use multi_agent_core::{MultiAgentGui, MultiAgentSimulation, Result};
use multi_agent_gui::AppGui;

#[derive(Debug)]
pub struct MultiAgentRuntimeManager<Simulation, Interface>
where
    Simulation: MultiAgentSimulation,
    Interface: MultiAgentGui<
            GuiDataOutput = Simulation::GuiDataInput,
            SimulationData = Simulation::SimulationData,
            MessageFromSimulation = Simulation::MessageToGui,
            MessageToSimulation = Simulation::MessageFromGui,
        >,
{
    simulation: Simulation,
    gui: AppGui<Interface>,
}

impl<Simulation, Interface> MultiAgentRuntimeManager<Simulation, Interface>
where
    Simulation: MultiAgentSimulation,
    Interface: MultiAgentGui<
            GuiDataOutput = Simulation::GuiDataInput,
            SimulationData = Simulation::SimulationData,
            MessageFromSimulation = Simulation::MessageToGui,
            MessageToSimulation = Simulation::MessageFromGui,
        >,
{
    #[inline]
    pub fn new() -> Result<Self> {
        let gui: AppGui<Interface> = AppGui::new();

        let initial_gui_data_input: Interface::GuiDataOutput = Interface::GuiDataOutput::default();
        let simulation: Simulation = Simulation::new(initial_gui_data_input)?;

        Ok(Self { gui, simulation })
    }

    #[inline]
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}

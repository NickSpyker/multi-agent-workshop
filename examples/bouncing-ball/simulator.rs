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

use crate::{
    data::{BouncingBallGuiData, BouncingBallSimulationData},
    message::{MessageFromGuiToSimulator, MessageFromSimulatorToGui},
};
use multi_agent::MultiAgentSimulation;
use std::time::Duration;

#[derive(Debug)]
pub struct BouncingBallSimulator {
    data: BouncingBallSimulationData,
}

impl MultiAgentSimulation for BouncingBallSimulator {
    type SimulationData = BouncingBallSimulationData;
    type GuiData = BouncingBallGuiData;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> multi_agent::Result<Self> {
        let data = BouncingBallSimulationData {
            x: initial_gui_data.width / 2.0,
            y: initial_gui_data.height / 2.0,
            dx: 0.0,
            dy: 1.0,
            radius: 10.0,
        };

        Ok(Self { data })
    }

    fn update<F>(
        &mut self,
        gui_data: Self::GuiData,
        message: Vec<Self::MessageFromGui>,
        delta_time: Duration,
        send_message_to_gui: F,
    ) -> multi_agent::Result<&Self::SimulationData>
    where
        F: Fn(Self::MessageToGui) -> multi_agent::Result<()>,
    {
        let data: &mut BouncingBallSimulationData = &mut self.data;

        data.x += data.dx * delta_time.as_secs_f32();
        data.y += data.dy * delta_time.as_secs_f32();

        Ok(&self.data)
    }
}

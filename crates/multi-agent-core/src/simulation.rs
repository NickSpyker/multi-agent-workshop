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

pub trait MultiAgentSimulation: Debug + Default {
    const FREQUENCY_IN_HZ: u64 = 30;

    type GuiDataInput: Default;
    type SimulationData: Default;

    type MessageFromGui;
    type MessageToGui;

    fn new(initial_gui_data_input: Self::GuiDataInput) -> Result<Self>
    where
        Self: Sized;

    fn receive_messages_from_gui(&mut self, messages: Vec<Self::MessageFromGui>) -> Result<()>;

    fn update<F>(
        &mut self,
        gui_data_input: Self::GuiDataInput,
        delta_time: Duration,
        send_to_gui: F,
    ) -> Result<&Self::SimulationData>
    where
        F: Fn(Vec<Self::MessageToGui>) -> Result<()>;
}

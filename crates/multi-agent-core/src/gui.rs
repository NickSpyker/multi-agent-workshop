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

use eframe::{
    egui::{Context, Ui},
    Frame,
};
use std::fmt::Debug;

pub trait MultiAgentGui: Debug + Default {
    const APP_NAME: &'static str;

    const WINDOW_SIZE_IN_PIXELS: [f32; 2] = [1280.0, 720.0];
    const MIN_WINDOW_SIZE_IN_PIXELS: [f32; 2] = [896.0, 504.0];
    const BACKGROUND_RGBA_COLOR: [u8; 4] = [12, 12, 12, 180];
    const SIDEBAR_DEFAULT_WIDTH_IN_PIXELS: f32 = 250.0;

    type GuiData: Default;
    type SimulationData: Default;

    type MessageFromSimulation: Clone;
    type MessageToSimulation: Clone;

    fn received_messages_from_simulation(&mut self, messages: Vec<Self::MessageFromSimulation>);

    fn sidebar<F>(
        &mut self,
        ctx: &Context,
        frame: &mut Frame,
        ui: &mut Ui,
        send_message_to_simulation: F,
    ) where
        F: Fn(Self::MessageToSimulation);

    fn content<F>(
        &mut self,
        ctx: &Context,
        frame: &mut Frame,
        ui: &mut Ui,
        send_message_to_simulation: F,
    ) where
        F: Fn(Self::MessageToSimulation);
}

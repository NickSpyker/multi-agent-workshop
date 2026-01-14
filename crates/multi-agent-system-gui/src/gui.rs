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
    egui::{Color32, Context, ViewportBuilder, Visuals}, App, Frame,
    NativeOptions,
};
use multi_agent_system_core::{Error, Result};

#[derive(Debug, Default)]
pub struct Gui {}

impl Gui {
    pub const APP_NAME: &str = "Multi-Agent System";
    pub const MIN_SIZE: [f32; 2] = [896.0, 504.0];
    pub const MAX_SIZE: [f32; 2] = [5000.0, 5000.0];
    pub const BACKGROUND_COLOR: [u8; 4] = [12, 12, 12, 180];

    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(self) -> Result<()> {
        eframe::run_native(
            Self::APP_NAME,
            NativeOptions {
                viewport: ViewportBuilder::default()
                    .with_min_inner_size(Self::MIN_SIZE)
                    .with_max_inner_size(Self::MAX_SIZE)
                    .with_maximized(true),
                centered: true,
                ..NativeOptions::default()
            },
            Box::new(|_| Ok(Box::new(self))),
        )
        .map_err(Error::Gui)
    }
}

impl App for Gui {
    fn update(&mut self, _ctx: &Context, _frame: &mut Frame) {}

    fn clear_color(&self, _: &Visuals) -> [f32; 4] {
        let [r, g, b, a]: [u8; 4] = Self::BACKGROUND_COLOR;
        Color32::from_rgba_unmultiplied(r, g, b, a).to_normalized_gamma_f32()
    }
}

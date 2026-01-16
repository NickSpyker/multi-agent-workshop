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
    egui::{CentralPanel, Color32, Context, SidePanel, ViewportBuilder, Visuals}, App, Frame,
    NativeOptions,
};
use multi_agent_core::{Error, MultiAgentGui, Result};

#[derive(Debug, Default)]
pub struct AppGui<Interface>
where
    Interface: MultiAgentGui + Default,
{
    inner: Interface,
}

impl<Interface> AppGui<Interface>
where
    Interface: MultiAgentGui + Default,
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn run(self) -> Result<()> {
        eframe::run_native(
            Interface::APP_NAME,
            NativeOptions {
                viewport: ViewportBuilder::default()
                    .with_min_inner_size(Interface::MAX_WINDOW_SIZE_IN_PIXELS)
                    .with_max_inner_size(Interface::MAX_WINDOW_SIZE_IN_PIXELS)
                    .with_maximized(true),
                centered: true,
                ..NativeOptions::default()
            },
            Box::new(|_| Ok(Box::new(self))),
        )
        .map_err(Error::Gui)
    }
}

impl<Interface> App for AppGui<Interface>
where
    Interface: MultiAgentGui + Default,
{
    #[inline]
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        SidePanel::left("multi-agent-gui::Gui.update[sidebar]")
            .default_width(Interface::SIDEBAR_DEFAULT_WIDTH_IN_PIXELS)
            .show(ctx, |ui| {} /*self.inner.sidebar(ctx, frame, ui)*/);

        CentralPanel::default().show(ctx, |ui| {} /*self.inner.content(ctx, frame, ui)*/);
    }

    #[inline]
    fn clear_color(&self, _: &Visuals) -> [f32; 4] {
        let [r, g, b, a]: [u8; 4] = Interface::BACKGROUND_RGBA_COLOR;
        Color32::from_rgba_unmultiplied(r, g, b, a).to_normalized_gamma_f32()
    }
}

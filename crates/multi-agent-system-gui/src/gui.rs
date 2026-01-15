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

use crate::View;
use eframe::{
    egui::{CentralPanel, Color32, ComboBox, Context, SidePanel, ViewportBuilder, Visuals}, App, Frame,
    NativeOptions,
};
use multi_agent_system_core::{Error, Result};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Gui {
    current_view: Option<String>,
    views: HashMap<String, Box<dyn View>>,
}

impl Gui {
    pub const APP_NAME: &str = "Multi-Agent System";
    pub const MIN_SIZE: [f32; 2] = [896.0, 504.0];
    pub const MAX_SIZE: [f32; 2] = [5000.0, 5000.0];
    pub const BACKGROUND_COLOR: [u8; 4] = [12, 12, 12, 180];

    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_view<T: View + 'static>(mut self, view: T) -> Result<Self> {
        let name: String = view.name().to_string();

        if self.views.contains_key(&name) {
            return Err(Error::GuiViewAlreadyExists(name));
        }
        self.views.insert(name, Box::new(view));

        Ok(self)
    }

    #[inline]
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
    #[inline]
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        SidePanel::left("sidebar")
            .default_width(250.0)
            .show(ctx, |ui| {
                ComboBox::from_label("Simulation")
                    .selected_text(self.current_view.clone().unwrap_or("Nothing".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.current_view, None, "Nothing");

                        let mut keys: Vec<&String> = self.views.keys().collect();
                        keys.sort();

                        for key in keys {
                            ui.selectable_value(&mut self.current_view, Some(key.clone()), key);
                        }
                    });

                if let Some(current_view) = &self.current_view {
                    if let Some(view) = self.views.get_mut(current_view) {
                        ui.separator();
                        view.sidebar(ctx, frame, ui);
                    }
                }
            });

        CentralPanel::default().show(ctx, |ui| {
            if let Some(current_view) = &self.current_view {
                if let Some(view) = self.views.get_mut(current_view) {
                    view.content(ctx, frame, ui);
                }
            }
        });
    }

    #[inline]
    fn clear_color(&self, _: &Visuals) -> [f32; 4] {
        let [r, g, b, a]: [u8; 4] = Self::BACKGROUND_COLOR;
        Color32::from_rgba_unmultiplied(r, g, b, a).to_normalized_gamma_f32()
    }
}

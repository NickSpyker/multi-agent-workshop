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
use multi_agent::{
    MultiAgentGui,
    eframe::Frame,
    egui::{
        Color32, Context, Painter, Pos2, Rect, Response, RichText, Slider, Stroke, StrokeKind, Ui,
        Vec2,
    },
};

#[derive(Debug, Default)]
pub struct BouncingBallGui {
    data: BouncingBallGuiData,
    area_size: Vec2,
    paused: bool,
}

impl MultiAgentGui for BouncingBallGui {
    const APP_NAME: &'static str = "Bouncing Ball";

    type GuiData = BouncingBallGuiData;
    type SimulationData = BouncingBallSimulationData;

    type MessageFromSimulation = MessageFromSimulatorToGui;
    type MessageToSimulation = MessageFromGuiToSimulator;

    fn received_messages_from_simulation(&mut self, _messages: Vec<Self::MessageFromSimulation>) {}

    fn sidebar<F>(
        &mut self,
        _ctx: &Context,
        _frame: &mut Frame,
        ui: &mut Ui,
        send_messages_to_simulation: F,
    ) where
        F: Fn(Self::MessageToSimulation),
    {
        if self.area_size == Vec2::ZERO {
            return;
        }

        let Vec2 {
            x: ui_width,
            y: ui_height,
        } = self.area_size;

        ui.heading("Configuration");

        ui.separator();
        ui.heading(RichText::new("Simulation").size(14.0));
        ui.horizontal(|ui| {
            if ui.selectable_label(!self.paused, "Run").clicked() {
                self.paused = false;
                send_messages_to_simulation(MessageFromGuiToSimulator::Resume);
            }
            if ui.selectable_label(self.paused, "Pause").clicked() {
                self.paused = true;
                send_messages_to_simulation(MessageFromGuiToSimulator::Pause);
            }
        });

        ui.separator();
        ui.heading(RichText::new("Area").size(14.0));

        let width_slider: Slider = Slider::new(&mut self.data.width, 0.0..=ui_width)
            .text("width")
            .suffix(" px");
        let width_slider_state: Response = ui.add(width_slider);

        let height_slider: Slider = Slider::new(&mut self.data.height, 0.0..=ui_height)
            .text("height")
            .suffix(" px");
        let height_slider_state: Response = ui.add(height_slider);

        if width_slider_state.changed() || height_slider_state.changed() {
            send_messages_to_simulation(MessageFromGuiToSimulator::RecalculateArea);
        }
    }

    fn content<F>(
        &mut self,
        _ctx: &Context,
        _frame: &mut Frame,
        ui: &mut Ui,
        _send_messages_to_simulation: F,
    ) where
        F: Fn(Self::MessageToSimulation),
    {
        let BouncingBallGuiData { width, height } = self.data;

        let area_rect: Rect = ui.max_rect();
        let area_size: Vec2 = area_rect.size();
        self.area_size = area_size;

        let centered_pos: Pos2 =
            area_rect.min + Vec2::new((area_size.x - width) / 2.0, (area_size.y - height) / 2.0);

        let painter: &Painter = ui.painter();

        painter.rect_stroke(
            Rect::from_min_size(centered_pos, Vec2::new(width, height)),
            0.0,
            Stroke::new(2.0, Color32::WHITE),
            StrokeKind::Outside,
        );

        painter.circle_filled(Pos2::new(0.0, 0.0), 10.0, Color32::RED);
    }
}

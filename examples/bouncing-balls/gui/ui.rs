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

use super::{BouncingAreaConfig, MessageFromGuiToSimulator};
use crate::simulation::{BouncingBall, MessageFromSimulatorToGui};
use multi_agent::{
    eframe::Frame, egui::{
        Color32, Context, Painter, Pos2, Rect, Response, RichText, Slider, Stroke, StrokeKind, Ui,
        Vec2,
    },
    GuardArc,
    MultiAgentGui,
};

#[derive(Debug, Default)]
pub struct BouncingBallsGui {
    area_config: BouncingAreaConfig,
    area_max_size: Vec2,
    paused: bool,
}

impl MultiAgentGui for BouncingBallsGui {
    const APP_NAME: &'static str = "Bouncing Ball";

    type GuiData = BouncingAreaConfig;
    type SimulationData = Vec<BouncingBall>;

    type MessageFromSimulation = MessageFromSimulatorToGui;
    type MessageToSimulation = MessageFromGuiToSimulator;

    fn received_messages_from_simulation(&mut self, _messages: Vec<Self::MessageFromSimulation>) {}

    fn sidebar<F>(
        &mut self,
        _simulation_data: &GuardArc<Self::SimulationData>,
        _ctx: &Context,
        _frame: &mut Frame,
        ui: &mut Ui,
        send_message_to_simulation: F,
    ) -> Option<Self::GuiData>
    where
        F: Fn(Self::MessageToSimulation),
    {
        if self.area_max_size == Vec2::ZERO {
            return None;
        }

        let Vec2 {
            x: ui_width,
            y: ui_height,
        } = self.area_max_size;

        ui.heading("Interaction");
        if ui.button("shake").clicked() {
            send_message_to_simulation(MessageFromGuiToSimulator::Shake);
        }

        ui.separator();
        ui.heading("Configuration");

        ui.heading(RichText::new("Simulation").size(14.0));
        ui.horizontal(|ui| {
            if ui.selectable_label(!self.paused, "Run").clicked() {
                self.paused = false;
                send_message_to_simulation(MessageFromGuiToSimulator::Resume);
            }
            if ui.selectable_label(self.paused, "Pause").clicked() {
                self.paused = true;
                send_message_to_simulation(MessageFromGuiToSimulator::Pause);
            }
        });

        ui.heading(RichText::new("Balls").size(14.0));
        let old_ball_count: usize = self.area_config.ball_count;
        let ball_count_slider: Slider =
            Slider::new(&mut self.area_config.ball_count, 0..=1_000).text("count");
        let ball_count_state: Response = ui.add(ball_count_slider);
        if ball_count_state.changed() {
            let current_ball_count: usize = self.area_config.ball_count;
            if current_ball_count < old_ball_count {
                send_message_to_simulation(MessageFromGuiToSimulator::RemoveBalls(
                    old_ball_count - current_ball_count,
                ));
            } else if current_ball_count > old_ball_count {
                send_message_to_simulation(MessageFromGuiToSimulator::AddBalls(
                    current_ball_count - old_ball_count,
                ));
            }
        }

        ui.heading(RichText::new("Area").size(14.0));

        let width_slider: Slider = Slider::new(&mut self.area_config.width, 0.0..=ui_width)
            .text("width")
            .suffix(" px");
        let width_slider_state: Response = ui.add(width_slider);

        let height_slider: Slider = Slider::new(&mut self.area_config.height, 0.0..=ui_height)
            .text("height")
            .suffix(" px");
        let height_slider_state: Response = ui.add(height_slider);

        if width_slider_state.changed() || height_slider_state.changed() {
            send_message_to_simulation(MessageFromGuiToSimulator::RecalculateArea);
        }

        Some(self.area_config.clone())
    }

    fn content<F>(
        &mut self,
        simulation_data: &GuardArc<Self::SimulationData>,
        _ctx: &Context,
        _frame: &mut Frame,
        ui: &mut Ui,
        _send_message_to_simulation: F,
    ) where
        F: Fn(Self::MessageToSimulation),
    {
        let (width, height): (f32, f32) = (self.area_config.width, self.area_config.height);

        let area_rect: Rect = ui.max_rect();
        let area_size: Vec2 = area_rect.size();
        let centered_pos: Pos2 =
            area_rect.min + Vec2::new((area_size.x - width) / 2.0, (area_size.y - height) / 2.0);
        self.area_max_size = area_size;

        let rect: Rect = Rect::from_min_size(centered_pos, Vec2::new(width, height));
        let min: Pos2 = rect.min;

        let painter: &Painter = ui.painter();
        painter.rect_stroke(
            Rect::from_min_size(centered_pos, Vec2::new(width, height)),
            0.0,
            Stroke::new(2.0, Color32::WHITE),
            StrokeKind::Outside,
        );
        for ball in simulation_data.iter() {
            painter.circle_filled(
                Pos2::new(ball.x + min.x, ball.y + min.y),
                ball.radius,
                Color32::RED,
            );
        }
    }
}

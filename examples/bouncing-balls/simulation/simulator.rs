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

use super::{BouncingBall, MessageFromSimulatorToGui};
use crate::gui::{BouncingAreaConfig, MessageFromGuiToSimulator};
use multi_agent::MultiAgentSimulation;
use rand::{Rng, rngs::ThreadRng};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct BouncingBallsSimulator {
    balls: Vec<BouncingBall>,
    paused: bool,
}

impl BouncingBallsSimulator {
    const DELTA_TIME_SCALING: f32 = 50.0;
    const GRAVITY: f32 = 9.81;
    const BOUNCE_DAMPING: f32 = 0.90;
    const RADIUS_RANGE: [f32; 2] = [5.0, 10.0];

    fn add_balls(&mut self, count: usize, max_x: f32, max_y: f32) {
        let mut rng: ThreadRng = rand::rng();

        for _ in 0..count {
            let radius: f32 = rng.random_range(Self::RADIUS_RANGE[0]..Self::RADIUS_RANGE[1]);

            self.balls.push(BouncingBall {
                x: rng.random_range(0.0..max_x - radius),
                y: rng.random_range(0.0..max_y - radius),
                dx: rng.random_range(-5.0..5.0),
                dy: rng.random_range(-5.0..5.0),
                radius,
                color: [
                    rng.random_range(0..255),
                    rng.random_range(0..255),
                    rng.random_range(0..255),
                ],
            });
        }
    }

    fn remove_balls(&mut self, count: usize) {
        if count >= self.balls.len() {
            self.balls.clear();
        } else {
            self.balls.truncate(self.balls.len() - count);
        }
    }

    fn shake(&mut self) {
        let mut rng: ThreadRng = rand::rng();
        for ball in self.balls.iter_mut() {
            ball.dx = rng.random_range(-50.0..50.0);
            ball.dy = rng.random_range(-50.0..-10.0);
        }
    }

    fn apply_gravity(&mut self, delta_time: f32) {
        for ball in self.balls.iter_mut() {
            ball.dy += Self::GRAVITY * delta_time;
        }
    }

    fn move_balls(&mut self, mut delta_time: f32) {
        delta_time *= Self::DELTA_TIME_SCALING;
        for ball in self.balls.iter_mut() {
            ball.x += ball.dx * delta_time;
            ball.y += ball.dy * delta_time;
        }
    }

    fn bounce_balls(&mut self, width: f32, height: f32) {
        for BouncingBall {
            x,
            y,
            dx,
            dy,
            radius,
            color: _,
        } in self.balls.iter_mut()
        {
            let is_in_area_x: bool = *radius <= *x && *x <= width - *radius;
            let is_in_area_y: bool = *radius <= *y && *y <= height - *radius;

            if is_in_area_x && is_in_area_y {
                continue;
            }

            let bounce_damping: f32 = Self::BOUNCE_DAMPING / (*radius / Self::RADIUS_RANGE[0]);

            if *x - *radius < 0.0 {
                *x = *radius;
                *dx = -*dx * bounce_damping;
            } else if *x + *radius > width {
                *x = width - *radius;
                *dx = -*dx * bounce_damping;
            }

            if *y - *radius < 0.0 {
                *y = *radius;
                *dy = -*dy * bounce_damping;
            } else if *y + *radius > height {
                *y = height - *radius;
                *dy = -*dy * bounce_damping;
                if dy.abs() < 0.5 {
                    *dy = 0.0;
                }
            }
        }
    }
}

impl MultiAgentSimulation for BouncingBallsSimulator {
    type SimulationData = Vec<BouncingBall>;
    type GuiData = BouncingAreaConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> multi_agent::Result<Self> {
        let (max_x, max_y): (f32, f32) = (initial_gui_data.width, initial_gui_data.height);

        let mut simulator: Self = Self::default();
        simulator.add_balls(initial_gui_data.ball_count, max_x, max_y);

        Ok(simulator)
    }

    fn update<F>(
        &mut self,
        gui_data: Self::GuiData,
        messages: Vec<Self::MessageFromGui>,
        delta_time: Duration,
        _send_message_to_gui: F,
    ) -> multi_agent::Result<&Self::SimulationData>
    where
        F: Fn(Self::MessageToGui),
    {
        let (width, height): (f32, f32) = (gui_data.width, gui_data.height);

        for message in messages {
            match message {
                MessageFromGuiToSimulator::Pause => self.paused = true,
                MessageFromGuiToSimulator::Resume => self.paused = false,
                MessageFromGuiToSimulator::RecalculateArea => self.bounce_balls(width, height),
                MessageFromGuiToSimulator::Shake => self.shake(),
                MessageFromGuiToSimulator::AddBalls(count) => self.add_balls(count, width, height),
                MessageFromGuiToSimulator::RemoveBalls(count) => self.remove_balls(count),
            }
        }

        if !self.paused {
            let dt: f32 = delta_time.as_secs_f32();
            self.apply_gravity(dt);
            self.move_balls(dt);
            self.bounce_balls(width, height);
        }

        Ok(&self.balls)
    }
}

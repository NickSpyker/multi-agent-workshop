use super::{MessageFromSimulatorToGui, Physarum};
use crate::gui::{MessageFromGuiToSimulator, PhysarumConfig};
use fastrand::Rng;
use multi_agent::{MultiAgentSimulation, Result};
use std::time::Duration;

#[derive(Debug)]
pub struct PhysarumSimulator {
    data: Physarum,
    rng: Rng,
}

impl MultiAgentSimulation for PhysarumSimulator {
    type SimulationData = Physarum;
    type GuiData = PhysarumConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
        let mut data = Physarum::new(
            initial_gui_data.width,
            initial_gui_data.height,
        );
        data.spawn_agents(initial_gui_data.agent_count, initial_gui_data.spawn_mode);

        Ok(Self {
            data,
            rng: Rng::new(),
        })
    }

    fn update<F>(
        &mut self,
        gui_data: Self::GuiData,
        messages: Vec<Self::MessageFromGui>,
        delta_time: Duration,
        _send_message_to_gui: F,
    ) -> Result<&Self::SimulationData> {
        // Handle messages
        for message in messages {
            match message {
                MessageFromGuiToSimulator::Reset => {
                    self.data.clear();
                    self.data.spawn_agents(gui_data.agent_count, gui_data.spawn_mode);
                }
                MessageFromGuiToSimulator::SetAgentCount(count) => {
                    self.data.set_agent_count(count, gui_data.spawn_mode);
                }
                MessageFromGuiToSimulator::ResizeWorld(width, height) => {
                    self.data.resize(width, height);
                }
                MessageFromGuiToSimulator::ClearTrails => {
                    self.data.trail_map.clear();
                }
            }
        }

        // Run simulation if not paused
        if !gui_data.paused {
            let dt = delta_time.as_secs_f32();

            // Run multiple steps per frame for smoother simulation
            for _ in 0..gui_data.steps_per_frame {
                self.process_agents(&gui_data, dt / gui_data.steps_per_frame as f32);
            }

            // Diffuse and decay the trail map
            self.data.trail_map.diffuse_and_decay(
                gui_data.diffuse_rate,
                gui_data.decay_rate,
                dt,
            );
        }

        Ok(&self.data)
    }
}

impl PhysarumSimulator {
    fn process_agents(&mut self, config: &PhysarumConfig, dt: f32) {
        let width = self.data.width as f32;
        let height = self.data.height as f32;

        // Convert angles to radians
        let sensor_angle_rad = config.sensor_angle.to_radians();
        let turn_speed_rad = config.turn_speed.to_radians();

        for agent in &mut self.data.agents {
            // === SENSE ===
            // Sample trail map at three sensor positions (left, forward, right)
            let sensor_left_angle = agent.angle + sensor_angle_rad;
            let sensor_forward_angle = agent.angle;
            let sensor_right_angle = agent.angle - sensor_angle_rad;

            let sensor_left_x = agent.x + sensor_left_angle.cos() * config.sensor_offset;
            let sensor_left_y = agent.y + sensor_left_angle.sin() * config.sensor_offset;

            let sensor_forward_x = agent.x + sensor_forward_angle.cos() * config.sensor_offset;
            let sensor_forward_y = agent.y + sensor_forward_angle.sin() * config.sensor_offset;

            let sensor_right_x = agent.x + sensor_right_angle.cos() * config.sensor_offset;
            let sensor_right_y = agent.y + sensor_right_angle.sin() * config.sensor_offset;

            let weight_left = self.data.trail_map.sample(
                sensor_left_x,
                sensor_left_y,
                config.sensor_size,
            );
            let weight_forward = self.data.trail_map.sample(
                sensor_forward_x,
                sensor_forward_y,
                config.sensor_size,
            );
            let weight_right = self.data.trail_map.sample(
                sensor_right_x,
                sensor_right_y,
                config.sensor_size,
            );

            // === TURN ===
            // Decide which direction to turn based on sensor readings
            let random_steer = self.rng.f32();

            if weight_forward > weight_left && weight_forward > weight_right {
                // Continue forward (no turn)
            } else if weight_forward < weight_left && weight_forward < weight_right {
                // Both sides are better than forward, turn randomly
                if random_steer < 0.5 {
                    agent.angle += turn_speed_rad * dt;
                } else {
                    agent.angle -= turn_speed_rad * dt;
                }
            } else if weight_right > weight_left {
                // Turn right
                agent.angle -= turn_speed_rad * dt;
            } else if weight_left > weight_right {
                // Turn left
                agent.angle += turn_speed_rad * dt;
            }
            // If weights are equal, continue forward

            // === MOVE ===
            let new_x = agent.x + agent.angle.cos() * config.move_speed * dt;
            let new_y = agent.y + agent.angle.sin() * config.move_speed * dt;

            // Handle boundary conditions (wrap around or bounce)
            if config.wrap_edges {
                // Toroidal wrapping
                agent.x = if new_x < 0.0 {
                    new_x + width
                } else if new_x >= width {
                    new_x - width
                } else {
                    new_x
                };

                agent.y = if new_y < 0.0 {
                    new_y + height
                } else if new_y >= height {
                    new_y - height
                } else {
                    new_y
                };
            } else {
                // Bounce off edges
                if new_x < 0.0 || new_x >= width {
                    agent.angle = std::f32::consts::PI - agent.angle;
                    agent.x = agent.x.clamp(0.0, width - 1.0);
                } else {
                    agent.x = new_x;
                }

                if new_y < 0.0 || new_y >= height {
                    agent.angle = -agent.angle;
                    agent.y = agent.y.clamp(0.0, height - 1.0);
                } else {
                    agent.y = new_y;
                }
            }

            // === DEPOSIT ===
            // Leave a trail at the current position
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                let px = agent.x as usize;
                let py = agent.y as usize;
                self.data.trail_map.add(px, py, config.deposit_amount * dt);
            }
        }
    }
}

use super::{Boids, MessageFromSimulatorToGui, Vec2};
use crate::gui::{BoidsConfig, MessageFromGuiToSimulator};
use multi_agent::{MultiAgentSimulation, Result};
use std::time::Duration;

#[derive(Debug)]
pub struct BoidsSimulator {
    data: Boids,
    accumulated_time: Duration,
}

impl BoidsSimulator {
    fn process_tick(&mut self, config: &BoidsConfig) {
        let dt = 1.0 / config.tick_rate_per_second;
        let boids_count = self.data.boids.len();

        if boids_count == 0 {
            return;
        }

        // Convert FOV to radians for half-angle comparison
        let half_fov_rad = (config.field_of_view / 2.0).to_radians();
        let cos_half_fov = half_fov_rad.cos();

        // Precompute squared radii for efficiency
        let sep_radius_sq = config.separation_radius * config.separation_radius;
        let ali_radius_sq = config.alignment_radius * config.alignment_radius;
        let coh_radius_sq = config.cohesion_radius * config.cohesion_radius;

        // Calculate accelerations for each boid
        let accelerations: Vec<Vec2> = (0..boids_count)
            .map(|i| {
                let boid = &self.data.boids[i];
                let boid_dir = boid.velocity.normalized();

                let mut separation = Vec2::ZERO;
                let mut alignment = Vec2::ZERO;
                let mut cohesion = Vec2::ZERO;

                let mut sep_count = 0;
                let mut ali_count = 0;
                let mut coh_count = 0;

                for (j, other) in self.data.boids.iter().enumerate() {
                    if i == j {
                        continue;
                    }

                    let offset = other.position - boid.position;
                    let dist_sq = offset.length_squared();

                    // Skip if too far for any behavior
                    if dist_sq > coh_radius_sq {
                        continue;
                    }

                    // Check if within field of view
                    let in_fov = if boid.velocity.length_squared() > 0.0001 {
                        let offset_dir = offset.normalized();
                        boid_dir.dot(offset_dir) >= cos_half_fov
                    } else {
                        true // If not moving, see all around
                    };

                    if !in_fov {
                        continue;
                    }

                    // Separation: steer away from nearby boids
                    if dist_sq < sep_radius_sq && dist_sq > 0.0001 {
                        let dist = dist_sq.sqrt();
                        // Weight inversely by distance
                        separation = separation - offset.normalized() * (1.0 - dist / config.separation_radius);
                        sep_count += 1;
                    }

                    // Alignment: match velocity of nearby boids
                    if dist_sq < ali_radius_sq {
                        alignment = alignment + other.velocity;
                        ali_count += 1;
                    }

                    // Cohesion: steer toward center of nearby boids
                    if dist_sq < coh_radius_sq {
                        cohesion = cohesion + other.position;
                        coh_count += 1;
                    }
                }

                // Calculate steering forces
                let mut acceleration = Vec2::ZERO;

                // Separation
                if sep_count > 0 {
                    acceleration = acceleration + separation.normalized() * config.separation_weight;
                }

                // Alignment
                if ali_count > 0 {
                    let avg_velocity = alignment / ali_count as f32;
                    let desired = avg_velocity.normalized() * config.max_speed;
                    let steer = desired - boid.velocity;
                    acceleration = acceleration + steer.normalized() * config.alignment_weight;
                }

                // Cohesion
                if coh_count > 0 {
                    let center_of_mass = cohesion / coh_count as f32;
                    let desired = (center_of_mass - boid.position).normalized() * config.max_speed;
                    let steer = desired - boid.velocity;
                    acceleration = acceleration + steer.normalized() * config.cohesion_weight;
                }

                acceleration
            })
            .collect();

        // Apply accelerations and update positions
        let width = self.data.width;
        let height = self.data.height;

        for (boid, acc) in self.data.boids.iter_mut().zip(accelerations.iter()) {
            // Update velocity
            boid.velocity = boid.velocity + *acc * dt * 100.0;

            // Clamp speed
            let speed = boid.velocity.length();
            if speed > config.max_speed {
                boid.velocity = boid.velocity.normalized() * config.max_speed;
            } else if speed < config.min_speed && speed > 0.0001 {
                boid.velocity = boid.velocity.normalized() * config.min_speed;
            }

            // Update position
            boid.position = boid.position + boid.velocity * dt;

            // Wrap around edges (toroidal world)
            if boid.position.x < 0.0 {
                boid.position.x += width;
            } else if boid.position.x >= width {
                boid.position.x -= width;
            }

            if boid.position.y < 0.0 {
                boid.position.y += height;
            } else if boid.position.y >= height {
                boid.position.y -= height;
            }
        }
    }
}

impl MultiAgentSimulation for BoidsSimulator {
    type SimulationData = Boids;
    type GuiData = BoidsConfig;

    type MessageFromGui = MessageFromGuiToSimulator;
    type MessageToGui = MessageFromSimulatorToGui;

    fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
        let mut data = Boids::default();
        data.spawn_random(initial_gui_data.boid_count, initial_gui_data.max_speed);

        Ok(Self {
            data,
            accumulated_time: Duration::ZERO,
        })
    }

    fn update<F>(
        &mut self,
        gui_data: Self::GuiData,
        messages: Vec<Self::MessageFromGui>,
        delta_time: Duration,
        _send_message_to_gui: F,
    ) -> Result<&Self::SimulationData> {
        // Process messages from GUI
        for message in messages {
            match message {
                MessageFromGuiToSimulator::Reset => {
                    self.data.clear();
                    self.data.spawn_random(gui_data.boid_count, gui_data.max_speed);
                }
                MessageFromGuiToSimulator::SpawnBoids(count) => {
                    self.data.spawn_random(count, gui_data.max_speed);
                }
                MessageFromGuiToSimulator::ResizeWorld(width, height) => {
                    self.data.resize(width, height);
                }
            }
        }

        // Update simulation with fixed timestep
        if !gui_data.paused && gui_data.tick_rate_per_second > 0.0 {
            let tick_duration = Duration::from_secs_f32(1.0 / gui_data.tick_rate_per_second);
            self.accumulated_time += delta_time;

            while self.accumulated_time >= tick_duration {
                self.process_tick(&gui_data);
                self.accumulated_time -= tick_duration;
            }
        }

        Ok(&self.data)
    }
}

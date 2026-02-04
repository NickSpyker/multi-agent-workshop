use super::{MessageFromGuiToSimulator, PhysarumConfig};
use crate::simulation::{MessageFromSimulatorToGui, Physarum, SpawnMode};
use eframe::Frame;
use egui::{Color32, ColorImage, Context, ScrollArea, Sense, TextureHandle, TextureOptions, Ui};
use multi_agent::{GuardArc, MultiAgentGui};

pub struct PhysarumGui {
    config: PhysarumConfig,
    last_world_size: (usize, usize),
    texture: Option<TextureHandle>,
}

impl std::fmt::Debug for PhysarumGui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhysarumGui")
            .field("config", &self.config)
            .field("last_world_size", &self.last_world_size)
            .field("texture", &self.texture.as_ref().map(|_| "TextureHandle"))
            .finish()
    }
}

impl Default for PhysarumGui {
    fn default() -> Self {
        Self {
            config: PhysarumConfig::default(),
            last_world_size: (0, 0),
            texture: None,
        }
    }
}

impl MultiAgentGui for PhysarumGui {
    const APP_NAME: &'static str = "Physarum";

    type GuiData = PhysarumConfig;
    type SimulationData = Physarum;

    type MessageFromSimulation = MessageFromSimulatorToGui;
    type MessageToSimulation = MessageFromGuiToSimulator;

    fn received_messages_from_simulation(&mut self, _messages: Vec<Self::MessageFromSimulation>) {}

    fn sidebar<F>(
        &mut self,
        _simulation_data: &GuardArc<Self::SimulationData>,
        _ctx: &Context,
        _frame: &mut Frame,
        ui: &mut Ui,
        mut send_message_to_simulation: F,
    ) -> Option<Self::GuiData>
    where
        F: FnMut(Self::MessageToSimulation),
    {
        let mut config_changed = false;

        ScrollArea::vertical().show(ui, |ui| {
            // === CONTROLS ===
            ui.heading("Controls");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui
                    .button(if self.config.paused {
                        "Play"
                    } else {
                        "Pause"
                    })
                    .clicked()
                {
                    self.config.paused = !self.config.paused;
                    config_changed = true;
                }

                if ui.button("Reset").clicked() {
                    send_message_to_simulation(MessageFromGuiToSimulator::Reset);
                }

                if ui.button("Clear Trails").clicked() {
                    send_message_to_simulation(MessageFromGuiToSimulator::ClearTrails);
                }
            });

            ui.add_space(5.0);

            ui.label("Steps per frame:");
            if ui
                .add(egui::Slider::new(&mut self.config.steps_per_frame, 1..=10))
                .changed()
            {
                config_changed = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // === AGENTS ===
            ui.heading("Agents");

            ui.label("Agent count:");
            if ui
                .add(egui::Slider::new(&mut self.config.agent_count, 100..=50000).logarithmic(true))
                .changed()
            {
                config_changed = true;
                send_message_to_simulation(MessageFromGuiToSimulator::SetAgentCount(
                    self.config.agent_count,
                ));
            }

            ui.label("Spawn mode:");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.config.spawn_mode == SpawnMode::Random, "Random")
                    .clicked()
                {
                    self.config.spawn_mode = SpawnMode::Random;
                    config_changed = true;
                }
                if ui
                    .selectable_label(self.config.spawn_mode == SpawnMode::Center, "Center")
                    .clicked()
                {
                    self.config.spawn_mode = SpawnMode::Center;
                    config_changed = true;
                }
                if ui
                    .selectable_label(self.config.spawn_mode == SpawnMode::Circle, "Circle")
                    .clicked()
                {
                    self.config.spawn_mode = SpawnMode::Circle;
                    config_changed = true;
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // === MOVEMENT ===
            ui.heading("Movement");

            ui.label("Move speed:");
            if ui
                .add(egui::Slider::new(&mut self.config.move_speed, 10.0..=500.0))
                .changed()
            {
                config_changed = true;
            }

            ui.label("Turn speed (deg/s):");
            if ui
                .add(egui::Slider::new(&mut self.config.turn_speed, 10.0..=720.0))
                .changed()
            {
                config_changed = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // === SENSORS ===
            ui.heading("Sensors");

            ui.label("Sensor angle (deg):");
            if ui
                .add(egui::Slider::new(&mut self.config.sensor_angle, 5.0..=90.0))
                .changed()
            {
                config_changed = true;
            }

            ui.label("Sensor distance:");
            if ui
                .add(egui::Slider::new(&mut self.config.sensor_offset, 1.0..=50.0))
                .changed()
            {
                config_changed = true;
            }

            ui.label("Sensor size:");
            if ui
                .add(egui::Slider::new(&mut self.config.sensor_size, 0..=5))
                .changed()
            {
                config_changed = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // === TRAIL ===
            ui.heading("Trail");

            ui.label("Deposit amount:");
            if ui
                .add(egui::Slider::new(&mut self.config.deposit_amount, 0.1..=20.0))
                .changed()
            {
                config_changed = true;
            }

            ui.label("Diffuse rate:");
            if ui
                .add(egui::Slider::new(&mut self.config.diffuse_rate, 0.0..=10.0))
                .changed()
            {
                config_changed = true;
            }

            ui.label("Decay rate:");
            if ui
                .add(egui::Slider::new(&mut self.config.decay_rate, 0.0..=5.0))
                .changed()
            {
                config_changed = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // === BOUNDARY ===
            ui.heading("Boundary");

            if ui
                .checkbox(&mut self.config.wrap_edges, "Wrap edges")
                .changed()
            {
                config_changed = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // === VISUAL ===
            ui.heading("Visual");

            if ui
                .checkbox(&mut self.config.show_agents, "Show agents")
                .changed()
            {
                config_changed = true;
            }

            ui.label("Trail color:");
            let mut color = self.config.trail_color;
            if ui.color_edit_button_rgb(&mut color).changed() {
                self.config.trail_color = color;
                config_changed = true;
            }
        });

        if config_changed {
            Some(self.config.clone())
        } else {
            None
        }
    }

    fn content<F>(
        &mut self,
        simulation_data: &GuardArc<Self::SimulationData>,
        ctx: &Context,
        _frame: &mut Frame,
        ui: &mut Ui,
        mut send_message_to_simulation: F,
    ) where
        F: FnMut(Self::MessageToSimulation),
    {
        let available_rect = ui.available_rect_before_wrap();
        let _response = ui.allocate_rect(available_rect, Sense::click_and_drag());

        // Handle world resize
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let new_size = (available_rect.width() as usize, available_rect.height() as usize);

        if new_size.0 != self.last_world_size.0 || new_size.1 != self.last_world_size.1 {
            if new_size.0 > 0 && new_size.1 > 0 {
                self.last_world_size = new_size;
                self.config.width = new_size.0;
                self.config.height = new_size.1;
                send_message_to_simulation(MessageFromGuiToSimulator::ResizeWorld(
                    new_size.0, new_size.1,
                ));
                // Invalidate texture
                self.texture = None;
            }
        }

        // Create image from trail map
        let width = simulation_data.trail_map.width;
        let height = simulation_data.trail_map.height;

        if width == 0 || height == 0 {
            return;
        }

        let mut pixels = Vec::with_capacity(width * height);
        let trail_color = self.config.trail_color;

        for y in 0..height {
            for x in 0..width {
                let value = simulation_data.trail_map.get(x, y);

                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let r = (value * trail_color[0] * 255.0).min(255.0) as u8;
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let g = (value * trail_color[1] * 255.0).min(255.0) as u8;
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let b = (value * trail_color[2] * 255.0).min(255.0) as u8;

                pixels.push(Color32::from_rgb(r, g, b));
            }
        }

        let image = ColorImage::from_rgba_unmultiplied(
            [width, height],
            &pixels
                .iter()
                .flat_map(|c| c.to_array())
                .collect::<Vec<u8>>(),
        );

        // Update or create texture
        let texture = self.texture.get_or_insert_with(|| {
            ctx.load_texture("trail_map", image.clone(), TextureOptions::NEAREST)
        });
        texture.set(image, TextureOptions::NEAREST);

        // Draw the texture
        let painter = ui.painter_at(available_rect);

        painter.image(
            texture.id(),
            available_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        // Optionally draw agents
        if self.config.show_agents {
            let scale_x = available_rect.width() / width as f32;
            let scale_y = available_rect.height() / height as f32;

            for agent in &simulation_data.agents {
                let screen_x = available_rect.left() + agent.x * scale_x;
                let screen_y = available_rect.top() + agent.y * scale_y;

                painter.circle_filled(
                    egui::pos2(screen_x, screen_y),
                    1.5,
                    Color32::WHITE,
                );
            }
        }
    }
}

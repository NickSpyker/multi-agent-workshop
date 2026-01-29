use super::{BoidsConfig, MessageFromGuiToSimulator};
use crate::simulation::{Boids, MessageFromSimulatorToGui, Vec2};
use eframe::Frame;
use egui::{Color32, Context, Pos2, Rect, Sense, Stroke, Ui};
use multi_agent::{GuardArc, MultiAgentGui};

#[derive(Debug)]
pub struct BoidsGui {
    config: BoidsConfig,
    last_world_size: (f32, f32),
}

impl Default for BoidsGui {
    fn default() -> Self {
        Self {
            config: BoidsConfig::default(),
            last_world_size: (0.0, 0.0),
        }
    }
}

impl MultiAgentGui for BoidsGui {
    const APP_NAME: &'static str = "Boids";

    type GuiData = BoidsConfig;
    type SimulationData = Boids;

    type MessageFromSimulation = MessageFromSimulatorToGui;
    type MessageToSimulation = MessageFromGuiToSimulator;

    fn received_messages_from_simulation(&mut self, _messages: Vec<Self::MessageFromSimulation>) {}

    fn sidebar<F>(
        &mut self,
        simulation_data: &GuardArc<Self::SimulationData>,
        _ctx: &Context,
        _frame: &mut Frame,
        ui: &mut Ui,
        mut send_message_to_simulation: F,
    ) -> Option<Self::GuiData>
    where
        F: FnMut(Self::MessageToSimulation),
    {
        let mut config_changed = false;

        ui.heading("Controls");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui
                .button(if self.config.paused {
                    "▶ Play"
                } else {
                    "⏸ Pause"
                })
                .clicked()
            {
                self.config.paused = !self.config.paused;
                config_changed = true;
            }

            if ui.button("Reset").clicked() {
                send_message_to_simulation(MessageFromGuiToSimulator::Reset);
            }
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.heading("Population");
        ui.label(format!("Current boids: {}", simulation_data.count()));

        ui.label("Initial count:");
        if ui
            .add(egui::Slider::new(&mut self.config.boid_count, 10..=2000))
            .changed()
        {
            config_changed = true;
        }

        ui.horizontal(|ui| {
            if ui.button("+10").clicked() {
                send_message_to_simulation(MessageFromGuiToSimulator::SpawnBoids(10));
            }
            if ui.button("+50").clicked() {
                send_message_to_simulation(MessageFromGuiToSimulator::SpawnBoids(50));
            }
            if ui.button("+100").clicked() {
                send_message_to_simulation(MessageFromGuiToSimulator::SpawnBoids(100));
            }
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.heading("Speed");

        ui.label("Max speed:");
        if ui
            .add(egui::Slider::new(&mut self.config.max_speed, 10.0..=500.0))
            .changed()
        {
            config_changed = true;
        }

        ui.label("Min speed:");
        if ui
            .add(egui::Slider::new(&mut self.config.min_speed, 0.0..=200.0))
            .changed()
        {
            config_changed = true;
        }

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.heading("Behavior Weights");

        ui.label("Separation:");
        if ui
            .add(egui::Slider::new(
                &mut self.config.separation_weight,
                0.0..=5.0,
            ))
            .changed()
        {
            config_changed = true;
        }

        ui.label("Alignment:");
        if ui
            .add(egui::Slider::new(
                &mut self.config.alignment_weight,
                0.0..=5.0,
            ))
            .changed()
        {
            config_changed = true;
        }

        ui.label("Cohesion:");
        if ui
            .add(egui::Slider::new(
                &mut self.config.cohesion_weight,
                0.0..=5.0,
            ))
            .changed()
        {
            config_changed = true;
        }

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.heading("Perception Radii");

        ui.label("Separation radius:");
        if ui
            .add(egui::Slider::new(
                &mut self.config.separation_radius,
                5.0..=100.0,
            ))
            .changed()
        {
            config_changed = true;
        }

        ui.label("Alignment radius:");
        if ui
            .add(egui::Slider::new(
                &mut self.config.alignment_radius,
                10.0..=200.0,
            ))
            .changed()
        {
            config_changed = true;
        }

        ui.label("Cohesion radius:");
        if ui
            .add(egui::Slider::new(
                &mut self.config.cohesion_radius,
                20.0..=300.0,
            ))
            .changed()
        {
            config_changed = true;
        }

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.heading("Field of View");

        ui.label("FOV (degrees):");
        if ui
            .add(egui::Slider::new(
                &mut self.config.field_of_view,
                30.0..=360.0,
            ))
            .changed()
        {
            config_changed = true;
        }

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.heading("Visual Settings");

        ui.label("Boid size:");
        if ui
            .add(egui::Slider::new(&mut self.config.boid_size, 2.0..=20.0))
            .changed()
        {
            config_changed = true;
        }

        if ui
            .checkbox(&mut self.config.show_vision_radius, "Show vision radius")
            .changed()
        {
            config_changed = true;
        }

        if config_changed {
            Some(self.config.clone())
        } else {
            None
        }
    }

    fn content<F>(
        &mut self,
        simulation_data: &GuardArc<Self::SimulationData>,
        _ctx: &Context,
        _frame: &mut Frame,
        ui: &mut Ui,
        mut send_message_to_simulation: F,
    ) where
        F: FnMut(Self::MessageToSimulation),
    {
        let available_rect = ui.available_rect_before_wrap();
        let _response = ui.allocate_rect(available_rect, Sense::click_and_drag());

        // Update world size if changed
        let new_size = (available_rect.width(), available_rect.height());
        if (new_size.0 - self.last_world_size.0).abs() > 1.0
            || (new_size.1 - self.last_world_size.1).abs() > 1.0
        {
            self.last_world_size = new_size;
            send_message_to_simulation(MessageFromGuiToSimulator::ResizeWorld(
                new_size.0, new_size.1,
            ));
        }

        let painter = ui.painter_at(available_rect);

        // Draw background
        painter.rect_filled(available_rect, 0.0, Color32::from_rgb(10, 15, 30));

        // Draw boids
        for boid in &simulation_data.boids {
            let screen_pos = self.world_to_screen(boid.position, available_rect);

            // Draw vision radius if enabled
            if self.config.show_vision_radius {
                painter.circle_stroke(
                    screen_pos,
                    self.config.cohesion_radius,
                    Stroke::new(0.5, Color32::from_rgba_unmultiplied(100, 100, 255, 30)),
                );
            }

            // Calculate boid direction
            let direction = if boid.velocity.length_squared() > 0.0001 {
                boid.velocity.normalized()
            } else {
                Vec2::new(1.0, 0.0)
            };

            // Draw boid as a triangle pointing in the direction of movement
            let size = self.config.boid_size;
            let front = screen_pos + egui::Vec2::new(direction.x * size, direction.y * size);
            let back_left = screen_pos
                + egui::Vec2::new(
                    (-direction.x * 0.5 + direction.y * 0.5) * size,
                    (-direction.y * 0.5 - direction.x * 0.5) * size,
                );
            let back_right = screen_pos
                + egui::Vec2::new(
                    (-direction.x * 0.5 - direction.y * 0.5) * size,
                    (-direction.y * 0.5 + direction.x * 0.5) * size,
                );

            // Color based on velocity direction for visual variety
            let hue = (direction.angle() + std::f32::consts::PI) / std::f32::consts::TAU;
            let color = hsv_to_rgb(hue, 0.7, 1.0);

            painter.add(egui::Shape::convex_polygon(
                vec![front, back_left, back_right],
                color,
                Stroke::NONE,
            ));
        }

        // Draw world bounds indicator with line segments
        let bounds_color = Color32::from_gray(60);
        let bounds_stroke = Stroke::new(1.0, bounds_color);
        let w = simulation_data.width;
        let h = simulation_data.height;
        let tl = available_rect.left_top();

        painter.line_segment([tl, Pos2::new(tl.x + w, tl.y)], bounds_stroke);
        painter.line_segment(
            [Pos2::new(tl.x + w, tl.y), Pos2::new(tl.x + w, tl.y + h)],
            bounds_stroke,
        );
        painter.line_segment(
            [Pos2::new(tl.x + w, tl.y + h), Pos2::new(tl.x, tl.y + h)],
            bounds_stroke,
        );
        painter.line_segment([Pos2::new(tl.x, tl.y + h), tl], bounds_stroke);
    }
}

impl BoidsGui {
    fn world_to_screen(&self, world_pos: Vec2, rect: Rect) -> Pos2 {
        Pos2::new(rect.left() + world_pos.x, rect.top() + world_pos.y)
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color32 {
    let h = h.fract();
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);

    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match (h * 6.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Color32::from_rgb(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

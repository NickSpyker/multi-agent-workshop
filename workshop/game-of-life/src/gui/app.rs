use super::{GameOfLifeConfig, MessageFromGuiToSimulator};
use crate::{
    rle::{Pattern, PatternCollection},
    simulation::{GameOfLife, MessageFromSimulatorToGui},
};
use eframe::Frame;
use egui::{Color32, Context, Pos2, Rect, ScrollArea, Sense, Stroke, Ui, Vec2, Window};
use multi_agent::{GuardArc, MultiAgentGui};
use std::fmt::{self, Debug, Formatter};

pub struct GameOfLifeGui {
    offset: Vec2,
    zoom: f32,
    is_panning: bool,
    last_pan_pos: Option<Pos2>,
    last_drawn_cell: Option<(i64, i64)>,
    config: GameOfLifeConfig,
    pattern_collection: Option<PatternCollection>,
    pattern_search: String,
    selected_pattern: Option<Pattern>,
    placing_pattern: bool,
    pattern_browser_open: bool,
    dragging_popup: bool,
}

impl Debug for GameOfLifeGui {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameOfLifeGui")
            .field("offset", &self.offset)
            .field("zoom", &self.zoom)
            .field("is_panning", &self.is_panning)
            .field("last_pan_pos", &self.last_pan_pos)
            .field("last_drawn_cell", &self.last_drawn_cell)
            .field("config", &self.config)
            .field(
                "pattern_collection",
                &self.pattern_collection.as_ref().map(|c| c.len()),
            )
            .field("pattern_search", &self.pattern_search)
            .field(
                "selected_pattern",
                &self.selected_pattern.as_ref().map(|p| p.display_name()),
            )
            .field("placing_pattern", &self.placing_pattern)
            .field("pattern_browser_open", &self.pattern_browser_open)
            .field("dragging_popup", &self.dragging_popup)
            .finish()
    }
}

impl Default for GameOfLifeGui {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            zoom: 20.0,
            is_panning: false,
            last_pan_pos: None,
            last_drawn_cell: None,
            config: GameOfLifeConfig::default(),
            pattern_collection: PatternCollection::load().ok(),
            pattern_search: String::new(),
            selected_pattern: None,
            placing_pattern: false,
            pattern_browser_open: false,
            dragging_popup: false,
        }
    }
}

impl MultiAgentGui for GameOfLifeGui {
    const APP_NAME: &'static str = "Game of Life";

    type GuiData = GameOfLifeConfig;
    type SimulationData = GameOfLife;

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
        let mut config_changed: bool = false;

        ScrollArea::vertical().show(ui, |ui| {
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
            });

            if ui.button("Reset Cells").clicked() {
                send_message_to_simulation(MessageFromGuiToSimulator::Reset);
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label("Tick Rate (per second):");
            if ui
                .add(
                    egui::Slider::new(&mut self.config.tick_rate_per_second, 0.1..=1024.0)
                        .logarithmic(true),
                )
                .changed()
            {
                config_changed = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.heading("Statistics");
            ui.label(format!("Generation: {}", simulation_data.generation));
            ui.label(format!("Living cells: {}", simulation_data.cells.len()));

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.heading("View");
            ui.label(format!("Zoom: {:.1}x", self.zoom));
            ui.label(format!(
                "Offset: ({:.1}, {:.1})",
                self.offset.x, self.offset.y
            ));

            if ui.button("Reset View").clicked() {
                self.offset = Vec2::ZERO;
                self.zoom = 20.0;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.heading("Patterns");

            if self.placing_pattern {
                if let Some(ref pattern) = self.selected_pattern {
                    ui.label(format!("Selected: {}", pattern.display_name()));
                }
                ui.colored_label(Color32::YELLOW, "Click on grid to place");
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.placing_pattern = false;
                        self.selected_pattern = None;
                    }
                    if ui.button("Rotate (R)").clicked() {
                        if let Some(ref mut pattern) = self.selected_pattern {
                            pattern.rotate_cw();
                        }
                    }
                });
            } else if ui.button("Browse Patterns...").clicked() {
                self.pattern_browser_open = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.heading("Keyboard & Mouse");
            ui.label("Left click/drag: Add cells");
            ui.label("Right click/drag: Remove cells");
            ui.add_space(5.0);
            ui.label("Pan: Middle drag or Space+drag");
            ui.label("Zoom: Scroll or +/-");
            if self.placing_pattern {
                ui.add_space(5.0);
                ui.colored_label(Color32::YELLOW, "Left click: Place pattern");
                ui.colored_label(Color32::YELLOW, "Right click: Cancel");
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
        send_message_to_simulation: F,
    ) where
        F: FnMut(Self::MessageToSimulation),
    {
        let available_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(available_rect, Sense::click_and_drag());

        let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
        if scroll_delta != 0.0 && response.hovered() {
            let zoom_factor = 1.1_f32.powf(scroll_delta / 50.0);
            let new_zoom = (self.zoom * zoom_factor).clamp(2.0, 200.0);

            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                let mouse_grid_before = self.screen_to_grid(mouse_pos, available_rect);
                self.zoom = new_zoom;
                let mouse_grid_after = self.screen_to_grid(mouse_pos, available_rect);
                self.offset += mouse_grid_before - mouse_grid_after;
            } else {
                self.zoom = new_zoom;
            }
        }

        let zoom_in_pressed = ui.input(|i| {
            i.key_pressed(egui::Key::Plus)
                || i.key_pressed(egui::Key::Equals)
                || (i.modifiers.ctrl && i.key_pressed(egui::Key::ArrowUp))
        });
        let zoom_out_pressed = ui.input(|i| {
            i.key_pressed(egui::Key::Minus)
                || (i.modifiers.ctrl && i.key_pressed(egui::Key::ArrowDown))
        });

        if zoom_in_pressed {
            self.zoom = (self.zoom * 1.2).clamp(2.0, 200.0);
        }
        if zoom_out_pressed {
            self.zoom = (self.zoom / 1.2).clamp(2.0, 200.0);
        }

        let space_held =
            ui.input(|i| i.modifiers.alt) || ui.input(|i| i.key_down(egui::Key::Space));
        let middle_down = ui.input(|i| i.pointer.middle_down());
        let primary_down = ui.input(|i| i.pointer.primary_down());
        let is_panning_input = middle_down || (space_held && primary_down);

        if is_panning_input && response.hovered() {
            if let Some(current_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if let Some(last_pos) = self.last_pan_pos {
                    let delta = current_pos - last_pos;
                    self.offset -= Vec2::new(delta.x / self.zoom, delta.y / self.zoom);
                }
                self.last_pan_pos = Some(current_pos);
                self.is_panning = true;
            }
        } else {
            self.last_pan_pos = None;
            self.is_panning = false;
        }

        let pointer_over_popup: bool = ctx.is_pointer_over_area();
        let primary_down: bool = ui.input(|i| i.pointer.primary_down());

        if primary_down && pointer_over_popup {
            self.dragging_popup = true;
        }
        if !primary_down {
            self.dragging_popup = false;
        }

        if !pointer_over_popup && !self.dragging_popup {
            if self.placing_pattern {
                self.handle_pattern_placement(ui, available_rect, send_message_to_simulation);
            } else {
                self.handle_cell_interaction(ui, available_rect, send_message_to_simulation);
            }
        }

        let painter = ui.painter_at(available_rect);
        self.render_grid(&painter, available_rect);
        self.render_cells(&painter, available_rect, simulation_data);

        if self.placing_pattern {
            self.render_pattern_preview(&painter, ui, available_rect);
        }

        self.render_coordinates(ui, available_rect);

        self.render_pattern_browser_window(ctx);
    }
}

impl GameOfLifeGui {
    fn screen_to_grid(&self, screen_pos: Pos2, rect: Rect) -> Vec2 {
        let center = rect.center();
        Vec2::new(
            (screen_pos.x - center.x) / self.zoom + self.offset.x,
            (screen_pos.y - center.y) / self.zoom + self.offset.y,
        )
    }

    fn grid_to_screen(&self, grid_pos: Vec2, rect: Rect) -> Pos2 {
        let center = rect.center();
        Pos2::new(
            (grid_pos.x - self.offset.x).mul_add(self.zoom, center.x),
            (grid_pos.y - self.offset.y).mul_add(self.zoom, center.y),
        )
    }

    #[allow(clippy::cast_possible_truncation)]
    fn handle_cell_interaction<F>(&mut self, ui: &Ui, rect: Rect, mut send_message: F)
    where
        F: FnMut(MessageFromGuiToSimulator),
    {
        if self.is_panning {
            self.last_drawn_cell = None;
            return;
        }

        let primary_down = ui.input(|i| i.pointer.primary_down());
        let secondary_down = ui.input(|i| i.pointer.secondary_down());

        if !primary_down && !secondary_down {
            self.last_drawn_cell = None;
            return;
        }

        let Some(pos) = ui.input(|i| i.pointer.hover_pos()) else {
            return;
        };

        if !rect.contains(pos) {
            return;
        }

        let grid_pos = self.screen_to_grid(pos, rect);
        let cell_x = grid_pos.x.floor() as i64;
        let cell_y = grid_pos.y.floor() as i64;
        let current_cell = (cell_x, cell_y);

        if self.last_drawn_cell == Some(current_cell) {
            return;
        }

        self.last_drawn_cell = Some(current_cell);

        if primary_down {
            send_message(MessageFromGuiToSimulator::SpawnCells(vec![current_cell]));
        } else if secondary_down {
            send_message(MessageFromGuiToSimulator::RemoveCells(vec![current_cell]));
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn render_grid(&self, painter: &egui::Painter, rect: Rect) {
        if self.zoom < 4.0 {
            return;
        }

        let grid_color = Color32::from_gray(40);
        let origin_color = Color32::from_gray(80);

        let top_left = self.screen_to_grid(rect.left_top(), rect);
        let bottom_right = self.screen_to_grid(rect.right_bottom(), rect);

        let min_x = top_left.x.floor() as i64 - 1;
        let max_x = bottom_right.x.ceil() as i64 + 1;
        let min_y = top_left.y.floor() as i64 - 1;
        let max_y = bottom_right.y.ceil() as i64 + 1;

        for x in min_x..=max_x {
            if x == 0 {
                continue;
            }
            let screen_x = self.grid_to_screen(Vec2::new(x as f32, 0.0), rect).x;
            painter.line_segment(
                [
                    Pos2::new(screen_x, rect.top()),
                    Pos2::new(screen_x, rect.bottom()),
                ],
                Stroke::new(1.0, grid_color),
            );
        }

        for y in min_y..=max_y {
            if y == 0 {
                continue;
            }
            let screen_y = self.grid_to_screen(Vec2::new(0.0, y as f32), rect).y;
            painter.line_segment(
                [
                    Pos2::new(rect.left(), screen_y),
                    Pos2::new(rect.right(), screen_y),
                ],
                Stroke::new(1.0, grid_color),
            );
        }

        let origin_x = self.grid_to_screen(Vec2::new(0.0, 0.0), rect).x;
        let origin_y = self.grid_to_screen(Vec2::new(0.0, 0.0), rect).y;

        painter.line_segment(
            [
                Pos2::new(origin_x, rect.top()),
                Pos2::new(origin_x, rect.bottom()),
            ],
            Stroke::new(2.0, origin_color),
        );

        painter.line_segment(
            [
                Pos2::new(rect.left(), origin_y),
                Pos2::new(rect.right(), origin_y),
            ],
            Stroke::new(2.0, origin_color),
        );
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn render_cells(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        simulation_data: &GuardArc<GameOfLife>,
    ) {
        let cell_color = Color32::WHITE;

        let top_left = self.screen_to_grid(rect.left_top(), rect);
        let bottom_right = self.screen_to_grid(rect.right_bottom(), rect);

        let min_x = top_left.x.floor() as i64 - 1;
        let max_x = bottom_right.x.ceil() as i64 + 1;
        let min_y = top_left.y.floor() as i64 - 1;
        let max_y = bottom_right.y.ceil() as i64 + 1;

        for &(x, y) in &simulation_data.cells {
            if x < min_x || x > max_x || y < min_y || y > max_y {
                continue;
            }

            let top_left_screen = self.grid_to_screen(Vec2::new(x as f32, y as f32), rect);
            let bottom_right_screen =
                self.grid_to_screen(Vec2::new((x + 1) as f32, (y + 1) as f32), rect);

            let cell_rect = Rect::from_two_pos(top_left_screen, bottom_right_screen).shrink(1.0);
            painter.rect_filled(cell_rect, 0.0, cell_color);
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn render_coordinates(&self, ui: &Ui, rect: Rect) {
        let font_id = egui::FontId::proportional(12.0);
        let text_color = Color32::from_gray(150);

        let top_left = self.screen_to_grid(rect.left_top(), rect);
        let bottom_right = self.screen_to_grid(rect.right_bottom(), rect);

        let step = self.calculate_coordinate_step();

        let min_x = (top_left.x / step as f32).floor() as i64 * step;
        let max_x = (bottom_right.x / step as f32).ceil() as i64 * step;
        let min_y = (top_left.y / step as f32).floor() as i64 * step;
        let max_y = (bottom_right.y / step as f32).ceil() as i64 * step;

        let mut x = min_x;
        while x <= max_x {
            let screen_x = self.grid_to_screen(Vec2::new(x as f32, 0.0), rect).x;
            if screen_x >= rect.left() && screen_x <= rect.right() {
                ui.painter().text(
                    Pos2::new(screen_x, rect.top() + 10.0),
                    egui::Align2::CENTER_TOP,
                    x.to_string(),
                    font_id.clone(),
                    text_color,
                );

                ui.painter().text(
                    Pos2::new(screen_x, rect.bottom() - 10.0),
                    egui::Align2::CENTER_BOTTOM,
                    x.to_string(),
                    font_id.clone(),
                    text_color,
                );
            }
            x += step;
        }

        let mut y = min_y;
        while y <= max_y {
            let screen_y = self.grid_to_screen(Vec2::new(0.0, y as f32), rect).y;
            if screen_y >= rect.top() && screen_y <= rect.bottom() {
                ui.painter().text(
                    Pos2::new(rect.left() + 10.0, screen_y),
                    egui::Align2::LEFT_CENTER,
                    y.to_string(),
                    font_id.clone(),
                    text_color,
                );

                ui.painter().text(
                    Pos2::new(rect.right() - 10.0, screen_y),
                    egui::Align2::RIGHT_CENTER,
                    y.to_string(),
                    font_id.clone(),
                    text_color,
                );
            }
            y += step;
        }
    }

    fn calculate_coordinate_step(&self) -> i64 {
        let pixels_per_label = 50.0;
        let cells_per_label = pixels_per_label / self.zoom;

        if cells_per_label <= 1.0 {
            1
        } else if cells_per_label <= 2.0 {
            2
        } else if cells_per_label <= 5.0 {
            5
        } else if cells_per_label <= 10.0 {
            10
        } else if cells_per_label <= 20.0 {
            20
        } else if cells_per_label <= 50.0 {
            50
        } else {
            100
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn handle_pattern_placement<F>(&mut self, ui: &Ui, rect: Rect, mut send_message: F)
    where
        F: FnMut(MessageFromGuiToSimulator),
    {
        if self.is_panning {
            return;
        }

        let primary_clicked = ui.input(|i| i.pointer.primary_clicked());
        let secondary_clicked = ui.input(|i| i.pointer.secondary_clicked());

        if secondary_clicked {
            self.placing_pattern = false;
            self.selected_pattern = None;
            return;
        }

        if primary_clicked {
            if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                if rect.contains(pos) {
                    if let Some(ref pattern) = self.selected_pattern {
                        let grid_pos = self.screen_to_grid(pos, rect);
                        let cell_x = grid_pos.x.floor() as i64;
                        let cell_y = grid_pos.y.floor() as i64;

                        let cells = pattern.cells_at_position(cell_x, cell_y);
                        send_message(MessageFromGuiToSimulator::PlacePattern(cells));
                    }
                }
            }
        }

        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.placing_pattern = false;
            self.selected_pattern = None;
        }

        if ui.input(|i| i.key_pressed(egui::Key::R)) {
            if let Some(ref mut pattern) = self.selected_pattern {
                pattern.rotate_cw();
            }
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn render_pattern_preview(&self, painter: &egui::Painter, ui: &Ui, rect: Rect) {
        let Some(ref pattern) = self.selected_pattern else {
            return;
        };

        let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) else {
            return;
        };

        if !rect.contains(mouse_pos) {
            return;
        }

        let grid_pos = self.screen_to_grid(mouse_pos, rect);
        let cell_x = grid_pos.x.floor() as i64;
        let cell_y = grid_pos.y.floor() as i64;

        let cells = pattern.cells_at_position(cell_x, cell_y);

        let preview_color = Color32::from_rgba_unmultiplied(255, 255, 0, 150);

        for (x, y) in cells {
            let top_left_screen = self.grid_to_screen(Vec2::new(x as f32, y as f32), rect);
            let bottom_right_screen =
                self.grid_to_screen(Vec2::new((x + 1) as f32, (y + 1) as f32), rect);

            let cell_rect = Rect::from_two_pos(top_left_screen, bottom_right_screen).shrink(1.0);
            painter.rect_filled(cell_rect, 0.0, preview_color);
        }
    }

    fn render_pattern_browser_window(&mut self, ctx: &Context) {
        if !self.pattern_browser_open {
            return;
        }

        let mut open: bool = self.pattern_browser_open;

        Window::new("Pattern Browser")
            .open(&mut open)
            .default_size([200.0, 400.0])
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.pattern_search);
                });

                ui.add_space(5.0);

                if let Some(ref collection) = self.pattern_collection {
                    let patterns: Vec<&Pattern> = if self.pattern_search.is_empty() {
                        collection.patterns().iter().collect()
                    } else {
                        collection.search(&self.pattern_search)
                    };

                    ui.label(format!("{} patterns found", patterns.len()));

                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    ScrollArea::vertical().show(ui, |ui| {
                        for pattern in patterns.iter().take(200) {
                            let is_selected = self
                                .selected_pattern
                                .as_ref()
                                .map_or(false, |p| p.display_name() == pattern.display_name());

                            ui.horizontal(|ui| {
                                let label = format!(
                                    "{} ({}x{})",
                                    pattern.display_name(),
                                    pattern.width,
                                    pattern.height
                                );

                                if ui.selectable_label(is_selected, label).clicked() {
                                    self.selected_pattern = Some((*pattern).clone());
                                    self.placing_pattern = true;
                                    self.pattern_browser_open = false;
                                }
                            });
                        }
                    });
                } else {
                    ui.colored_label(Color32::RED, "Failed to load patterns");
                }
            });

        self.pattern_browser_open = open;
    }
}

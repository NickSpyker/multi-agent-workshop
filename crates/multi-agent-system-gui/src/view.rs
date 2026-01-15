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
    egui::{Context, Ui},
    Frame,
};
use std::fmt::Debug;

pub trait View: Debug {
    fn name(&self) -> &str;
    fn sidebar(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui);
    fn content(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui);
}

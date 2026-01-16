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

use multi_agent_core::Result;
use multi_agent_gui::{Gui, View};

#[derive(Debug, Default)]
pub struct App {
    gui: Gui,
}

impl App {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(self) -> Result<()> {
        self.gui.run()
    }
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn test_app_new() {
        let _app = App::new();
    }
}

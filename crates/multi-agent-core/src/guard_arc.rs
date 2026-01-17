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

use arc_swap::Guard;
use std::sync::Arc;

/// A lock-free read guard for simulation data shared between threads.
///
/// `GuardArc<T>` provides read-only access to simulation data without blocking writes.
/// It uses the `arc-swap` crate internally to enable wait-free reads, ensuring the GUI
/// thread never blocks the simulation thread.
///
/// # Usage
///
/// You receive `GuardArc<SimulationData>` as a parameter in the `sidebar()` and `content()`
/// methods of `MultiAgentGui`. Use it like a reference via the `Deref` trait:
///
/// ```no_run
/// # use multi_agent_core::GuardArc;
/// # struct Agent { x: f32 }
/// fn render_agents(data: &GuardArc<Vec<Agent>>) {
///     for agent in data.iter() {  // Automatic deref to &Vec<Agent>
///         println!("Agent at x={}", agent.x);
///     }
/// }
/// ```
///
/// # Performance
///
/// Reading through `GuardArc` is extremely fast (a few atomic operations) and never blocks.
/// The simulation thread can update the data concurrently without waiting for readers.
pub type GuardArc<T> = Guard<Arc<T>>;

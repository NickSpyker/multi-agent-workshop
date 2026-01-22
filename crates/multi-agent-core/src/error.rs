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

use std::time::Duration;
use thiserror::Error as ThisError;

/// Errors that can occur in the multi-agent framework.
#[derive(Debug, ThisError)]
#[non_exhaustive]
pub enum Error {
    /// Simulation thread panicked with the given error message.
    #[error("Simulation thread panicked: {0}")]
    SimulationPanic(String),

    /// Simulation thread failed to stop within the specified timeout period.
    #[error("Simulation thread failed to stop within {timeout:?}")]
    ShutdownTimeout {
        /// The timeout duration that was exceeded.
        timeout: Duration,
    },

    /// GUI error occurred with the given error message.
    #[error("GUI error: {0}")]
    Gui(String),

    /// Message channel is full and cannot accept more messages.
    #[error("Message channel full (capacity: {capacity})")]
    MessageChannelFull {
        /// The capacity of the channel.
        capacity: usize,
    },

    /// Message channel is disconnected and cannot send messages.
    #[error("Message channel disconnected")]
    MessageChannelDisconnected,
}

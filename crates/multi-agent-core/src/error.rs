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

use std::{
    any::Any,
    error,
    fmt::{self, Debug, Display, Formatter},
};

/// Errors that can occur in the multi-agent runtime.
///
/// This error type covers failures in thread management, GUI initialization,
/// and message passing between simulation and GUI threads.
///
/// The `#[non_exhaustive]` attribute means new error variants may be added
/// in future versions without breaking existing code.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A thread panicked with the given payload.
    Thread(Box<dyn Any + Send + 'static>),

    /// The simulation thread failed to stop within the timeout period.
    ThreadStopTimeout,

    /// An error occurred during GUI initialization or rendering.
    Gui(String),

    /// Attempted to send a message but the channel is full.
    MessageSenderFull,

    /// Attempted to send a message but the receiving end disconnected.
    MessageSenderDisconnected,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thread(err) => write!(f, "{err:?}"),
            Self::ThreadStopTimeout => write!(
                f,
                "thread failed to stop within the timeout period and will be abandoned"
            ),
            Self::Gui(err) => write!(f, "{err}"),
            Self::MessageSenderFull => write!(f, "sending on a full channel"),
            Self::MessageSenderDisconnected => write!(f, "sending on a disconnected channel"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Thread(_) => None,
            Self::ThreadStopTimeout => None,
            Self::Gui(_) => None,
            Self::MessageSenderFull => None,
            Self::MessageSenderDisconnected => None,
        }
    }
}

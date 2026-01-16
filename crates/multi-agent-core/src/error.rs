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

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Thread(Box<dyn Any + Send + 'static>),
    Gui(eframe::Error),
    GuiViewAlreadyExists(String),
    MessageSenderFull,
    MessageSenderDisconnected,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thread(err) => write!(f, "{err:?}"),
            Self::Gui(err) => write!(f, "{err}"),
            Self::GuiViewAlreadyExists(err) => write!(f, "view already exists {err}"),
            Self::MessageSenderFull => write!(f, "sending on a full channel"),
            Self::MessageSenderDisconnected => write!(f, "sending on a disconnected channel"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Thread(_) => None,
            Self::Gui(err) => Some(err),
            Self::GuiViewAlreadyExists(_) => None,
            Self::MessageSenderFull => None,
            Self::MessageSenderDisconnected => None,
        }
    }
}

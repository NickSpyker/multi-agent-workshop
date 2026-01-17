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

use crossbeam_channel::{Sender, TrySendError};
use multi_agent_core::{Error, Result};
use std::fmt::Debug;

/// The sending half of a message channel.
///
/// `MessageSender` can send discrete messages to the receiving thread.
/// It never blocks - if the channel is full, the message is either
/// dropped (with `send_lossy`) or an error is returned (with `send`).
///
/// Can be cloned to create multiple senders for the same channel.
#[derive(Debug, Clone)]
pub struct MessageSender<T> {
    inner: Sender<T>,
}

impl<T> MessageSender<T> {
    #[inline]
    pub(super) fn new(sender: Sender<T>) -> Self {
        Self { inner: sender }
    }

    /// Attempts to send a message, returning an error if the channel is full or disconnected.
    ///
    /// This method never blocks. If the channel is at capacity, it returns
    /// `Err(Error::MessageSenderFull)`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use multi_agent_sync::message::MessageChannel;
    /// # let channel = MessageChannel::<i32>::new(10);
    /// # let (sender, _) = channel.split();
    /// match sender.send(42) {
    ///     Ok(()) => println!("Message sent"),
    ///     Err(e) => println!("Send failed: {}", e),
    /// }
    /// ```
    #[inline]
    pub fn send(&self, message: T) -> Result<()> {
        self.inner.try_send(message).map_err(|err| match err {
            TrySendError::Full(_) => Error::MessageSenderFull,
            TrySendError::Disconnected(_) => Error::MessageSenderDisconnected,
        })
    }

    /// Sends a message, silently dropping it if the channel is full.
    ///
    /// This is a convenience method for when you want to send messages
    /// without handling errors. If the channel is full or disconnected,
    /// the message is simply discarded.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use multi_agent_sync::message::MessageChannel;
    /// # let channel = MessageChannel::<i32>::new(10);
    /// # let (sender, _) = channel.split();
    /// sender.send_lossy(42);  // Fire and forget
    /// ```
    #[inline]
    pub fn send_lossy(&self, message: T) {
        let _ = self.send(message);
    }

    /// Returns the number of messages currently in the channel.
    #[inline]
    pub fn pending(&self) -> usize {
        self.inner.len()
    }

    /// Returns the channel capacity (maximum number of buffered messages).
    #[inline]
    pub fn capacity(&self) -> Option<usize> {
        self.inner.capacity()
    }

    /// Returns `true` if there are no messages in the channel.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns `true` if the channel is at capacity.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.inner.is_full()
    }
}

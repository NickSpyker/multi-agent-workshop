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

use crossbeam_channel::Receiver;
use std::fmt::Debug;

/// The receiving half of a message channel.
///
/// `MessageReceiver` can receive discrete messages from the sending thread.
/// It never blocks - methods return immediately with available messages or `None`.
///
/// Can be cloned to create multiple receivers for the same channel (though
/// in practice, the runtime creates only one receiver per direction).
#[derive(Debug, Clone)]
pub struct MessageReceiver<T> {
    inner: Receiver<T>,
}

impl<T> MessageReceiver<T> {
    #[inline]
    pub(super) fn new(sender: Receiver<T>) -> Self {
        Self { inner: sender }
    }

    /// Drains all pending messages from the channel into a vector.
    ///
    /// This is the most common way to receive messages. It returns all messages
    /// that are currently in the channel, or an empty vector if none are available.
    /// Never blocks.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use multi_agent_sync::message::MessageChannel;
    /// # let channel = MessageChannel::<i32>::new(10);
    /// # let (sender, receiver) = channel.split();
    /// # sender.send_lossy(1);
    /// # sender.send_lossy(2);
    /// let messages = receiver.drain();
    /// for msg in messages {
    ///     println!("Received: {}", msg);
    /// }
    /// ```
    #[inline]
    pub fn drain(&self) -> Vec<T> {
        self.inner.try_iter().collect()
    }

    /// Drains up to `limit` messages from the channel.
    ///
    /// Like `drain()`, but stops after receiving `limit` messages even if
    /// more are available. Useful for rate-limiting message processing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use multi_agent_sync::message::MessageChannel;
    /// # let channel = MessageChannel::<i32>::new(10);
    /// # let (_, receiver) = channel.split();
    /// let messages = receiver.drain_limit(5);  // Get at most 5 messages
    /// ```
    #[inline]
    pub fn drain_limit(&self, limit: usize) -> Vec<T> {
        self.inner.try_iter().take(limit).collect()
    }

    /// Tries to receive a single message, returning `None` if the channel is empty.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use multi_agent_sync::message::MessageChannel;
    /// # let channel = MessageChannel::<i32>::new(10);
    /// # let (_, receiver) = channel.split();
    /// if let Some(msg) = receiver.try_recv() {
    ///     println!("Received: {}", msg);
    /// }
    /// ```
    #[inline]
    pub fn try_recv(&self) -> Option<T> {
        self.inner.try_recv().ok()
    }

    /// Returns an iterator over all currently pending messages.
    ///
    /// The iterator will yield all messages currently in the channel and then stop.
    /// It never blocks.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use multi_agent_sync::message::MessageChannel;
    /// # let channel = MessageChannel::<i32>::new(10);
    /// # let (_, receiver) = channel.split();
    /// for msg in receiver.iter() {
    ///     println!("Received: {}", msg);
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.inner.try_iter()
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

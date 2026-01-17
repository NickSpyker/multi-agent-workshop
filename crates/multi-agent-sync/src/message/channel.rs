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

use super::{MessageReceiver, MessageSender};
use crossbeam_channel::{self as channel, Receiver, Sender};

/// A bounded message channel for sending discrete events between threads.
///
/// `MessageChannel<T>` provides a pair of `MessageSender` and `MessageReceiver`
/// for communicating one-off events (like button clicks) between the GUI and
/// simulation threads.
///
/// # Bounded Capacity
///
/// The channel has a fixed capacity. If the channel is full, sends will fail
/// gracefully (using `send_lossy`) rather than blocking or panicking.
///
/// # Example
///
/// ```no_run
/// use multi_agent_sync::message::MessageChannel;
///
/// #[derive(Clone)]
/// enum Command {
///     Start,
///     Stop,
/// }
///
/// let channel = MessageChannel::new(10);  // Capacity of 10 messages
/// let (sender, receiver) = channel.split();
///
/// // Send a message
/// sender.send_lossy(Command::Start);
///
/// // Receive all pending messages
/// let messages = receiver.drain();
/// ```
#[derive(Debug)]
pub struct MessageChannel<T> {
    pub sender: MessageSender<T>,
    pub receiver: MessageReceiver<T>,
}

impl<T> MessageChannel<T> {
    /// Creates a new bounded message channel with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of messages that can be buffered
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use multi_agent_sync::message::MessageChannel;
    /// let channel = MessageChannel::<String>::new(100);
    /// ```
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver): (Sender<T>, Receiver<T>) = channel::bounded(capacity);

        Self {
            sender: MessageSender::new(sender),
            receiver: MessageReceiver::new(receiver),
        }
    }

    /// Splits the channel into separate sender and receiver halves.
    ///
    /// This consumes the channel and returns the sender and receiver,
    /// which can then be moved to different threads.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use multi_agent_sync::message::MessageChannel;
    /// let channel = MessageChannel::<i32>::new(10);
    /// let (sender, receiver) = channel.split();
    /// ```
    #[inline]
    pub fn split(self) -> (MessageSender<T>, MessageReceiver<T>) {
        (self.sender, self.receiver)
    }
}

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

#[derive(Debug)]
pub struct MessageChannel<T> {
    pub sender: MessageSender<T>,
    pub receiver: MessageReceiver<T>,
}

impl<T> MessageChannel<T> {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver): (Sender<T>, Receiver<T>) = channel::bounded(capacity);

        Self {
            sender: MessageSender::new(sender),
            receiver: MessageReceiver::new(receiver),
        }
    }

    #[inline]
    pub fn split(self) -> (MessageSender<T>, MessageReceiver<T>) {
        (self.sender, self.receiver)
    }
}

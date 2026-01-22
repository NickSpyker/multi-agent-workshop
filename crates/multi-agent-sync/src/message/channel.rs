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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_send_receive() {
        let (sender, receiver) = MessageChannel::new(10).split();

        sender.send(42).expect("Failed to send message");
        let messages = receiver.drain();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], 42);
    }

    #[test]
    fn test_channel_capacity_full() {
        let (sender, _receiver) = MessageChannel::new(2).split();

        assert!(sender.send(1).is_ok());
        assert!(sender.send(2).is_ok());

        // Channel is now full
        let result = sender.send(3);
        assert!(result.is_err());
    }

    #[test]
    fn test_channel_drain_multiple() {
        let (sender, receiver) = MessageChannel::new(10).split();

        sender.send(1).unwrap();
        sender.send(2).unwrap();
        sender.send(3).unwrap();

        let messages = receiver.drain();
        assert_eq!(messages, vec![1, 2, 3]);

        // Drain again should be empty
        let messages2 = receiver.drain();
        assert!(messages2.is_empty());
    }

    #[test]
    fn test_channel_is_empty_is_full() {
        let (sender, receiver) = MessageChannel::new(2).split();

        assert!(sender.is_empty());
        assert!(!sender.is_full());

        sender.send(1).unwrap();
        assert!(!sender.is_empty());
        assert!(!sender.is_full());

        sender.send(2).unwrap();
        assert!(!sender.is_empty());
        assert!(sender.is_full());

        receiver.drain();
        assert!(sender.is_empty());
        assert!(!sender.is_full());
    }

    #[test]
    fn test_channel_lossy_send() {
        let (sender, receiver) = MessageChannel::new(2).split();

        sender.send(1).unwrap();
        sender.send(2).unwrap();

        // Lossy send should not panic when full
        sender.send_lossy(3);
        sender.send_lossy(4);

        let messages = receiver.drain();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages, vec![1, 2]);
    }

    #[test]
    fn test_channel_pending() {
        let (sender, receiver) = MessageChannel::new(10).split();

        assert_eq!(sender.pending(), 0);

        sender.send(1).unwrap();
        assert_eq!(sender.pending(), 1);

        sender.send(2).unwrap();
        assert_eq!(sender.pending(), 2);

        receiver.drain();
        assert_eq!(sender.pending(), 0);
    }

    #[test]
    fn test_channel_try_recv() {
        let (sender, receiver) = MessageChannel::new(10).split();

        assert_eq!(receiver.try_recv(), None);

        sender.send(42).unwrap();
        assert_eq!(receiver.try_recv(), Some(42));
        assert_eq!(receiver.try_recv(), None);
    }

    #[test]
    fn test_channel_drain_limit() {
        let (sender, receiver) = MessageChannel::new(10).split();

        for i in 0..5 {
            sender.send(i).unwrap();
        }

        let messages = receiver.drain_limit(3);
        assert_eq!(messages.len(), 3);
        assert_eq!(messages, vec![0, 1, 2]);

        let remaining = receiver.drain();
        assert_eq!(remaining, vec![3, 4]);
    }
}

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

use arc_swap::ArcSwap;
use multi_agent_core::GuardArc;
use std::sync::Arc;

/// A thread-safe, lock-free shared state container using the RCU (Read-Copy-Update) pattern.
///
/// `Shared<T>` allows multiple threads to safely read and modify shared data without locks.
/// Reads are extremely fast (lock-free), while writes involve cloning the data.
///
/// # Performance Characteristics
/// - **Read (`load`)**: Lock-free, very fast, O(1)
/// - **Write (`store`, `update`)**: Requires cloning T, O(n) where n is the size of T
///
/// # Best Practices
/// - Keep `T` small and cheap to clone
/// - Use for read-heavy workloads (many reads, few writes)
/// - For large data structures, consider wrapping in `Arc` internally
///
/// # Example
/// ```rust
/// use multi_agent_sync::Shared;
///
/// #[derive(Clone)]
/// struct GameState {
///     score: i32,
///     level: u32,
/// }
///
/// let state = Shared::new(GameState { score: 0, level: 1 });
///
/// // Read from any thread
/// let current = state.load();
/// println!("Score: {}", current.score);
///
/// // Write from any thread
/// state.update(|s| {
///     s.score += 100;
/// });
/// ```
#[derive(Debug, Clone)]
pub struct Shared<T> {
    inner: Arc<ArcSwap<T>>,
}

impl<T> Shared<T> {
    /// Create a new shared state container with the given initial value.
    ///
    /// # Example
    /// ```rust
    /// use multi_agent_sync::Shared;
    ///
    /// let shared = Shared::new(42);
    /// assert_eq!(**shared.load(), 42);
    /// ```
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(ArcSwap::from_pointee(data)),
        }
    }

    /// Load a reference to the current value.
    ///
    /// This operation is lock-free and very fast. The returned guard keeps the
    /// value alive even if other threads update the shared state.
    ///
    /// # Returns
    /// A guard that dereferences to `&T`. The guard keeps the underlying data alive.
    ///
    /// # Example
    /// ```rust
    /// use multi_agent_sync::Shared;
    ///
    /// let shared = Shared::new(vec![1, 2, 3]);
    /// let data = shared.load();
    /// assert_eq!(data.len(), 3);
    /// ```
    #[inline]
    pub fn load(&self) -> GuardArc<T> {
        self.inner.load()
    }
}

impl<T: Clone> Shared<T> {
    /// Replace the current value with a new one.
    ///
    /// This operation clones the new value into an `Arc`. Readers that have already
    /// loaded the old value will continue to see it until they load again.
    ///
    /// # Arguments
    /// * `data` - The new value to store
    ///
    /// # Example
    /// ```rust
    /// use multi_agent_sync::Shared;
    ///
    /// let shared = Shared::new(42);
    /// shared.store(100);
    /// assert_eq!(**shared.load(), 100);
    /// ```
    #[inline]
    pub fn store(&self, data: T) {
        self.inner.store(Arc::new(data));
    }

    /// Update the value using a closure (RCU pattern).
    ///
    /// This operation:
    /// 1. Loads the current value
    /// 2. Clones it
    /// 3. Applies the closure to the clone
    /// 4. Stores the modified clone
    ///
    /// # Arguments
    /// * `f` - A closure that modifies the value
    ///
    /// # Performance
    /// This operation clones `T`, so it's best suited for small types or infrequent updates.
    ///
    /// # Example
    /// ```rust
    /// use multi_agent_sync::Shared;
    ///
    /// let shared = Shared::new(vec![1, 2, 3]);
    /// shared.update(|v| v.push(4));
    /// assert_eq!(shared.load().len(), 4);
    /// ```
    #[inline]
    pub fn update<F: Fn(&mut T)>(&self, f: F) {
        self.inner.rcu(|current_data: &Arc<T>| {
            let mut new_data: T = (**current_data).clone();
            f(&mut new_data);
            new_data
        });
    }
}

#[cfg(test)]
mod tests {
    use super::Shared;
    use std::{
        thread,
        time::Duration,
    };

    #[test]
    fn test_shared_load() {
        let data: String = "Hello, World!".to_string();
        let shared: Shared<String> = Shared::new(data.clone());

        assert_eq!(*shared.load(), data.into());
    }

    #[test]
    fn test_shared_store() {
        let data: String = "Hello, World!".to_string();
        let shared: Shared<String> = Shared::new(data);

        let data: String = "Goodbye, World!".to_string();
        shared.store(data.clone());

        assert_eq!(*shared.load(), data.into());
    }

    #[derive(Debug, Clone)]
    struct MockData {
        pub data_number: u8,
        pub data_string: String,
    }

    impl MockData {
        #[inline]
        pub fn new(data_number: u8, data_string: String) -> Self {
            Self {
                data_number,
                data_string,
            }
        }
    }

    #[test]
    fn test_concurrent_updates() {
        // Test that multiple threads can update without data races
        let shared = Shared::new(MockData::new(0, String::new()));
        let mut handles = vec![];

        for _ in 0..4 {
            let shared_clone = shared.clone();
            let handle = thread::spawn(move || {
                for _ in 0..25 {
                    shared_clone.update(|data| data.data_number += 1);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // All 100 increments should be visible
        assert_eq!(shared.load().data_number, 100);
    }

    #[test]
    fn test_update_atomicity() {
        // Test that updates are atomic (all-or-nothing)
        let shared = Shared::new(MockData::new(0, "initial".to_string()));

        shared.update(|data| {
            data.data_number = 42;
            data.data_string = "updated".to_string();
        });

        let result = shared.load();
        assert_eq!(result.data_number, 42);
        assert_eq!(result.data_string, "updated");
    }

    #[test]
    fn test_update_visibility() {
        // Test that updates are immediately visible to other threads
        let shared = Shared::new(MockData::new(0, String::new()));
        let shared_clone = shared.clone();

        let handle = thread::spawn(move || {
            shared_clone.store(MockData::new(100, "done".to_string()));
        });

        handle.join().unwrap();

        // Update from thread should be visible
        assert_eq!(shared.load().data_number, 100);
    }

    #[test]
    fn test_concurrent_read_write() {
        // Test that readers see consistent state during concurrent writes
        let shared = Shared::new(MockData::new(0, "0".to_string()));

        let reader = {
            let shared = shared.clone();
            thread::spawn(move || {
                for _ in 0..100 {
                    let data = shared.load();
                    // Number and string should always be consistent
                    assert_eq!(data.data_string, data.data_number.to_string());
                    thread::sleep(Duration::from_micros(10));
                }
            })
        };

        let writer = {
            let shared = shared.clone();
            thread::spawn(move || {
                for i in 0..50 {
                    shared.update(|data| {
                        data.data_number = i;
                        data.data_string = i.to_string();
                    });
                    thread::sleep(Duration::from_micros(100));
                }
            })
        };

        reader.join().unwrap();
        writer.join().unwrap();
    }
}

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

#[derive(Debug, Clone)]
pub struct Shared<T: Clone> {
    inner: Arc<ArcSwap<T>>,
}

impl<T: Clone> Shared<T> {
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(ArcSwap::from_pointee(data)),
        }
    }

    #[inline]
    pub fn load(&self) -> GuardArc<T> {
        self.inner.load()
    }

    #[inline]
    pub fn store(&self, data: T) {
        self.inner.store(Arc::new(data));
    }

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
    use super::{GuardArc, Shared};
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread::{self, JoinHandle},
        time::{Duration, Instant},
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
    fn test_shared_update() {
        let timeout: Duration = Duration::from_secs(5);
        let start_time: Instant = Instant::now();
        let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

        let data: MockData = MockData::new(0, "0".to_string());
        let shared: Shared<MockData> = Shared::new(data);

        let shared1: Shared<MockData> = shared.clone();
        let running1: Arc<AtomicBool> = running.clone();
        let thread1: JoinHandle<()> = thread::spawn(move || {
            while running1.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(10));
                match shared1.load().data_number {
                    1 => shared1.update(|shared| {
                        shared.data_number = 2;
                    }),
                    3 => shared1.update(|shared| {
                        shared.data_number = 4;
                        shared.data_string = "1".to_string();
                    }),
                    5 => break,
                    _ => continue,
                }
            }
        });

        let shared2: Shared<MockData> = shared.clone();
        let running2: Arc<AtomicBool> = running.clone();
        let thread2: JoinHandle<()> = thread::spawn(move || {
            while running2.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(10));
                match shared2.load().data_number {
                    0 => shared2.update(|shared| {
                        shared.data_number = 1;
                    }),
                    2 => shared2.update(|shared| {
                        shared.data_number = 3;
                    }),
                    4 => {
                        shared2.update(|shared| {
                            shared.data_number = 5;
                            if shared.data_string.eq("1") {
                                shared.data_string = "2".to_string();
                            }
                        });
                        break;
                    }
                    _ => continue,
                }
            }
        });

        while !thread1.is_finished() || !thread2.is_finished() {
            if start_time.elapsed() > timeout {
                running.store(false, Ordering::Relaxed);
                panic!("The test exceeded the time limit of {timeout:?}");
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(thread1.join().is_ok());
        assert!(thread2.join().is_ok());

        let shared: GuardArc<MockData> = shared.load();
        assert_eq!(shared.data_number, 5);
        assert_eq!(shared.data_string, "2");
    }
}

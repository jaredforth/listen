// Copyright 2020 Jared Forth.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A filesystem listener.
//!
//! **listen** listens for filesystem events and executes a function when the event occurs.

use walkdir::WalkDir;

/// `Event` enumerates the possible filesystem events
/// that can be listened for.
pub enum Event {
    /// If a file is added or removed from the directory
    /// this event is triggered.
    OnFileChange,
    /// If a file is added to the directory
    /// this event is triggered.
    OnFileAdd,
    /// If a file is removed from the directory
    /// this event is triggered.
    OnFileRemove
}

/// This function takes a path to listen on, the event to listen for,
/// and a function to execute when that event happens.
///
/// `path: &str` is the directory to listen on,
/// `event: Event` is the event to listen for,
/// `arg: T` is the argument to pass to `func`,
/// and
/// `func: fn(T) -> R` is the function to execute when event happens
pub fn listen<T, R>(path: &str, event: Event, arg: T, func: fn(T) -> R) {
    internal_listener(path, event, arg, func);
}

/// A Listener
pub struct Listener {
    /// The event to listen for
    event: Event,
    /// If true, the listener will store files in `store_files`;
    /// otherwise it will not
    store_files: bool,
    /// Vector of all files currently in directory
    /// listened to.
    pub files: Vec<String>,
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            event: Event::OnFileChange,
            store_files: false,
            files: vec![],
        }
    }
}

impl Listener {
    pub fn new() -> Listener {
        Listener::default()
    }
    /// This function takes a path to listen on, the event to listen for,
    /// and a function to execute when that event happens.
    ///
    /// `path: &str` is the directory to listen on,
    /// `arg: T` is the argument to pass to `func`,
    /// and
    /// `func: fn(T) -> R` is the function to execute when event happens
    pub fn listen<T, R>(&mut self, path: &str, arg: T, func: fn(T) -> R) {
        match self.event {
            Event::OnFileChange => {
                if self.file_change_listener(path) {
                    func(arg);
                }
            }
            Event::OnFileAdd => {
                unimplemented!()
            }
            Event::OnFileRemove => {
                unimplemented!()
            }
        }
    }
    fn file_change_listener(&mut self, path: &str) -> bool {
        let mut changed: bool = false;
        let initial_count = self.count_directory_files(path);

        while !changed {
            let count = self.count_directory_files(path);
            if count == initial_count {
                continue;
            } else {
                changed = true
            }
        }

        changed
    }
    fn count_directory_files(&mut self, path: &str) -> i64 {
        let mut count = 0;
        for entry in WalkDir::new(path).max_depth(1) {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    // Check if is file
                    if !path.is_dir() {
                        // Increment counter
                        count = count + 1;
                        // Store files if told to in configuration
                        if self.store_files {
                            match path.to_str() {
                                Some(file) => {
                                    // Add path to `files` vector
                                    self.files.push(String::from(file));
                                }
                                None => {
                                    println!("listen | Error adding files");
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        count
    }
}

fn internal_listener<T, R>(path: &str, _event: Event, arg: T, func: fn(T) -> R) {
    let mut listener = Listener::new();
    listener.listen(path, arg, func);
}
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

/// `ListenerOptions` contains the configuration details for
/// The `listen` function.
pub struct ListenerOptions<T, R> {
    /// The path to listen on
    path: &'static str,
    /// The event to listen for
    event: Event,
    /// The argument to pass to `func`
    arg: T,
    /// Function to execute when event happens
    func: fn(T) -> R,
}

/// This function takes a path to listen on, the event to listen for,
/// and a function to execute when that event happens.
pub fn listen<T, R>(path: &str, event: Event, arg: T, func: fn(T) -> R) {
    match event {
        Event::OnFileAdd => {
            if listener(Event::OnFileAdd, path) {
                func(arg);
            }
        }
        Event::OnFileRemove => {
            if listener(Event::OnFileRemove, path) {
                func(arg);
            }
        }
    }
}

fn listener(_event: Event, path: &str) -> bool {
    let mut changed: bool = false;
    let initial_count = count_directory_files(path);
    println!("initial count: {}", initial_count);

    while !changed {
        let count = count_directory_files(path);
        if count == initial_count {
            continue;
        } else {
            changed = true
        }
    }

    changed
}

fn count_directory_files(path: &str) -> i64 {
    let mut count = 0;
    for entry in WalkDir::new(path).max_depth(1) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                // Check if is file
                if !path.is_dir() {
                    // println!("Path: {:?}", path);
                    count = count + 1;
                    //     // Check if is CSV file and push to vector
                    //     if path.extension() == Some(OsStr::new("csv")) {
                    //         let path_str = String::from(path.to_str().unwrap());
                    //         studies.push(path_str);
                    //     }
                }
            }
            Err(_) => ()
        }
    }
    count
}
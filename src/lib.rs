use walkdir::WalkDir;

pub enum Event {
    OnFileAdd,
    OnFileRemove
}

pub struct ListenerOptions<T, R> {
    /// The path to listen on
    path: &'static str,
    /// The event to listen for
    event: Event,
    /// The argument to pass to `func`
    arg: T,
    /// Function to execute when event happens
    func: fn(T) -> R
}

/// This function takes a path to listen on, the event to listen for,
/// and a function to execute when that event happens.
pub fn listen<T, R>(path: &str, event: Event, arg: T, func: fn(T) -> R) {
    match event {
        Event::OnFileAdd=> {
            println!("listen for added file");
            listener(Event::OnFileAdd, path);
        },
        Event::OnFileRemove => {
            println!("listen for removed file");
            listener(Event::OnFileRemove, path);
        }
    }
}

fn listener(_event: Event, path: &str) {
    let mut changed: bool = false;
    let initial_count = count_directory_files(path);
    println!("initial count: {}", initial_count);

    while !changed {
        let count = count_directory_files(path);
        if count == initial_count {
            continue
        } else {
            changed = true
        }
    }
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
            },
            Err(_) => ()
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{listen, Event};

    #[test]
    fn test_listen() {
        // Create test directory
        fsutils::mkdir("test_directory");
        fn print_to_console(arg: &str) {
            println!("file added {}", arg);
        }
        let l = listen::<&str, ()>("test_directory", Event::OnFileAdd, "the function argument", print_to_console);
        // Clean up
        fsutils::rmdir("test_directory");
    }
}
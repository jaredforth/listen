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
    func(arg);
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
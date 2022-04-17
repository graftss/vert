pub mod input;
pub mod listener;

pub mod raw_input;
pub use raw_input::RawInputReader;

#[cfg(target_family = "windows")]
pub use raw_input::windows as raw_input_reader;

#[cfg(target_family = "unix")]
pub use raw_input::macos as raw_input_reader;

#[cfg(never)]
pub mod raw_input_test;

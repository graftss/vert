pub mod input;
pub use input::test_gamepad_system;

pub mod raw_input;
pub use raw_input::RawInputReader;

#[cfg(target_os = "windows")]
pub use raw_input::windows as raw_input_reader;

#[cfg(target_os = "macos")]
pub use raw_input::macos as raw_input_reader;

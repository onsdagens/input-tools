use std::time::Instant;

#[derive(Debug)]
pub struct Mouse {
    pub name: String,
    events: ,
}

#[derive(Debug)]
pub enum MouseEvent {
    MoveEvent(Instant, i32, i32), //timestamp x, y
    ButtonEvent(Instant),         //timestamp
}

#[derive(Debug)]
pub struct Keyboard {
    pub name: String,
}

#[derive(Debug)]
pub struct Devices {
    pub mice: Vec<Mouse>,
    pub keyboards: Vec<Keyboard>,
    poller: Option<bool>, //placeholder for Windows thread handle.
}

trait EventQueue {}

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

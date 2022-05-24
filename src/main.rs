#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod interface_terminal;
mod interface_gui;


fn main() {
    interface_gui::launch_gui()
    // interface_terminal::start_terminal();
}

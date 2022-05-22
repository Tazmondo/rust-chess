#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chess::*;
use eframe::egui;

pub fn launch_gui() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rust Chess",
        options,
        Box::new(|_cc| Box::new(State::default()))
    )
}

struct State {
    board: Board,
}

impl Default for State {
    fn default() -> Self {
        Self {
            board: Board::new()
        }
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let board = &self.board;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chess");
            ui.monospace(format!("{:?}", match board.pieces[0] {
                Space::Full(piece) => piece.variant,
                _ => {Piece::Queen}
            }))
        });
    }
}
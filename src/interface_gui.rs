#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chess::*;
use eframe::egui;
use eframe::egui::{InnerResponse, Vec2};
use egui_extras::RetainedImage;

pub fn launch_gui() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(480.0, 480.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "Rust Chess",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

struct Assets {
    white_pawn: RetainedImage,
    black_pawn: RetainedImage,
    white_bishop: RetainedImage,
    black_bishop: RetainedImage,
    white_king: RetainedImage,
    black_king: RetainedImage,
    white_knight: RetainedImage,
    black_knight: RetainedImage,
    white_rook: RetainedImage,
    black_rook: RetainedImage,
    white_queen: RetainedImage,
    black_queen: RetainedImage,
    empty: RetainedImage,
}

impl Default for Assets {
    fn default() -> Self {
        Self {
            white_pawn: RetainedImage::from_image_bytes("white_pawn", include_bytes!("../assets/white_pawn.png")).unwrap(),
            black_pawn: RetainedImage::from_image_bytes("black_pawn", include_bytes!("../assets/black_pawn.png")).unwrap(),
            white_bishop: RetainedImage::from_image_bytes("white_bishop", include_bytes!("../assets/white_bishop.png")).unwrap(),
            black_bishop: RetainedImage::from_image_bytes("black_bishop", include_bytes!("../assets/black_bishop.png")).unwrap(),
            white_knight: RetainedImage::from_image_bytes("white_knight", include_bytes!("../assets/white_knight.png")).unwrap(),
            black_knight: RetainedImage::from_image_bytes("black_knight", include_bytes!("../assets/black_knight.png")).unwrap(),
            white_rook: RetainedImage::from_image_bytes("white_rook", include_bytes!("../assets/white_rook.png")).unwrap(),
            black_rook: RetainedImage::from_image_bytes("black_rook", include_bytes!("../assets/black_rook.png")).unwrap(),
            white_king: RetainedImage::from_image_bytes("white_king", include_bytes!("../assets/white_king.png")).unwrap(),
            black_king: RetainedImage::from_image_bytes("black_king", include_bytes!("../assets/black_king.png")).unwrap(),
            white_queen: RetainedImage::from_image_bytes("white_queen", include_bytes!("../assets/white_queen.png")).unwrap(),
            black_queen: RetainedImage::from_image_bytes("black_queen", include_bytes!("../assets/black_queen.png")).unwrap(),
            empty: RetainedImage::from_image_bytes("empty", include_bytes!("../assets/empty.png")).unwrap(),
        }
    }
}

struct App {
    board: Board,
    assets: Assets,
    selected: Option<Square>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Board::new(),
            assets: Assets::default(),
            selected: None,
        }
    }
}

impl App {
    fn get_asset(&self, space: &Space) -> &RetainedImage {
        match space {
            Space::Full(piece) => match piece {
                ColourPiece { variant: Piece::Pawn, colour: Colour::White } => &self.assets.white_pawn,
                ColourPiece { variant: Piece::Pawn, colour: Colour::Black } => &self.assets.black_pawn,
                ColourPiece { variant: Piece::Knight, colour: Colour::White } => &self.assets.white_knight,
                ColourPiece { variant: Piece::Knight, colour: Colour::Black } => &self.assets.black_knight,
                ColourPiece { variant: Piece::Bishop, colour: Colour::White } => &self.assets.white_bishop,
                ColourPiece { variant: Piece::Bishop, colour: Colour::Black } => &self.assets.black_bishop,
                ColourPiece { variant: Piece::Rook, colour: Colour::White } => &self.assets.white_rook,
                ColourPiece { variant: Piece::Rook, colour: Colour::Black } => &self.assets.black_rook,
                ColourPiece { variant: Piece::King, colour: Colour::White } => &self.assets.white_king,
                ColourPiece { variant: Piece::King, colour: Colour::Black } => &self.assets.black_king,
                ColourPiece { variant: Piece::Queen, colour: Colour::White } => &self.assets.white_queen,
                ColourPiece { variant: Piece::Queen, colour: Colour::Black } => &self.assets.black_queen,
            }
            Space::Empty => &self.assets.empty
        }
    }

    fn render_board(&self, ctx: &egui::Context, ui: &mut egui::Ui) -> InnerResponse<()> {
        let pieces = &self.board.pieces;

        egui::Grid::new("board")
            .spacing(Vec2::new(0.0, 0.0))
            .show(ui, |ui| {
                ui.style_mut().visuals.widgets.inactive.rounding = egui::Rounding::none();
                ui.style_mut().visuals.widgets.hovered.rounding = egui::Rounding::none();
                ui.style_mut().visuals.widgets.hovered.expansion = 2.0;
                ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::new(4.0, egui::Color32::from_rgb(0, 170, 0));

                pieces.iter().enumerate().for_each(|(index, space)| {
                    if index % 8 == 0 && index > 0 {
                        ui.end_row();
                    }

                    let selected_index = self.selected.map_or(-1, |square| square.index);

                    let piece_size = Vec2::new(50.0, 50.0);

                    let is_enabled = true; // For future conditional disabling

                    ui.add_visible_ui(true, |ui| {
                        if index % 2 == ((index / 8) % 2) {
                            ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(165);
                            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_gray(165);
                        } else {
                            ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(80);
                            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_gray(80);
                        }

                        ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.get_asset(space).texture_id(ctx), piece_size));
                    });
                })
            })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let board = &self.board;

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_board(ctx, ui);
        });
    }
}
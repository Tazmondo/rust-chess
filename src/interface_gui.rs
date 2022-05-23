#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chess::*;
use eframe::egui;
use eframe::egui::{InnerResponse, Vec2};
use egui_extras::RetainedImage;
use winapi_util::console::Intense::No;

pub fn launch_gui() {
    let options = eframe::NativeOptions::default();

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
    fn render_board(&self, ctx: &egui::Context, ui: &mut egui::Ui) -> InnerResponse<()> {
        let pieces = self.board.pieces;
        egui::Grid::new("board")
            .spacing(Vec2::new(0.0, 0.0))
            .show(ui, |ui| {
                pieces.iter().enumerate().for_each(|(index, piece)| {
                    if index % 8 == 0 && index > 0 {
                        ui.end_row();
                    }

                    let selected_index = self.selected.map_or(-1, |square| square.index);

                    let piece_size = Vec2::new(50.0, 50.0);

                    let is_enabled = true; // For future conditional disabling

                    match piece {
                        Space::Full(piece) => match piece {
                            ColourPiece { variant: Piece::Pawn, colour } => match colour {
                                Colour::White => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.white_pawn.texture_id(ctx), piece_size)); }
                                Colour::Black => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.black_pawn.texture_id(ctx), piece_size)); }
                            },
                            ColourPiece { variant: Piece::Bishop, colour } => match colour {
                                Colour::White => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.white_bishop.texture_id(ctx), piece_size)); }
                                Colour::Black => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.black_bishop.texture_id(ctx), piece_size)); }
                            },
                            ColourPiece { variant: Piece::Knight, colour } => match colour {
                                Colour::White => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.white_knight.texture_id(ctx), piece_size)); }
                                Colour::Black => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.black_knight.texture_id(ctx), piece_size)); }
                            },
                            ColourPiece { variant: Piece::Rook, colour } => match colour {
                                Colour::White => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.white_rook.texture_id(ctx), piece_size)); }
                                Colour::Black => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.black_rook.texture_id(ctx), piece_size)); }
                            },
                            ColourPiece { variant: Piece::King, colour } => match colour {
                                Colour::White => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.white_king.texture_id(ctx), piece_size)); }
                                Colour::Black => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.black_king.texture_id(ctx), piece_size)); }
                            },
                            ColourPiece { variant: Piece::Queen, colour } => match colour {
                                Colour::White => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.white_queen.texture_id(ctx), piece_size)); }
                                Colour::Black => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.black_queen.texture_id(ctx), piece_size)); }
                            },
                        },
                        Space::Empty => { ui.add_enabled(is_enabled, egui::widgets::ImageButton::new(self.assets.empty.texture_id(ctx), piece_size)); }
                    };
                })
            })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let board = &self.board;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chess");
            self.render_board(ctx, ui);
        });
    }
}
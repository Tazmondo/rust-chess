use std::cell::RefCell;
use std::ops::{Add, Sub};
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

fn drop_target<R>(
    ui: &mut egui::Ui,
    can_accept_what_is_being_dragged: bool,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> InnerResponse<R> {
    let is_being_dragged = ui.memory().is_anything_being_dragged();

    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds;
    let where_to_put_background = ui.painter().add(egui::Shape::Noop);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    let ret = body(&mut content_ui);
    let outer_rect = egui::Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), egui::Sense::hover());

    let style = if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
        ui.visuals().widgets.active
    } else {
        ui.visuals().widgets.inactive
    };

    let mut fill = style.bg_fill;
    let mut stroke = style.bg_stroke;
    if is_being_dragged && !can_accept_what_is_being_dragged {
        // gray out:
        fill = egui::color::tint_color_towards(fill, ui.visuals().window_fill());
        stroke.color = egui::color::tint_color_towards(stroke.color, ui.visuals().window_fill());
    }

    ui.painter().set(
        where_to_put_background,
        egui::epaint::RectShape {
            rounding: style.rounding,
            fill,
            stroke,
            rect,
        },
    );

    InnerResponse::new(ret, response)
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

fn colour_from_index(index: usize) -> egui::Color32 {
    match index % 2 == ((index / 8) % 2) {
        true => egui::Color32::from_gray(165),
        false => egui::Color32::from_gray(80)
    }
}

fn set_piece_style(style: &mut egui::Style, enabled: bool, index: usize) {
    let mut colour = colour_from_index(index);

    style.visuals.widgets.inactive.rounding = egui::Rounding::none();
    style.visuals.widgets.hovered.rounding = egui::Rounding::none();
    style.visuals.widgets.active.rounding = egui::Rounding::none();

    if enabled {
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 150, 0));
        style.visuals.widgets.hovered.expansion = 2.0;
        colour = egui::Color32::from_rgb(colour.r() - 15, colour.g() + 40, colour.b() - 15);
        style.visuals.widgets.active = style.visuals.widgets.hovered;
    } else {
        style.visuals.widgets.hovered = style.visuals.widgets.inactive;
        style.visuals.widgets.hovered.expansion = 4.0;
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(0.0, egui::Color32::WHITE);
        // style.visuals.widgets.active = style.visuals.widgets.hovered;
        style.visuals.widgets.active.expansion = 2.75;
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(0.0, egui::Color32::WHITE);
    }

    style.visuals.widgets.inactive.bg_fill = colour;
    style.visuals.widgets.hovered.bg_fill = colour;
    style.visuals.widgets.active.bg_fill = colour;
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

    fn render_board(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> InnerResponse<()> {
        let board = &mut self.board;
        let selected_square_id = egui::Id::new("selected_square");

        let selected_square_option: Option<Square> = ctx.data().get_temp(selected_square_id);
        let selected_pieces: Option<Vec<Square>> = match selected_square_option {
            Some(square) => board.get_square_moves(square).map(|move_vec| move_vec.into_iter().map(|_move| _move.end).collect()),
            None => None
        };

        let pieces = board.pieces;

        egui::Grid::new("board")
            .spacing(Vec2::new(0.0, 0.0))
            .show(ui, |ui| {
                pieces.iter().enumerate().for_each(|(index, space)| {
                    if index % 8 == 0 && index > 0 {
                        ui.end_row();
                    }

                    let selected_index = self.selected.map_or(-1, |square| square.index);

                    let piece_size = Vec2::new(50.0, 50.0);

                    let square = Square::from_index(index as i32);
                    let piece_id = egui::Id::new(square.index);

                    let is_enabled = match &selected_pieces {
                        Some(moves) => moves.contains(&square),
                        None => self.board.can_square_move(space, &square)
                    };

                    // So that each button can have a different style.
                    // There may be a better way of doing this
                    set_piece_style(ui.style_mut(), is_enabled, index);

                    let board_layer = ui.layer_id();

                    let response = if ui.memory().is_being_dragged(piece_id) {
                        let layer_id = egui::LayerId::new(egui::Order::Tooltip, piece_id);
                        let response = ui.with_layer_id(layer_id, |ui| ui.add(egui::widgets::ImageButton::new(self.get_asset(space).texture_id(ctx), piece_size).sense(egui::Sense::drag()).frame(false))).response;

                        // Now we move the visuals of the body to where the mouse is.
                        // Normally you need to decide a location for a widget first,
                        // because otherwise that widget cannot interact with the mouse.
                        // However, a dragged component cannot be interacted with anyway
                        // (anything with `Order::Tooltip` always gets an empty [`Response`])
                        // So this is fine!

                        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                            let delta = pointer_pos - response.rect.center();
                            ui.ctx().translate_layer(layer_id, delta);
                            ui.ctx().layer_painter(board_layer).rect_filled(egui::Rect::from_min_max(response.rect.min, response.rect.min.add(piece_size + Vec2::splat(8.0))), egui::Rounding::none(), colour_from_index(index));
                        }
                        response

                    } else if ui.memory().is_anything_being_dragged() && is_enabled {
                        drop_target(ui, is_enabled, |ui| {
                            ui.add(egui::widgets::ImageButton::new(self.get_asset(space).texture_id(ctx), piece_size).sense(egui::Sense::drag()))
                        }).response
                    } else {
                        ui.add(egui::widgets::ImageButton::new(self.get_asset(space).texture_id(ctx), piece_size).sense(egui::Sense::drag()))
                    };

                    if response.drag_started() && is_enabled {
                        ui.memory().set_dragged_id(piece_id);
                        ctx.data().insert_temp(selected_square_id, square);
                    }
                    if ui.input().pointer.any_released() && response.hovered() && is_enabled {
                        match ctx.data().get_temp::<Square>(selected_square_id) {
                            Some(start_square) => {
                                let new_move = Move {
                                    piece: self.board.piece_at_coord(&start_square.coord).unwrap(),
                                    start: start_square,
                                    end: square
                                };
                                let result = self.board.move_piece(new_move);
                                match result {
                                    Ok(state) => println!("{state:?}"),
                                    Err(msg) => println!("{msg}")
                                };
                            }
                            None => {}
                        }
                    }

                    if !ui.memory().is_anything_being_dragged() {
                        ctx.data().remove::<Square>(selected_square_id);
                    }

                });
            })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let board = &self.board;

        let response = egui::CentralPanel::default().show(ctx, |ui| {
            self.render_board(ctx, ui)
        });
    }
}
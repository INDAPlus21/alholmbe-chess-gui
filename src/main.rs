use eliasfl_chess::{Color as Colour, Game, GameState, Piece as PieceType, Position};
/**
 * Chess GUI template.
 * Author: Viola Söderlund <violaso@kth.se>
 * Last updated: 2021-10-03
 */
use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use std::{collections::HashMap, env, path};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;

const BOTTOM_BOARD_BORDER: f32 = 715.0;

const MENU_SIZE: i16 = 3;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    (GRID_SIZE + MENU_SIZE) as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color =
    graphics::Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: graphics::Color =
    graphics::Color::new(188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);
const GREENWHITE: graphics::Color =
    graphics::Color::new(188.0 / 255.0, 255.0 / 255.0, 76.0 / 255.0, 1.0);
const GREENBLACK: graphics::Color =
    graphics::Color::new(228.0 / 255.0, 255.0 / 255.0, 108.0 / 255.0, 1.0);

// text color representations
const GREEN_TEXT: [f32; 4] = [0.0, 255.0, 0.0, 1.1];
const BLACK_TEXT: [f32; 4] = [0.0, 0.0, 0.0, 1.1];
/// GUI logic and event implementation structure.
struct AppState {
    sprites: HashMap<(Colour, PieceType), graphics::Image>,
    game: Game, // Save piece positions, which tiles has been clicked, current colour, etc...
    currently_clicked: String,
    moves_for_clicked: Vec<String>,
}

impl AppState {
    fn get_currently_clicked(&self) -> String {
        self.currently_clicked.clone()
    }
    fn get_moves_for_clicked(&self) -> Vec<String> {
        self.moves_for_clicked.clone()
    }
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            game: Game::new(),
            currently_clicked: String::from(""),
            moves_for_clicked: vec![],
        };

        Ok(state)
    }

    /// Loads chess piece images into vector.
    fn load_sprites(ctx: &mut Context) -> HashMap<(Colour, PieceType), graphics::Image> {
        [
            (
                (Colour::Black, PieceType::King(Colour::Black)),
                "/black_king.png".to_string(),
            ),
            (
                (Colour::Black, PieceType::Queen(Colour::Black)),
                "/black_queen.png".to_string(),
            ),
            (
                (Colour::Black, PieceType::Rook(Colour::Black)),
                "/black_rook.png".to_string(),
            ),
            (
                (Colour::Black, PieceType::Pawn(Colour::Black)),
                "/black_pawn.png".to_string(),
            ),
            (
                (Colour::Black, PieceType::Bishop(Colour::Black)),
                "/black_bishop.png".to_string(),
            ),
            (
                (Colour::Black, PieceType::Knight(Colour::Black)),
                "/black_knight.png".to_string(),
            ),
            (
                (Colour::White, PieceType::King(Colour::White)),
                "/white_king.png".to_string(),
            ),
            (
                (Colour::White, PieceType::Queen(Colour::White)),
                "/white_queen.png".to_string(),
            ),
            (
                (Colour::White, PieceType::Rook(Colour::White)),
                "/white_rook.png".to_string(),
            ),
            (
                (Colour::White, PieceType::Pawn(Colour::White)),
                "/white_pawn.png".to_string(),
            ),
            (
                (Colour::White, PieceType::Bishop(Colour::White)),
                "/white_bishop.png".to_string(),
            ),
            (
                (Colour::White, PieceType::Knight(Colour::White)),
                "/white_knight.png".to_string(),
            ),
        ]
        .iter()
        .map(|(_piece, _path)| (*_piece, graphics::Image::new(ctx, _path).unwrap()))
        .collect::<HashMap<(Colour, PieceType), graphics::Image>>()
    }
}

fn coordinates_to_string(row: usize, col: usize) -> String {
    let rank = (7 - row + 1).to_string();
    let file = match col + 1 {
        1 => "a",
        2 => "b",
        3 => "c",
        4 => "d",
        5 => "e",
        6 => "f",
        7 => "g",
        8 => "h",
        _ => panic!(),
    };
    format!("{}{}", file, rank)
}

impl event::EventHandler<GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Game is {:?}.", self.game.get_game_state()))
                .scale(graphics::PxScale { x: 35.0, y: 35.0 }),
        );

        // draw the state of the game
        graphics::draw(
            ctx,
            &state_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: 10 as f32,
                    y: (GRID_CELL_SIZE.1 * 8 + 45) as f32,
                }),
        )?;
        let turn_text = graphics::Text::new(
            graphics::TextFragment::from(format!("{:?}:s turn", self.game.active_color))
                .scale(graphics::PxScale { x: 35.0, y: 35.0 }),
        );

        // draw who's turn it is
        graphics::draw(
            ctx,
            &turn_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: 10 as f32,
                    y: (GRID_CELL_SIZE.1 * 8 + 80) as f32,
                }),
        )?;

        let reset_game_text = graphics::Text::new(
            graphics::TextFragment::from("RESET").scale(graphics::PxScale { x: 35.0, y: 35.0 }),
        );
        // draw reset_game_text
        graphics::draw(
            ctx,
            &reset_game_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: 10 as f32,
                    y: (GRID_CELL_SIZE.1 * 8 + 10) as f32,
                }),
        )?;

        // promotion stuff
        let promote_to_text = graphics::Text::new(
            graphics::TextFragment::from("Piece you want to promote to:")
                .scale(graphics::PxScale { x: 35.0, y: 35.0 }),
        );

        graphics::draw(
            ctx,
            &promote_to_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: 10 as f32,
                    y: (GRID_CELL_SIZE.0 * 8 + 120) as f32,
                }),
        )?;

        let queen_text = graphics::Text::new(
            graphics::TextFragment::from("Queen").scale(graphics::PxScale { x: 25.0, y: 25.0 }),
        );
        let bishop_text = graphics::Text::new(
            graphics::TextFragment::from("Bishop").scale(graphics::PxScale { x: 25.0, y: 25.0 }),
        );
        let rook_text = graphics::Text::new(
            graphics::TextFragment::from("Rook").scale(graphics::PxScale { x: 25.0, y: 25.0 }),
        );
        let knight_text = graphics::Text::new(
            graphics::TextFragment::from("Knight").scale(graphics::PxScale { x: 25.0, y: 25.0 }),
        );

        let mut queen_text_colour = BLACK_TEXT;
        let mut bishop_text_colour = BLACK_TEXT;
        let mut rook_text_colour = BLACK_TEXT;
        let mut knight_text_colour = BLACK_TEXT;

        if self.game.active_color == Colour::White {
            match self.game.promotion[0] {
                PieceType::Queen(_) => queen_text_colour = GREEN_TEXT,
                PieceType::Bishop(_) => bishop_text_colour = GREEN_TEXT,
                PieceType::Rook(_) => rook_text_colour = GREEN_TEXT,
                PieceType::Knight(_) => knight_text_colour = GREEN_TEXT,
                _ => {}
            }
        } else {
            match self.game.promotion[1] {
                PieceType::Queen(_) => queen_text_colour = GREEN_TEXT,
                PieceType::Bishop(_) => bishop_text_colour = GREEN_TEXT,
                PieceType::Rook(_) => rook_text_colour = GREEN_TEXT,
                PieceType::Knight(_) => knight_text_colour = GREEN_TEXT,
                _ => {}
            }
        }

        graphics::draw(
            ctx,
            &queen_text,
            graphics::DrawParam::default()
                .color(queen_text_colour.into())
                .dest(ggez::mint::Point2 {
                    x: 30 as f32,
                    y: (GRID_CELL_SIZE.0 * 8 + 170) as f32,
                }),
        )?;
        graphics::draw(
            ctx,
            &bishop_text,
            graphics::DrawParam::default()
                .color(bishop_text_colour.into())
                .dest(ggez::mint::Point2 {
                    x: (30 + 50 * 2) as f32,
                    y: (GRID_CELL_SIZE.0 * 8 + 170) as f32,
                }),
        )?;
        graphics::draw(
            ctx,
            &rook_text,
            graphics::DrawParam::default()
                .color(rook_text_colour.into())
                .dest(ggez::mint::Point2 {
                    x: (30 + 50 * 4 + 15) as f32,
                    y: (GRID_CELL_SIZE.0 * 8 + 170) as f32,
                }),
        )?;
        graphics::draw(
            ctx,
            &knight_text,
            graphics::DrawParam::default()
                .color(knight_text_colour.into())
                .dest(ggez::mint::Point2 {
                    x: (30 + 50 * 6) as f32,
                    y: (GRID_CELL_SIZE.0 * 8 + 170) as f32,
                }),
        )?;

        // draw grid
        for _row in (0..8).rev() {
            for _col in (0..8).rev() {
                // check if in possible moves
                let mut is_a_move = false;
                let tile = coordinates_to_string(_row, _col);
                for mv in self.get_moves_for_clicked() {
                    if mv == tile {
                        is_a_move = true;
                    }
                }
                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        _col as i32 * GRID_CELL_SIZE.0 as i32,
                        _row as i32 * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    match _col % 2 {
                        0 => {
                            if _row % 2 == 0 {
                                if is_a_move {
                                    GREENWHITE
                                } else {
                                    WHITE
                                }
                            } else {
                                if is_a_move {
                                    GREENBLACK
                                } else {
                                    BLACK
                                }
                            }
                        }
                        _ => {
                            if _row % 2 == 0 {
                                if is_a_move {
                                    GREENBLACK
                                } else {
                                    BLACK
                                }
                            } else {
                                if is_a_move {
                                    GREENWHITE
                                } else {
                                    WHITE
                                }
                            }
                        }
                    },
                )
                .expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw tiles.");

                // draw piece
                let pos = Position {
                    file: (_col + 1) as u8,
                    rank: (7 - _row + 1) as u8,
                };

                if let Some(_piece) = self.game.board.get(&pos) {
                    let colour = match _piece {
                        PieceType::King(_colour)
                        | PieceType::Queen(_colour)
                        | PieceType::Rook(_colour)
                        | PieceType::Bishop(_colour)
                        | PieceType::Knight(_colour)
                        | PieceType::Pawn(_colour) => _colour,
                    };
                    graphics::draw(
                        ctx,
                        // Colour, PieceType
                        self.sprites.get(&(*colour, *_piece)).unwrap(),
                        graphics::DrawParam::default()
                            .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                            .dest([
                                _col as f32 * GRID_CELL_SIZE.0 as f32,
                                _row as f32 * GRID_CELL_SIZE.1 as f32,
                            ]),
                    )
                    .expect("Failed to draw piece.");
                }
            }
        }

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == event::MouseButton::Left {
            if y > BOTTOM_BOARD_BORDER {
                if y > 895.0 && y < 913.0 {
                    if x > 29.0 && x < 97.0 {
                        self.game.set_promotion(String::from("queen")).unwrap();
                    } else if x > 132.0 && x < 210.0 {
                        self.game.set_promotion(String::from("bishop")).unwrap();
                    } else if x > 247.0 && x < 300.0 {
                        self.game.set_promotion(String::from("rook")).unwrap();
                    } else if x > 331.0 && x < 410.0 {
                        self.game.set_promotion(String::from("knight")).unwrap();
                    }
                }
                if y > 735.0 && y < 758.0 && x > 11.0 && x < 103.0 {
                    self.game = Game::new();
                    self.moves_for_clicked = vec![];
                    self.currently_clicked = String::from("");
                }
                return;
            }

            let row = (y / GRID_CELL_SIZE.0 as f32).floor() as usize;
            let col = (x / GRID_CELL_SIZE.1 as f32).floor() as usize;
            if self.currently_clicked != String::from("") {
                let move_to = coordinates_to_string(row, col);
                if self.currently_clicked == move_to {
                    self.currently_clicked = String::from("");
                    self.moves_for_clicked = vec![];
                    return;
                }
                // try to make a move
                for mv in self.get_moves_for_clicked() {
                    if mv == move_to {
                        // make move
                        let res = self.game.make_move(self.get_currently_clicked(), mv);
                        match res {
                            Ok(_) => {}
                            Err(_) => {
                                // handle illegal input error
                                println!("Cant move opponents piece!");
                                self.currently_clicked = String::from("");
                                return;
                            }
                        };

                        self.currently_clicked = String::from("");
                        self.moves_for_clicked = vec![];
                    };
                }
            } else {
                let position = coordinates_to_string(row, col);
                /* check click position and update board accordingly */
                let piece = self.game.board.get(&Position {
                    file: (col + 1) as u8,
                    rank: (7 - row + 1) as u8,
                });
                let piece = match piece {
                    Some(p) => p,
                    None => {
                        return;
                    }
                };
                let colour = match piece {
                    PieceType::King(_colour)
                    | PieceType::Queen(_colour)
                    | PieceType::Rook(_colour)
                    | PieceType::Bishop(_colour)
                    | PieceType::Knight(_colour)
                    | PieceType::Pawn(_colour) => _colour,
                };
                if *colour != self.game.active_color {
                    return;
                }
                self.currently_clicked = position;
                let moves = self.game.get_possible_moves(self.get_currently_clicked());
                let moves = match moves {
                    Some(mvs) => mvs,
                    None => vec![],
                };
                if moves.len() == 0 {
                    self.currently_clicked = String::new();
                }
                self.moves_for_clicked = moves;
            }
        }
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    // config stuff
    let context_builder = ContextBuilder::new("schack", "alexander")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()
                .title("Scharre") // Set window title "Schack"
                .icon("/icon.png"), // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fixate window size
        );
    // build the context
    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}

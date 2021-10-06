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
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color =
    graphics::Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: graphics::Color =
    graphics::Color::new(188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);

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
        // clear interface with gray background colour
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // create text representation
        let turn_text = graphics::Text::new(
            graphics::TextFragment::from(format!("{:?}:s turn", self.game.active_color))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Game is {:?}.", self.game.get_game_state()))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions_state = state_text.dimensions(ctx);
        let text_dimensions_turn = turn_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (SCREEN_SIZE.0 - text_dimensions_state.w as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions_state.h as f32) / 2f32 as f32,
                text_dimensions_state.w as f32 + 16.0,
                text_dimensions_state.h as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        let background_box_2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (SCREEN_SIZE.0 - text_dimensions_turn.w as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions_turn.h as f32) / 2f32 as f32,
                text_dimensions_turn.w as f32 + 16.0,
                text_dimensions_turn.h as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default())
            .expect("Failed to draw background.");
        graphics::draw(ctx, &background_box_2, graphics::DrawParam::default())
            .expect("Failed to draw background.");

        // draw grid
        for _row in (0..8).rev() {
            for _col in (0..8).rev() {
                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        _col * GRID_CELL_SIZE.0 as i32,
                        _row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    match _col % 2 {
                        0 => {
                            if _row % 2 == 0 {
                                WHITE
                            } else {
                                BLACK
                            }
                        }
                        _ => {
                            if _row % 2 == 0 {
                                BLACK
                            } else {
                                WHITE
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
                //println!("pos = {:?}, col = {}, row = {}", pos, _col, _row);
                if let Some(_piece) = self.game.board.get(&pos) {
                    //println!("piece = {:?}", _piece);
                    let colour = match _piece {
                        PieceType::King(_colour)
                        | PieceType::Queen(_colour)
                        | PieceType::Rook(_colour)
                        | PieceType::Bishop(_colour)
                        | PieceType::Knight(_colour)
                        | PieceType::Pawn(_colour) => _colour,
                    };
                    //println!("colour = {:?}", colour);
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

        // draw text with dark gray colouring and center position
        graphics::draw(
            ctx,
            &state_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions_state.w as f32) / 2f32 as f32,
                    y: (SCREEN_SIZE.0 - 50 as f32 - text_dimensions_state.h as f32) / 2f32 as f32,
                }),
        )
        .expect("Failed to draw text.");
        // draw text with dark gray colouring and center position
        graphics::draw(
            ctx,
            &turn_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions_turn.w as f32) / 2f32 as f32,
                    y: (SCREEN_SIZE.0 - text_dimensions_turn.h as f32) / 2f32 as f32,
                }),
        )
        .expect("Failed to draw text.");

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == event::MouseButton::Left {
            /* check click position and update board accordingly */
            // find click position
            // call make_move
            println!("up x = {}, up y = {}", x, y);
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == event::MouseButton::Left {
            //println!("ctx = {:?}", ctx);

            let row = (y / GRID_CELL_SIZE.0 as f32).floor() as usize;
            let col = (x / GRID_CELL_SIZE.1 as f32).floor() as usize;
            if self.currently_clicked != String::from("") {
                let move_to = coordinates_to_string(row, col);
                println!("move_to = {}", move_to);
                // try to make a move
                for mv in self.get_moves_for_clicked() {
                    if mv == move_to {
                        println!("legit move = {}", mv);
                        // make move
                        let res = self.game.make_move(self.get_currently_clicked(), mv);
                        println!("res = {:?}", res);
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
                        println!(
                            "currently clicked after move = {}",
                            self.get_currently_clicked()
                        );
                    };
                }
            } else {
                let position = coordinates_to_string(row, col);
                /* check click position and update board accordingly */
                // find click position
                // update state with currently clicked position
                // CHECK IF THE PIECE HERE IS OF THE RIGHT COLOUR
                self.currently_clicked = position;
                // call get possible moves
                let moves = self.game.get_possible_moves(self.get_currently_clicked());
                let moves = match moves {
                    Some(mvs) => mvs,
                    None => vec![],
                };
                if moves.len() == 0 {
                    self.currently_clicked = String::new();
                }
                self.moves_for_clicked = moves;
                println!("Currently Clicked = {:?}", self.get_currently_clicked());
                println!("moves = {:?}", self.get_moves_for_clicked());
                // paint the board with possible moves
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

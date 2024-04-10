use crate::{
    constants::{
        BLUE, DOTS, ONE_EYED, ORANGE, PIECE_BLUE_LAB, PIECE_INVERT_COLOR, PIECE_INVERT_PATTERN,
        PIECE_INVERT_SHAPE, PIECE_RED_LAB, PIECE_VENT, PIECE_YELLOW_LAB, STRIPES, TWO_EYED,
    },
    game::PanicBoard,
};

pub mod constants;
pub mod game;

fn main() {
    let board = generate_board();

    let mut panic_board = PanicBoard::new(board);

    let result = panic_board.check_amoeba_pos(false, PIECE_YELLOW_LAB, ONE_EYED, STRIPES, ORANGE);

    println!("Result: {result:#?}");
}

fn generate_board() -> [u8; 25] {
    let all_pieces: [u8; 25] = [
        PIECE_RED_LAB,
        PanicBoard::build_current_amoeba(TWO_EYED, BLUE, DOTS),
        PIECE_INVERT_PATTERN,
        PanicBoard::build_current_amoeba(TWO_EYED, STRIPES, ORANGE),
        PIECE_VENT,
        PanicBoard::build_current_amoeba(TWO_EYED, STRIPES, BLUE),
        PIECE_INVERT_SHAPE,
        PanicBoard::build_current_amoeba(ONE_EYED, DOTS, BLUE),
        PanicBoard::build_current_amoeba(ONE_EYED, ORANGE, DOTS),
        PIECE_YELLOW_LAB,
        PanicBoard::build_current_amoeba(TWO_EYED, BLUE, STRIPES),
        PanicBoard::build_current_amoeba(ONE_EYED, ORANGE, STRIPES),
        PIECE_VENT,
        PanicBoard::build_current_amoeba(TWO_EYED, ORANGE, DOTS),
        PIECE_INVERT_COLOR,
        PanicBoard::build_current_amoeba(ONE_EYED, BLUE, STRIPES),
        PanicBoard::build_current_amoeba(ONE_EYED, DOTS, BLUE),
        PanicBoard::build_current_amoeba(TWO_EYED, STRIPES, ORANGE),
        PanicBoard::build_current_amoeba(TWO_EYED, BLUE, DOTS),
        PIECE_BLUE_LAB,
        PanicBoard::build_current_amoeba(ONE_EYED, STRIPES, BLUE),
        PanicBoard::build_current_amoeba(TWO_EYED, ORANGE, DOTS),
        PanicBoard::build_current_amoeba(ONE_EYED, STRIPES, ORANGE),
        PanicBoard::build_current_amoeba(ONE_EYED, DOTS, ORANGE),
        PIECE_VENT,
    ];

    all_pieces
}

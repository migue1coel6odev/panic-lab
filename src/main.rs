use crate::game::PanicBoard;

pub mod constants;
pub mod game;

fn main() {

    let board = generate_board();

    let mut panic_board = PanicBoard::new(board);

    panic_board.set_current_pos(9);
    panic_board.set_current_amoeba(0b0000_0101);
    panic_board.set_clockwise(false);

    let result = panic_board.check_amoeba_pos();

    println!("Result: {result:#?}");

}


fn generate_board() -> [u8; 25] {
    let all_pieces: [u8; 25] = [
        constants::PIECE_RED_LAB,
        0b0000_0111,
        constants::PIECE_INVERT_PATTERN,
        0b0000_0100,
        constants::PIECE_VENT,
        0b0000_0110,
        constants::PIECE_INVERT_SHAPE,
        0b0000_0011,
        0b0000_0001,
        constants::PIECE_YELLOW_LAB,
        0b0000_0110,
        0b0000_0000,
        constants::PIECE_VENT,
        0b0000_0101,
        constants::PIECE_INVERT_COLOR,
        0b0000_0010,
        0b0000_0011,
        0b0000_0100, 
        0b0000_0111,
        constants::PIECE_BLUE_LAB,
        0b0000_0010,
        0b0000_0101,
        0b0000_0000,
        0b0000_0001,
        constants::PIECE_VENT,
    ];

    all_pieces

}

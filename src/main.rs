use rand::rngs::mock::StepRng;
use shuffle::{irs::Irs, shuffler::Shuffler};

pub mod constants;
pub mod game;

fn main() {
    let mut peca: u8 = 0b0000_0111;

    peca ^= constants::INVERT_COLOR;
    println!("New piece: {peca:08b}");

    generate_board();
}


fn generate_board() {
    // let vent_pieces = [constants::PIECE_VENT; 3];
    // let lab_pieces = [
    //     constants::PIECE_BLUE_LAB,
    //     constants::PIECE_RED_LAB,
    //     constants::PIECE_YELLOW_LAB,
    // ];
    // let amoebas_pieces_one_eyed: [u8; 4] = [0b0000_0000, 0b0000_0001, 0b0000_0010, 0b0000_0011];
    // let amoebas_pieces_two_eyed: [u8; 4] = [0b0000_0100, 0b0000_0101, 0b0000_0110, 0b0000_0111];

    let all_pieces: [u8; 25] = [
        constants::PIECE_VENT,
        constants::PIECE_VENT,
        constants::PIECE_VENT,
        constants::PIECE_BLUE_LAB,
        constants::PIECE_RED_LAB,
        constants::PIECE_YELLOW_LAB,
        constants::PIECE_INVERT_COLOR,
        constants::PIECE_INVERT_PATTERN,
        constants::PIECE_INVERT_SHAPE,
        0b0000_0000,
        0b0000_0000,
        0b0000_0001,
        0b0000_0001,
        0b0000_0010,
        0b0000_0010,
        0b0000_0011,
        0b0000_0011,
        0b0000_0100,
        0b0000_0100, 
        0b0000_0101,
        0b0000_0101,
        0b0000_0110,
        0b0000_0110,
        0b0000_0111,
        0b0000_0111,
    ];

    println!("{all_pieces:#?}");
    
    let mut rng = StepRng::new(2, 100);
    let mut irs = Irs::default();

    let mut all_pieces = all_pieces.to_vec();

    irs.shuffle(&mut all_pieces, &mut rng).unwrap();

    println!("{all_pieces:?}");

}

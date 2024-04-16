use crate::constants::{
    INVERT_COLOR, INVERT_PATTERN, INVERT_SHAPE, PIECE_BLUE_LAB, PIECE_INVERT_COLOR,
    PIECE_INVERT_PATTERN, PIECE_INVERT_SHAPE, PIECE_RED_LAB, PIECE_VENT, PIECE_YELLOW_LAB,
};

pub struct PanicBoard {
    pub board: [u8; 25],
}

impl PanicBoard {
    pub fn new(board: [u8; 25]) -> PanicBoard {
        PanicBoard { board }
    }

    pub fn check_amoeba_pos(
        &mut self,
        is_clockwise: bool,
        initial_lab: u8,
        prop1: u8,
        prop2: u8,
        prop3: u8,
    ) -> Result<usize, ()> {
        let mut current_amoeba = PanicBoard::build_current_amoeba(prop1, prop2, prop3);

        let mut board = self.board;
        if is_clockwise == false {
            board.reverse();
        }

        let mut current_pos: usize = PanicBoard::get_starting_pos(board, initial_lab);
        let mut count_mutations: u8 = 0;
        loop {
            match board[current_pos] {
                PIECE_VENT => current_pos = PanicBoard::get_next_vent(board, current_pos),
                PIECE_INVERT_COLOR => {
                    count_mutations += 1;
                    current_amoeba ^= INVERT_COLOR
                }
                PIECE_INVERT_PATTERN => {
                    count_mutations += 1;
                    current_amoeba ^= INVERT_PATTERN
                }
                PIECE_INVERT_SHAPE => {
                    count_mutations += 1;
                    current_amoeba ^= INVERT_SHAPE
                }
                PIECE_BLUE_LAB => {}
                PIECE_RED_LAB => {}
                PIECE_YELLOW_LAB => {}
                amoeba => {
                    if amoeba.eq(&current_amoeba) {
                        break;
                    }
                }
            }

            if count_mutations == 4 {
                break;
            }

            current_pos += 1;
            if current_pos == self.board.len() {
                current_pos = 0;
            }
        }

        Ok(current_pos)
    }

    pub fn build_current_amoeba(prop1: u8, prop2: u8, prop3: u8) -> u8 {
        let mut amoeba = 0b0000_0000;
        amoeba ^= prop1;
        amoeba ^= prop2;
        amoeba ^= prop3;

        amoeba
    }

    fn get_starting_pos(board: [u8; 25], initial_lab: u8) -> usize {
        board
            .iter()
            .position(|x| *x == initial_lab)
            .expect("This should never give an error. We should always be able to find the initial lab in the board!")
    }

    fn get_next_vent(board: [u8; 25], current_pos: usize) -> usize {
        board
            .iter()
            .skip(current_pos)
            .cycle()
            .position(|x| *x == PIECE_VENT)
            .expect("This should never give an error. We should always be able to find a PIECE_VENT. Unless this is not working as it should and not cycling the array!")
    }
}

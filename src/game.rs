use crate::constants::{
    INVERT_COLOR, INVERT_PATTERN, INVERT_SHAPE, PIECE_BLUE_LAB, PIECE_INVERT_COLOR,
    PIECE_INVERT_PATTERN, PIECE_INVERT_SHAPE, PIECE_RED_LAB, PIECE_VENT, PIECE_YELLOW_LAB,
};

pub struct PanicBoard {
    pub board: [u8; 25],
    current_pos: usize,
    current_amoeba: u8,
    clockwise: bool,
}

impl PanicBoard {
    pub fn new(board: [u8; 25]) -> PanicBoard {
        PanicBoard {
            board,
            current_pos: 0,
            current_amoeba: 0b0000_0000,
            clockwise: true
        }
    }

    pub fn set_clockwise(&mut self, is_clockwise: bool) {
        self.clockwise = is_clockwise;
    }

    pub fn set_current_amoeba(&mut self, new_current_amoeba: u8) {
        self.current_amoeba = new_current_amoeba;
    }

    pub fn set_current_pos(&mut self, new_current_pos: usize) {
        self.current_pos = new_current_pos;
    }

    pub fn check_amoeba_pos(&mut self) -> Result<usize, ()> {
        let mut count_mutations: u8 = 0;

        let mut current_pos: usize = self.current_pos;

        let mut board = self.board;
        if self.clockwise == false {
            board.reverse();
        }

        loop {
            match board[current_pos] {
                PIECE_VENT => current_pos = self.get_next_vent(current_pos),
                PIECE_INVERT_COLOR => {
                    count_mutations += 1;
                    self.current_amoeba ^= INVERT_COLOR
                }
                PIECE_INVERT_PATTERN => {
                    count_mutations += 1;
                    self.current_amoeba ^= INVERT_PATTERN
                }
                PIECE_INVERT_SHAPE => {
                    count_mutations += 1;
                    self.current_amoeba ^= INVERT_SHAPE
                }
                PIECE_BLUE_LAB => {}
                PIECE_RED_LAB => {}
                PIECE_YELLOW_LAB => {}
                amoeba => {
                    if amoeba.eq(&self.current_amoeba) {
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

    fn get_next_vent(&self, current_pos: usize) -> usize {
        let mut next_current_pos = current_pos + 1;

        loop {
            match self.board[next_current_pos] {
                PIECE_VENT => break,
                _ => next_current_pos += 1,
            }
        }

        next_current_pos
    }
}

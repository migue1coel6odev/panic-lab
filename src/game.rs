
use crate::constants::{
    INVERT_COLOR, INVERT_PATTERN, INVERT_SHAPE, PIECE_BLUE_LAB, PIECE_INVERT_COLOR,
    PIECE_INVERT_PATTERN, PIECE_INVERT_SHAPE, PIECE_RED_LAB, PIECE_VENT, PIECE_YELLOW_LAB,
};

pub struct PanicBoard {
    pub board: [u8; 25],
    
}

impl PanicBoard {
    pub fn new(board: [u8; 25]) -> PanicBoard {
        PanicBoard {
            board,
        }
    }

    pub fn check_amoeba_pos(&mut self, is_clockwise: bool, initial_lab: u8, prop1: u8, prop2: u8, prop3: u8) -> Result<usize, ()> {
        let mut current_amoeba = PanicBoard::build_current_amoeba(prop1, prop2, prop3);
        
        
        let mut board = self.board;
        if is_clockwise == false {
            board.reverse();
        }
        
        let mut current_pos: usize = PanicBoard::get_starting_pos(board, initial_lab);
        let mut count_mutations: u8 = 0;
        loop {
            match board[current_pos] {
                PIECE_VENT => current_pos = self.get_next_vent(current_pos),
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

    pub fn build_current_amoeba(prop1: u8, prop2: u8, prop3: u8) -> u8{
        let mut amoeba = 0b0000_0000;
        amoeba ^= prop1;
        amoeba ^= prop2;
        amoeba ^= prop3;

        amoeba
    }

    fn get_starting_pos(board: [u8;25], initial_lab: u8) -> usize {
        let mut index: usize = 0;

        loop {
            match board[index] {
                PIECE_BLUE_LAB => {
                    if initial_lab == PIECE_BLUE_LAB {
                        break;
                    }
                },
                PIECE_RED_LAB => {
                    if initial_lab == PIECE_RED_LAB {
                        break;
                    }
                },
                PIECE_YELLOW_LAB => {
                    if initial_lab == PIECE_YELLOW_LAB {
                        break;
                    }
                },
                _ => {}
            }
            index += 1;
        }
        
        index

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

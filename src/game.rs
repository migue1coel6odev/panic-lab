use crate::constants::{PIECE_VENT, PIECE_INVERT_COLOR, INVERT_COLOR, PIECE_INVERT_PATTERN, INVERT_PATTERN, PIECE_INVERT_SHAPE, INVERT_SHAPE, PIECE_BLUE_LAB, PIECE_RED_LAB, PIECE_YELLOW_LAB};

pub struct PanicBoard {
    pub board: [u8; 25],
    current_pos: usize,
    current_amoeba: u8,
}

impl PanicBoard {

    pub fn new (board: [u8; 25]) -> PanicBoard {
        PanicBoard {
            board,
            current_pos: 0,
            current_amoeba: 0b0000_0000
        }
    }

    pub fn set_current_pos(&mut self, new_current_pos: usize) {
        self.current_pos =  new_current_pos;
    }

    pub fn check_amoeba_pos(&mut self) -> Result<usize, ()> {
        

        let mut current_pos: usize = self.current_pos;

        loop {
            match self.board[current_pos] {
                PIECE_VENT => self.get_next_vent(),
                PIECE_INVERT_COLOR => self.current_amoeba ^= INVERT_COLOR,
                PIECE_INVERT_PATTERN => self.current_amoeba ^= INVERT_PATTERN,
                PIECE_INVERT_SHAPE => self.current_amoeba ^= INVERT_SHAPE,
                PIECE_BLUE_LAB => {}, 
                PIECE_RED_LAB => {},
                PIECE_YELLOW_LAB => {},
                amoeba => {
                    if amoeba.eq(&self.current_amoeba) {
                        break;
                    }
                }
            }
            current_pos += 1;
            if current_pos == self.board.len() {
                current_pos = 0;
            }
        
        }
 
        Ok(current_pos)

    }

    fn get_next_vent (&mut self) {
        let mut current_pos = self.current_pos;
        loop {
            match self.board[current_pos] {
                PIECE_VENT => {
                    self.current_pos = current_pos;
                    break
                },
                _ => current_pos += 1
            }
        }
    }


}
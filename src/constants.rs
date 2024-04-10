/**
 * Pattern
 * 0 - stripes
 * 1 - dots
 */
pub const INVERT_PATTERN: u8 = 0b0000_0001;

/**
 * Color
 * 0 - orange
 * 1 - blue
 */
pub const INVERT_COLOR: u8 = 0b0000_0010;

/**
 * Shape
 * 0 - 1 eye
 * 1 - 2 eyes
 */
pub const INVERT_SHAPE: u8 = 0b0000_0100;

pub const PIECE_VENT: u8 = 0b0001_0000;

pub const PIECE_BLUE_LAB: u8 = 0b1000_0001;
pub const PIECE_RED_LAB: u8 = 0b1000_0010;
pub const PIECE_YELLOW_LAB: u8 = 0b1000_0100;

pub const PIECE_INVERT_PATTERN: u8 = 0b0010_0001;
pub const PIECE_INVERT_COLOR: u8 = 0b0010_0010;
pub const PIECE_INVERT_SHAPE: u8 = 0b0010_0100;

pub const ONE_EYED: u8 = 0b0000_0000;
pub const TWO_EYED: u8 = 0b0000_0100;

pub const ORANGE: u8 = 0b0000_0000;
pub const BLUE: u8 = 0b0000_0010;

pub const STRIPES: u8 = 0b0000_0000;
pub const DOTS: u8 = 0b0000_0001;

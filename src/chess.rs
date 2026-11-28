use core::fmt;

pub const SIZE: i8 = 8;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pos {
    pub i: i8,
    pub j: i8,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]

pub struct Piece {
    pub color: bool,
    pub kind: Type,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
    Pawn,
}

impl Type {
    pub fn san(&self) -> &str {
        match self {
            Type::King => "K",
            Type::Queen => "Q",
            Type::Rook => "R",
            Type::Bishop => "B",
            Type::Knight => "N",
            Type::Pawn => "",
        }
    }
}

/**
 * Constructor
 */

impl Pos {
    pub fn at(s: &str) -> Self {
        let b = s.as_bytes();

        Self {
            i: SIZE - (b[1] - b'0') as i8,
            j: (b[0] - b'a') as i8,
        }
    }

    pub fn is(i: i8, j: i8) -> Self {
        Self { i, j }
    }

    pub fn from(&self, i: i8, j: i8) -> Self {
        Self {
            i: self.i + i,
            j: self.j + j,
        }
    }
}

impl Piece {
    pub fn new(color: bool, kind: Type) -> Self {
        Self { color, kind }
    }
}

/**
 * Index help
 */

impl Pos {
    pub fn index(&self) -> usize {
        return ((self.i) * SIZE + self.j) as usize;
    }

    pub fn indexa(&self, i: i8, j: i8) -> usize {
        return ((self.i + i) * SIZE + (self.j + j)) as usize;
    }
}

/**
 * Pretty print struct
 */

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}{}", (b'a' + (self.j) as u8) as char, SIZE - self.i);
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::King => write!(f, "King"),
            Type::Queen => write!(f, "Queen"),
            Type::Rook => write!(f, "Rook"),
            Type::Bishop => write!(f, "Bishop"),
            Type::Knight => write!(f, "Knight"),
            Type::Pawn => write!(f, "Pawn"),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{} {}",
            if self.color { "White" } else { "Black" },
            self.kind
        );
    }
}

impl Piece {
    pub fn icon(&self) -> &str {
        if self.color {
            match self.kind {
                Type::King => "♚",
                Type::Queen => "♛",
                Type::Rook => "♜",
                Type::Bishop => "♝",
                Type::Knight => "♞",
                Type::Pawn => "♟",
            }
        } else {
            match self.kind {
                Type::King => "♔",
                Type::Queen => "♕",
                Type::Rook => "♖",
                Type::Bishop => "♗",
                Type::Knight => "♘",
                Type::Pawn => "♙",
            }
        }
    }

    pub fn san(&self) -> &str {
        return self.kind.san();
    }

    pub fn value(&self) -> f32 {
        match self.kind {
            Type::King => 0.0f32,
            Type::Queen => 9.0f32,
            Type::Rook => 5.0f32,
            Type::Bishop => 3.3f32,
            Type::Knight => 3.2f32,
            Type::Pawn => 1.0f32,
        }
    }
}

pub mod chess_lib {
    
    use core::fmt;
    use std::{println, write};

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Piece {
        pub color: bool,
        pub kind: PieceKind,
    }

    #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    pub enum PieceKind {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn,
    }

    impl Piece {
        pub fn new(color: bool, kind: PieceKind) -> Self {
            Self {
                color: color,
                kind: kind,
            }
        }

        pub fn same_kind(&self, p: &Piece) -> bool {
            return self.kind == p.kind;
        }

        pub fn same_color(&self, p: &Piece) -> bool {
            return self.color ^ p.color;
        }

        pub fn icon(&self) -> &str {
            if self.color {
                match self.kind {
                    PieceKind::King => "]+",
                    PieceKind::Queen => "[+",
                    PieceKind::Rook => "$+",
                    PieceKind::Bishop => "&+",
                    PieceKind::Knight => "£+",
                    PieceKind::Pawn => "@+",
                }
            } else {
                match self.kind {
                    PieceKind::King => "]°",
                    PieceKind::Queen => "[°",
                    PieceKind::Rook => "$°",
                    PieceKind::Bishop => "&°",
                    PieceKind::Knight => "£°",
                    PieceKind::Pawn => "@°",
                }
            }
        }
    }

    impl fmt::Display for PieceKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PieceKind::King => write!(f, "King"),
                PieceKind::Queen => write!(f, "Queen"),
                PieceKind::Rook => write!(f, "Rook"),
                PieceKind::Bishop => write!(f, "Bishop"),
                PieceKind::Knight => write!(f, "Knight"),
                PieceKind::Pawn => write!(f, "Pawn"),
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

    #[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
    pub struct Pos {
        pub i: i8,
        pub j: i8,
    }

    impl Pos {
        pub fn new_c(i: i8, j: i8) -> Self {
            Self { i, j }
        }

        pub fn new(str: &str) -> Option<Self> {
            let mut chars = str.chars();

            let letter = chars.next()?;
            let number = chars.next()?;

            if chars.next().is_some() {
                return None;
            }

            if !('a'..='h').contains(&letter) || !('1'..='8').contains(&number) {
                return None;
            }

            Some(Self {
                i:  8-((number as u8 - b'0')) as i8,
                j: ((letter as u8) - b'a') as i8,
            })
        }

        pub fn index(&self) -> usize {
            return ((self.i) * 8 + self.j) as usize;
        }
        pub fn indexa(&self, i: i8, j: i8) -> usize {
            println!("{}", self.i);
            return ((self.i+i) * 8 + (self.j+j)) as usize;
        }
    }

    impl fmt::Display for Pos {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return write!(f, "{}{}", (b'a' + (self.j) as u8) as char, 8 - self.i);
        }
    }
}

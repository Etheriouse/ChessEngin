use core::fmt;
use std::{collections::HashMap, write};

use crate::chess_lib::chess_lib::{Piece, PieceKind, Pos};

pub struct Engin {
    board: [Option<Piece>; 64],
    played: bool,

    eat_black: HashMap<Piece, u8>,
    eat_white: HashMap<Piece, u8>,

    white_control: HashMap<Pos, u8>,
    black_control: HashMap<Pos, u8>
}

impl Engin {
    pub fn empty(played: bool) -> Self {
        Self {
            board: [None; 64],
            played,
            eat_black: HashMap::new(),
            eat_white: HashMap::new(),
            white_control: HashMap::new(),
            black_control: HashMap::new(),
        }
    }

    pub fn classic(played: bool) -> Self {
        let mut base = [None; 64];
        let white = [
            Some(Piece::new(true, PieceKind::Rook)),
            Some(Piece::new(true, PieceKind::Knight)),
            Some(Piece::new(true, PieceKind::Bishop)),
            Some(Piece::new(true, PieceKind::Queen)),
            Some(Piece::new(true, PieceKind::King)),
            Some(Piece::new(true, PieceKind::Bishop)),
            Some(Piece::new(true, PieceKind::Knight)),
            Some(Piece::new(true, PieceKind::Rook)),
        ];
        let black = [
            Some(Piece::new(false, PieceKind::Rook)),
            Some(Piece::new(false, PieceKind::Knight)),
            Some(Piece::new(false, PieceKind::Bishop)),
            Some(Piece::new(false, PieceKind::Queen)),
            Some(Piece::new(false, PieceKind::King)),
            Some(Piece::new(false, PieceKind::Bishop)),
            Some(Piece::new(false, PieceKind::Knight)),
            Some(Piece::new(false, PieceKind::Rook)),
        ];
        if played {
            base[0..8].copy_from_slice(&white);
            base[8..16].copy_from_slice(&[Some(Piece::new(true, PieceKind::Pawn)); 8]);
            base[48..56].copy_from_slice(&[Some(Piece::new(false, PieceKind::Pawn)); 8]);
            base[56..64].copy_from_slice(&black);
        } else {
            base[56..64].copy_from_slice(&white);
            base[48..56].copy_from_slice(&[Some(Piece::new(true, PieceKind::Pawn)); 8]);
            base[8..16].copy_from_slice(&[Some(Piece::new(false, PieceKind::Pawn)); 8]);
            base[0..8].copy_from_slice(&black);
        }

        Self {
            board: base,
            played,
            eat_black: HashMap::new(),
            eat_white: HashMap::new(),
            white_control: HashMap::new(),
            black_control: HashMap::new(),
        }
    }
}

impl fmt::Display for Engin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board: \n")?;
        for i in 0..8 {
            write!(f, "{}", 8 - i)?;
            for j in 0..8 {
                let index = i * 8 + j;
                match self.board[index] {
                    Some(p) => write!(f, " {}", p.icon())?,
                    None => write!(f, "  .")?,
                }
            }
            writeln!(f, "")?;
        }
        for i in 0..8 {
            write!(f, "  {}", (b'a' + i) as char)?;
        }
        Ok(())
    }
}

use core::fmt;
use std::{collections::HashMap, print, println, vec, write};

use crate::chess_lib::chess_lib::{Piece, PieceKind, Pos};

pub struct PlayerData {
    pub eat: HashMap<Piece, u8>,
    king: Pos,
}

pub struct Engin {
    board: [Option<Piece>; 64],
    pub played: bool,

    white: PlayerData,
    black: PlayerData,

    history: Vec<(Pos, Pos)>,

    // position if pass is possible
    // position are one cell behind pawn
    pass: Option<Pos>,
}

// base
impl Engin {
    pub fn empty(played: bool) -> Self {
        Self {
            board: [None; 64],
            played,
            white: PlayerData {
                eat: HashMap::new(),
                king: Pos::new_c(0, 3),
            },
            black: PlayerData {
                eat: HashMap::new(),
                king: Pos::new_c(7, 3),
            },
            history: vec![],
            pass: None,
        }
    }

    pub fn classic(played: bool) -> Self {
        let mut base = [None; 64];
        let white = [
            Some(Piece::new(true, PieceKind::Rook)),
            Some(Piece::new(true, PieceKind::Knight)),
            Some(Piece::new(true, PieceKind::Bishop)),
            Some(Piece::new(true, PieceKind::King)),
            Some(Piece::new(true, PieceKind::Queen)),
            Some(Piece::new(true, PieceKind::Bishop)),
            Some(Piece::new(true, PieceKind::Knight)),
            Some(Piece::new(true, PieceKind::Rook)),
        ];
        let black = [
            Some(Piece::new(false, PieceKind::Rook)),
            Some(Piece::new(false, PieceKind::Knight)),
            Some(Piece::new(false, PieceKind::Bishop)),
            Some(Piece::new(false, PieceKind::King)),
            Some(Piece::new(false, PieceKind::Queen)),
            Some(Piece::new(false, PieceKind::Bishop)),
            Some(Piece::new(false, PieceKind::Knight)),
            Some(Piece::new(false, PieceKind::Rook)),
        ];
        base[0..8].copy_from_slice(&black);
        base[8..16].copy_from_slice(&[Some(Piece::new(false, PieceKind::Pawn)); 8]);
        base[48..56].copy_from_slice(&[Some(Piece::new(true, PieceKind::Pawn)); 8]);
        base[56..64].copy_from_slice(&white);

        Self {
            board: base,
            played,
            white: PlayerData {
                eat: HashMap::new(),
                king: Pos::new_c(0, 3),
            },
            black: PlayerData {
                eat: HashMap::new(),
                king: Pos::new_c(7, 3),
            },
            history: vec![],
            pass: None,
        }
    }

    pub fn place(&mut self, p: Piece, to: &Pos) {
        self.board[to.index()] = Some(p);
    }

    pub fn get(&self, from: &Pos) -> &Option<Piece> {
        return &self.board[from.index()];
    }

    pub fn data(&self, color: bool) -> &PlayerData {
        return if color { &self.white } else { &self.black };
    }

    pub fn modify_data(&mut self, color: bool) -> &mut PlayerData {
        return if color {
            &mut self.white
        } else {
            &mut self.black
        };
    }

    // get all possible move without check the king rule
}

// controle
impl Engin {
    pub fn is_attack(&self, from: &Pos, camp: bool) -> bool {
        // check patern
        let knight = [
            [2, 1],
            [2, -1],
            [-2, 1],
            [-2, -1],
            [1, 2],
            [1, -2],
            [-1, 2],
            [-1, -2],
        ];
        for pos in knight {
            let i = from.i + pos[0];
            let j = from.j + pos[1];
            if !(i < 8 && j < 8 && i >= 0 && j >= 0) {
                continue;
            }
            if match self.board[((from.i + pos[0]) * 8 + from.j + pos[1]) as usize] {
                Some(piece) => match piece.kind {
                    PieceKind::Knight => return piece.color ^ camp,
                    _ => continue,
                },
                None => false,
            } {
                return true;
            }
        }

        let rook = [[1, 0], [-1, 0], [0, 1], [0, -1]];
        let bishop = [[1, 1], [-1, -1], [-1, 1], [1, -1]];

        for pos in rook {
            let mut look = from.clone();
            while look.i < 8 && look.j < 8 && look.i >= 0 && look.j >= 0 {
                if match self.board[look.index()] {
                    Some(piece) => match piece.kind {
                        PieceKind::Queen => piece.color ^ camp,
                        PieceKind::Rook => piece.color ^ camp,
                        _ => break,
                    },
                    None => false,
                } {
                    return true;
                }
                look.i += pos[0];
                look.j += pos[1];
            }
        }

        for pos in bishop {
            let mut look = from.clone();
            while look.i < 8 && look.j < 8 && look.i >= 0 && look.j >= 0 {
                if match self.board[look.index()] {
                    Some(piece) => match piece.kind {
                        PieceKind::Queen => piece.color ^ camp,
                        PieceKind::Bishop => piece.color ^ camp,
                        PieceKind::King => false, // self
                        _ => break,
                    },
                    None => false,
                } {
                    return true;
                }
                look.i += pos[0];
                look.j += pos[1];
            }
        }

        return false;
    }
}

// movement

impl Engin {
    fn get_theoric_moves(&self, from: &Pos, control: bool) -> Vec<Pos> {
        let mut base = vec![];
        let index = (from.i * 8 + from.j) as usize;
        let cell = self.board[index];
        match cell {
            Some(Piece { color, kind }) => match kind {
                PieceKind::King => Self::king(&self, &from, &mut base, color, control),
                PieceKind::Queen => Self::move_line(
                    &self,
                    &from,
                    &mut base,
                    color,
                    &vec![
                        [1, 0],
                        [-1, 0],
                        [0, 1],
                        [0, -1],
                        [1, 1],
                        [1, -1],
                        [-1, 1],
                        [-1, -1],
                    ],
                ),
                PieceKind::Rook => Self::move_line(
                    &self,
                    &from,
                    &mut base,
                    color,
                    &vec![[1, 0], [-1, 0], [0, 1], [0, -1]],
                ),
                PieceKind::Bishop => Self::move_line(
                    &self,
                    &from,
                    &mut base,
                    color,
                    &vec![[1, 1], [1, -1], [-1, 1], [-1, -1]],
                ),
                PieceKind::Knight => Self::patern(
                    &self,
                    &from,
                    &mut base,
                    color,
                    &vec![
                        [2, 1],
                        [2, -1],
                        [-2, 1],
                        [-2, -1],
                        [1, 2],
                        [1, -2],
                        [-1, 2],
                        [-1, -2],
                    ],
                ),
                PieceKind::Pawn => Self::pawn(&self, &from, &mut base, color, control),
            },
            None => (),
        }
        base
    }

    pub fn get_moves(&self, from: &Pos) -> Vec<Pos> {
        let mut base = Self::get_theoric_moves(self, from, false);

        base
    }

    // rook bishop queen
    fn move_line(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, dir: &Vec<[i8; 2]>) {
        for d in dir {
            let mut i: i8 = from.i + d[0];
            let mut j: i8 = from.j + d[1];
            while i < 8 && j < 8 && i >= 0 && j >= 0 {
                match self.board[(i * 8 + j) as usize] {
                    Some(Piece { color, kind: _ }) => {
                        if color ^ color_piece {
                            // adverse color
                            base.push(Pos::new_c(i, j));
                        }
                        break;
                    }
                    None => base.push(Pos::new_c(i, j)),
                }

                i += d[0];
                j += d[1];
            }
        }
    }

    // knight
    fn patern(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, dir: &Vec<[i8; 2]>) {
        for d in dir {
            let i: i8 = from.i + d[0];
            let j: i8 = from.j + d[1];
            if !(i < 8 && j < 8 && i >= 0 && j >= 0) {
                continue;
            }
            match self.board[(i * 8 + j) as usize] {
                Some(Piece { color, kind: _ }) => {
                    if color ^ color_piece {
                        // adverse color
                        base.push(Pos::new_c(i, j));
                        continue;
                    }
                }
                None => base.push(Pos::new_c(i, j)),
            }
        }
    }

    // pawn
    fn pawn(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, control: bool) {
        // black en letter 8 et white en letter 1

        let dir = if color_piece { -1 } else { 1 }; // si white alors deplacement vers 8
        let start = if color_piece { 1 } else { 6 };

        // println!("piece: {}", self.board[from.index()]);
        // if let Some(p) = self.pass {
        //     if (from.j - p.j.abs() == 1) && (from.i == p.i - dir) {
        //         if self.board[p.index()].is_none() {
        //             base.push(p);
        //         }
        //     }
        // }

        if !control {
            if self.board[from.indexa(dir, 0)].is_none() {
                base.push(Pos::new_c(from.i + dir, from.j));
                if self.board[from.indexa(dir * 2, 0)].is_none() {
                    base.push(Pos::new_c(from.i + (dir * 2), from.j));
                }
            }
        }
    }

    // king
    fn king(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, control: bool) {
        // manque juste le rock
        for y in from.i - 1..from.i + 2 {
            for x in from.j - 1..from.j + 2 {
                if y >= 0 && x >= 0 && y < 8 && x < 8 {
                    if !(x != from.j && y != from.i) {
                        let look = Pos::new_c(y, x);
                        match self.board[(y * 8 + x) as usize] {
                            Some(Piece { color, kind: _ }) => {
                                if ((color ^ color_piece) && self.is_attack(&look, color_piece))
                                    || control
                                {
                                    base.push(look);
                                }
                            }
                            None => {
                                if self.is_attack(&look, color_piece) || control {
                                    base.push(look);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// king
impl Engin {
    pub fn check(&self, color: bool) -> bool {
        return self.is_attack(
            if color {
                &self.white.king
            } else {
                &self.white.king
            },
            color,
        );
    }

    pub fn checkmate(&self, color: bool) -> bool {
        return false;
    }

    pub fn stalemate(&self, color: bool) -> bool {
        return false;
    }
}

impl Engin {
    // return if moved piece
    pub fn move_piece(&mut self, fromopt: &mut Option<Pos>, to: &Pos, player: bool) -> bool {
        return match fromopt {
            Some(from) => {
                for pos in self.get_moves(from).iter() {
                    // un return true dedans la
                    if pos == to {
                        self.eat(from, to, player);
                        return true;
                    };
                }
                match self.get(to) {
                    Some(_) => {
                        *fromopt = Some(*to); // 
                    }
                    None => {
                        *fromopt = None; // release from after moved piece   
                    }
                }
                return false;
            }
            None => false,
        };
    }

    fn eat(&mut self, from: &Pos, to: &Pos, player: bool) {
        let eated = self.board[to.index()];
        let piece = self.board[from.index()];

        // check pass is pawn
        // check promotion if pawn
        // check rock if king
        // maj data().king if king moved

        match eated {
            Some(eated) => {
                *(if player {
                    &mut self.white
                } else {
                    &mut self.black
                })
                .eat
                .entry(eated)
                .or_insert(0) += 1;
            }
            None => (),
        }
        self.board[to.index()] = piece;
        self.board[from.index()] = None;
    }

    pub fn selected_movement(&self, from: &Option<Pos>) -> Vec<Pos> {
        return match from {
            None => vec![],
            Some(pos) => self.get_moves(pos),
        };
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

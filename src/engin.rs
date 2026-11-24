// all things about manipulation of board

use core::fmt;
use std::{collections::HashMap, print, println, vec, write};

use crate::chess::{Piece, Pos, SIZE, Type};

const ROOK: &[[i8; 2]] = &[[1, 0], [-1, 0], [0, 1], [0, -1]];
const BISHOP: &[[i8; 2]] = &[[1, 1], [1, -1], [-1, 1], [-1, -1]];
const QUEEN: &[[i8; 2]] = &[
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1],
    [1, 0],
    [-1, 0],
    [0, 1],
    [0, -1],
];
const KNIGHT: &[[i8; 2]] = &[
    [2, 1],
    [2, -1],
    [-2, 1],
    [-2, -1],
    [1, 2],
    [1, -2],
    [-1, 2],
    [-1, -2],
];

pub struct Data {
    pub king: Pos,
    pub eat: HashMap<Piece, Pos>,
}

pub struct Engin {
    board: [Option<Piece>; (SIZE * SIZE) as usize],
    played: bool,

    white: Data,
    black: Data,

    halfmove_clock: u16,
}

/**
 * Movement
 */
impl Engin {
    pub fn moves(&mut self, from: &Pos) -> Vec<Move> {
        let Some(p) = self.board[from.index()] else {
            return vec![];
        };

        let mut base = vec![];

        for to in self.theoretical_moves(from) {
           
            if self.board[to.index()].is_some_and(|p| p.kind == Type::King) {
                continue;
            }

            let promotion = p.kind == Type::Pawn && (to.i == 0 || to.i == SIZE - 1);
           
            let proms: &[Option<Type>] = if promotion {
                &[
                    Some(Type::Knight),
                    Some(Type::Rook),
                    Some(Type::Bishop),
                    Some(Type::Queen),
                ]
            } else {
                &[None]
            };

            for &prom in proms {
                let mv = Move {
                    from: from.clone(),
                    to: to.clone(),
                    prom,
                };
                let ctx = self.do_mv(&mv);
                let verify = self.check(p.color);
                self.undo_mv(&mv, ctx);
                if !verify {
                    base.push(mv);
                }
            }
        }
        base
    }

    fn theoretical_moves(&self, from: &Pos) -> Vec<Pos> {
        let mut base = vec![];

        match self.board[from.index()] {
            None => (),
            Some(Piece { color, kind }) => match kind {
                Type::Rook => self.move_line(from, &mut base, color, &ROOK),
                Type::Bishop => self.move_line(from, &mut base, color, &BISHOP),
                Type::Knight => self.patern(from, &mut base, color, &KNIGHT),
                Type::King => (), //self.patern(from, &mut base, color, &knight),
                Type::Queen => self.move_line(from, &mut base, color, &QUEEN),
                Type::Pawn => self.pawn(from, &mut base, color),
            },
        }

        return base;
    }

    fn move_line(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, dir: &[[i8; 2]]) {
        for d in dir {
            let mut look = from.from(d[0], d[1]);
            while look.i < 8 && look.j < 8 && look.i >= 0 && look.j >= 0 {
                match self.board[look.index()] {
                    Some(Piece { color, kind: _ }) => {
                        if color ^ color_piece {
                            // adverse color
                            base.push(look.clone());
                        }
                        break;
                    }
                    None => base.push(look.clone()),
                }
                look.i += d[0];
                look.j += d[1];
            }
        }
    }

    fn patern(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, dir: &[[i8; 2]]) {
        for d in dir {
            let i: i8 = from.i + d[0];
            let j: i8 = from.j + d[1];
            if !(i < 8 && j < 8 && i >= 0 && j >= 0) {
                continue;
            }
            match self.board[(i * 8 + j) as usize] {
                Some(Piece { color, kind: _ }) => {
                    if color ^ color_piece {
                        base.push(Pos::is(i, j));
                        continue;
                    }
                }
                None => base.push(Pos::is(i, j)),
            }
        }
    }

    fn pawn(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool) {
        let dir = if color_piece { -1 } else { 1 }; // vertical movement (i axis)
        let start = if color_piece { 6 } else { 1 }; // row 

        if from.j > 0 {
            match self.board[from.indexa(dir, -1)] {
                None => (),
                Some(piece) => {
                    if piece.color ^ color_piece {
                        base.push(from.from(dir, -1));
                    }
                }
            }
        }

        if from.j < SIZE - 2 {
            match self.board[from.indexa(dir, 1)] {
                None => (),
                Some(piece) => {
                    if piece.color ^ color_piece {
                        base.push(from.from(dir, 1));
                    }
                }
            }
        }

        let look = from.i + dir;
        if (0..SIZE).contains(&look) {
            if self.board[from.indexa(dir, 0)].is_none() {
                base.push(from.from(dir, 0));
                let look = from.i + dir * 2;
                if (0..SIZE).contains(&look)
                    && from.i == start
                    && self.board[from.indexa(dir * 2, 0)].is_none()
                {
                    base.push(from.from(dir * 2, 0));
                }
            }
        }
    }

    fn king(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool) {
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let look = from.from(i, j);
                if !((0..8).contains(&look.i) && (0..8).contains(&look.j)) {
                    continue;
                }
                match self.board[look.index()] {
                    Some(p) if color_piece ^ p.color => continue,
                    Some(Piece {
                        kind: Type::King, ..
                    }) => continue,
                    _ => base.push(look),
                }
            }
        }
    }
}

pub struct UndoCtx {
    eat: Option<(Pos, Piece)>,
    pass: Option<Pos>,
    castle: [bool; 4], // little white -> big black
    white_king: Pos,
    black_king: Pos,
    moved: Piece,
    halfmove_clock: u16, // if 100 -> match nulle (remi a 0 si joue pawn or capture sinon +1)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
    pub prom: Option<Type>,
}

/**
 * Controls
 */

impl Engin {
    pub fn is_attack(&self, look: &Pos, me: bool) -> bool {
        // check aussi si la 1er case croiser est un roi
        let dir = if me { -1 } else { 1 }; // vertical movement (i axis)

        for d in ROOK {
            let mut distance = 1;
            let mut head = look.from(d[0], d[1]);
            while head.i < 8 && head.j < 8 && head.i >= 0 && head.j >= 0 {
                match self.board[head.index()] {
                    Some(Piece {
                        color,
                        kind: Type::Queen | Type::Rook,
                    }) => {
                        if color ^ me {
                            return true;
                        }
                        break;
                    }
                    Some(Piece {
                        color,
                        kind: Type::King,
                    }) if color ^ me && distance == 1 => return true,

                    Some(_) => break,
                    None => (),
                }
                head.i += d[0];
                head.j += d[1];
                distance += 1;
            }
        }

        // pareil roi mais aussi pawn inverse
        for d in BISHOP {
            let mut distance = 1;
            let mut head = look.from(d[0], d[1]);
            while head.i < 8 && head.j < 8 && head.i >= 0 && head.j >= 0 {
                match self.board[head.index()] {
                    Some(Piece {
                        color,
                        kind: Type::Queen | Type::Bishop,
                    }) => {
                        if color ^ me {
                            return true;
                        }
                        break;
                    }

                    Some(Piece {
                        color,
                        kind: Type::King,
                    }) if color ^ me && distance == 1 => return true,

                    Some(Piece {
                        color,
                        kind: Type::Pawn,
                    }) if color ^ me && distance == 1 && d[0] == dir => return true,
                    Some(_) => break,
                    None => (),
                }
                head.i += d[0];
                head.j += d[1];
                distance += 1;
            }
        }

        for d in KNIGHT {
            let i: i8 = look.i + d[0];
            let j: i8 = look.j + d[1];
            if !(i < 8 && j < 8 && i >= 0 && j >= 0) {
                continue;
            }
            match self.board[(i * 8 + j) as usize] {
                Some(Piece {
                    color,
                    kind: Type::Knight,
                }) => {
                    if color ^ me {
                        return true;
                    }
                }
                _ => (),
            }
        }
        return false;
    }

    pub fn check(&self, camp: bool) -> bool {
        let king = if camp {
            &self.white.king
        } else {
            &self.black.king
        };
        return self.is_attack(king, camp);
    }

    pub fn undo_mv(&mut self, mv: &Move, ctx: UndoCtx) {
        let to = mv.to.index();
        let from = mv.from.index();

        self.board[from] = Some(ctx.moved);
        self.board[to] = ctx.eat.map(|(_, p)| p);

        // castle rule
        // pass rule

        self.halfmove_clock = ctx.halfmove_clock;
        self.white.king = ctx.white_king;
        self.black.king = ctx.black_king;
    }

    pub fn do_mv(&mut self, mv: &Move) -> UndoCtx {
        let to = mv.to.index();
        let from = mv.from.index();

        let moved = self.board[from].expect("No pieces on the starting cell");

        let capture = self.board[to].map(|piece| (mv.to, piece));

        let ctx = UndoCtx {
            eat: capture,
            pass: None,
            castle: [false; 4],
            white_king: self.white.king,
            black_king: self.black.king,
            moved,
            halfmove_clock: self.halfmove_clock,
        };

        let head = match mv.prom {
            Some(kind) if kind == Type::Pawn => {
                Piece { color: moved.color, kind }
            },
            _ => moved,
        };

        self.board[to] = Some(head);
        self.board[from] = None;

        // castle rule
        // pass rule

        if moved.kind == Type::King {
            if moved.color {
                self.white.king = mv.to;
            } else {
                self.black.king = mv.to;
            }
        }

        if moved.kind == Type::Pawn || capture.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        return ctx;
    }
}

/**
 * Constructor
 */

impl Engin {
    pub fn classic(played: bool) -> Self {
        let mut base = [None; (SIZE * SIZE) as usize];
        let white = [
            Some(Piece::new(true, Type::Rook)),
            Some(Piece::new(true, Type::Knight)),
            Some(Piece::new(true, Type::Bishop)),
            Some(Piece::new(true, Type::King)),
            Some(Piece::new(true, Type::Queen)),
            Some(Piece::new(true, Type::Bishop)),
            Some(Piece::new(true, Type::Knight)),
            Some(Piece::new(true, Type::Rook)),
        ];
        let black = [
            Some(Piece::new(false, Type::Rook)),
            Some(Piece::new(false, Type::Knight)),
            Some(Piece::new(false, Type::Bishop)),
            Some(Piece::new(false, Type::King)),
            Some(Piece::new(false, Type::Queen)),
            Some(Piece::new(false, Type::Bishop)),
            Some(Piece::new(false, Type::Knight)),
            Some(Piece::new(false, Type::Rook)),
        ];
        base[0..8].copy_from_slice(&black);
        base[8..16].copy_from_slice(&[Some(Piece::new(false, Type::Pawn)); SIZE as usize]);
        base[48..56].copy_from_slice(&[Some(Piece::new(true, Type::Pawn)); SIZE as usize]);
        base[56..64].copy_from_slice(&white);

        Self {
            board: base,
            played,
            white: Data {
                king: Pos::is(7, 3),
                eat: HashMap::new(),
            },
            black: Data {
                king: Pos::is(0, 3),
                eat: HashMap::new(),
            },
            halfmove_clock: 0,
        }
    }
}

/**
 * Pretty print
 */
impl fmt::Display for Engin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..SIZE {
            let row = if self.played { i } else { SIZE - 1 - i };
            write!(f, "{}", SIZE - row)?;

            for j in 0..SIZE {
                let col = if self.played { j } else { SIZE - 1 - j };

                let index = (row * SIZE + col) as usize;

                match self.board[index] {
                    Some(p) => write!(f, " {}", p.icon())?,
                    None => write!(f, "  .")?,
                }
            }
            writeln!(f, "")?;
        }

        for j in 0..SIZE {
            let col = if self.played { j } else { SIZE - 1 - j };
            write!(f, "  {}", (b'a' + col as u8) as char)?;
        }

        Ok(())
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.from, self.to)?;
        if let Some(kind) = self.prom {
            write!(f, " ({})", kind)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}



#[cfg(test)]
#[path = "test/tests.rs"]
mod tests;

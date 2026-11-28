// all things about manipulation of board

use core::fmt;
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    vec, write,
};

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

const START_WHITE_PAWN: i8 = 6;
const START_BLACK_PAWN: i8 = 1;

const START_WHITE_ROOK: i8 = 7;
const START_BLACK_ROOK: i8 = 0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub king: Pos,
    pub _eat: HashMap<Piece, Pos>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Engin {
    board: [Option<Piece>; (SIZE * SIZE) as usize],
    played: bool,

    white: Data,
    black: Data,

    halfmove_clock: u16,
    pass: Option<Pos>,
    castle_rule: [bool; 4],

    history: Vec<String>,
    pos_history: Vec<u64>,
    current_hash: u64,
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
                Type::King => self.king(from, &mut base, color),
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
        // the things with the castle
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
                    Some(p) if !(color_piece ^ p.color) => continue,
                    Some(Piece {
                        kind: Type::King, ..
                    }) => continue,
                    _ => base.push(look),
                }
            }
        }

        // first two bit // white // two last bit // black
        // (left) big castle little casle (right)
        // little castle white

        if self.check(color_piece) {
            return;
        }

        let color_index = if color_piece { 0 } else { 2 };

        let little = 0;
        let big = 1;

        let row = if color_piece {
            START_WHITE_ROOK
        } else {
            START_BLACK_ROOK
        };
        let posi = |j: i8| Pos::is(row, j).index();
        let pos = |j: i8| Pos::is(row, j);

        // castle_rule show that tower and king not moved
        if self.castle_rule[little + color_index]
            && self.board[posi(5)].is_none()
            && self.board[posi(6)].is_none()
            && !self.is_attack(&pos(5), color_piece)
            && !self.is_attack(&pos(6), color_piece)
        {
            base.push(pos(6));
        }

        if self.castle_rule[big + color_index]
            && self.board[posi(1)].is_none()
            && self.board[posi(2)].is_none()
            && self.board[posi(3)].is_none()
            && !self.is_attack(&pos(2), color_piece)
            && !self.is_attack(&pos(3), color_piece)
        {
            base.push(pos(2));
        }
    }

    pub fn mobility(&self, from: &Pos) -> Vec<Pos> {
        let mut base = vec![];

        match self.board[from.index()] {
            None => (),
            Some(Piece { color, kind }) => match kind {
                Type::Rook => self.move_line(from, &mut base, color, &ROOK),
                Type::Bishop => self.move_line(from, &mut base, color, &BISHOP),
                Type::Knight => self.patern(from, &mut base, color, &KNIGHT),
                Type::Queen => self.move_line(from, &mut base, color, &QUEEN),
                _ => (),
            },
        }

        return base;
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

    pub fn checkmate(&mut self, camp: bool) -> bool {
        self.check(camp) && !self.has_legal_moves(camp)
    }

    pub fn stalemate(&mut self, camp: bool) -> bool {
        return !self.check(camp) && !self.has_legal_moves(camp);
    }

    pub fn has_legal_moves(&mut self, color: bool) -> bool {
        for i in 0..SIZE {
            for j in 0..SIZE {
                let pos = Pos::is(i, j);

                if self.board[pos.index()].is_some_and(|p| p.color == color)
                    && !self.moves(&pos).is_empty()
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn fifty_move_rule(&self) -> bool {
        self.halfmove_clock >= 100
    }

    fn count_repetition(&self) -> usize {
        return self
            .pos_history
            .iter()
            .filter(|h| **h == self.current_hash)
            .count();
    }

    pub fn draw_possible(&self) -> bool {
        return self.count_repetition() >= 3;
    }

    pub fn auto_draw(&self) -> bool {
        return self.count_repetition() >= 5;
    }

    pub fn undo_mv(&mut self, mv: &Move, ctx: UndoCtx) {
        let to = mv.to.index();
        let from = mv.from.index();

        self.board[from] = Some(ctx.moved);
        self.board[to] = ctx.eat.map(|(_, p)| p);

        if ctx.moved.kind == Type::King && mv.from.i == mv.to.i && (mv.to.j - mv.from.j).abs() == 2
        {
            let row = mv.from.i;

            let (rook_from, rook_to) = if mv.to.j == 6 {
                (Pos::is(row, 7), Pos::is(row, 5)) // Petit roque
            } else {
                (Pos::is(row, 0), Pos::is(row, 3)) // Grand roque
            };

            self.board[rook_from.index()] = self.board[rook_to.index()].take();
        }

        self.castle_rule = ctx.castle;
        self.pass = ctx.pass;

        self.halfmove_clock = ctx.halfmove_clock;
        self.white.king = ctx.white_king;
        self.black.king = ctx.black_king;
        self.history.pop();
    }

    pub fn do_mv(&mut self, mv: &Move) -> UndoCtx {
        let to = mv.to.index();
        let from = mv.from.index();

        let moved = self.board[from].expect("No pieces on the starting cell");

        let capture = self.board[to].map(|piece| (mv.to, piece));

        let ctx = UndoCtx {
            eat: capture,
            pass: self.pass,
            castle: self.castle_rule,
            white_king: self.white.king,
            black_king: self.black.king,
            moved,
            halfmove_clock: self.halfmove_clock,
        };

        let head = match mv.prom {
            Some(kind) if moved.kind == Type::Pawn => Piece {
                color: moved.color,
                kind,
            },
            _ => moved,
        };

        self.pass = None;

        let little = 1;
        let big = 0;

        match moved.kind {
            Type::Pawn => {
                if mv.from.i
                    == (if moved.color {
                        START_WHITE_PAWN
                    } else {
                        START_BLACK_PAWN
                    })
                    && (mv.from.i - mv.to.i).abs() == 2
                {
                    self.pass = Some(mv.to.from(if moved.color { 1 } else { -1 }, 0));
                }
            }

            Type::Rook => {
                // if moved => cancel side rook castle_rule for color
                let color_index = if moved.color { 0 } else { 2 };
                let start_index = if moved.color {
                    START_WHITE_ROOK
                } else {
                    START_BLACK_ROOK
                };

                if mv.from.j == 0 && mv.from.i == start_index {
                    // left
                    self.castle_rule[big + color_index] = false;
                } else if mv.from.j == 7 && mv.from.i == start_index {
                    // right
                    self.castle_rule[little + color_index] = false;
                }
            }
            Type::King => {
                let color_index = if moved.color { 0 } else { 2 };
                self.castle_rule[color_index] = false;
                self.castle_rule[color_index + 1] = false;

                let is_castle = moved.kind == Type::King
                    && mv.from.i == mv.to.i
                    && (mv.to.j - mv.from.j).abs() == 2;

                if is_castle {
                    let row = mv.from.i;

                    let (rook_from, rook_to) = if mv.to.j == 6 {
                        (Pos::is(row, 7), Pos::is(row, 5)) // Petit roque : h -> f
                    } else {
                        (Pos::is(row, 0), Pos::is(row, 3)) // Grand roque : a -> d
                    };

                    self.board[rook_to.index()] = self.board[rook_from.index()].take();
                }
            }
            _ => (),
        }

        match capture {
            Some((p, piece)) => {
                match piece.kind {
                    Type::Rook => {
                        let color_index = if piece.color { 0 } else { 2 };
                        let start_index = if piece.color {
                            START_WHITE_ROOK
                        } else {
                            START_BLACK_ROOK
                        };
                        // left
                        if p.i == start_index {
                            if p.i == 0 {
                                self.castle_rule[big + color_index] = false;
                            }
                            // right
                            if p.i == 7 {
                                self.castle_rule[little + color_index] = false;
                            }
                        }
                    }
                    _ => (),
                }
            }
            None => (),
        }

        // if king moved > 1 => castle

        self.board[to] = Some(head);
        self.board[from] = None;

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
 * Reel move
 */
impl Engin {
    /**
     * notation: true = san false = uci
     */
    pub fn mv(&mut self, mv: &Move, camp: bool, notation: bool) {
        let p = self.board[mv.from.index()].expect("nothing here");
        let mut nmv: String = String::new() + p.icon();
        nmv += &self.san_ambiguity(mv, camp);
        let ctx = self.do_mv(mv);
        if ctx.eat.is_some() {
            nmv += "x";
        }
        nmv += mv.to.to_string().as_str();
        if let Some(t) = mv.prom {
            nmv += "=";
            nmv += t.san();
        }

        if (ctx.castle[0] != self.castle_rule[0]) || (ctx.castle[2] != self.castle_rule[2]) {
            nmv = "O-O".to_owned();
        }

        if (ctx.castle[1] != self.castle_rule[1]) || (ctx.castle[3] != self.castle_rule[3]) {
            nmv = "O-O-O".to_owned();
        }

        if self.check(!camp) {
            nmv += "+";
        }
        if self.checkmate(!camp) {
            nmv += "#";
        }

        self.history.push(if notation {
            nmv
        } else {
            mv.from.to_string() + &mv.to.to_string()
        });

        self.current_hash = self.hash_zobrist();
        self.pos_history.push(self.current_hash);
    }

    fn san_ambiguity(&mut self, mv: &Move, camp: bool) -> String {
        let p = self.board[mv.from.index()].expect("nothing here");

        // No disambiguation for pawns
        if p.san().is_empty() || p.san() == "P" {
            return String::new();
        }

        let from = mv.from.to_string();

        let mut ambiguous = false;
        let mut same_file = false;
        let mut same_rank = false;

        for i in 0..SIZE {
            for j in 0..SIZE {
                let pos = Pos::is(i, j);

                if pos.index() == mv.from.index() {
                    continue;
                }

                let Some(other) = self.board[pos.index()] else {
                    continue;
                };

                if other.color != p.color || other.san() != p.san() {
                    continue;
                }

                for candidate in self.moves(&pos) {
                    if candidate.to.index() != mv.to.index() {
                        continue;
                    }

                    let ctx = self.do_mv(&candidate);
                    let legal = !self.check(camp);
                    self.undo_mv(&candidate, ctx);

                    if legal {
                        ambiguous = true;

                        let other_from = pos.to_string();

                        same_file |= other_from.as_bytes()[0] == from.as_bytes()[0];
                        same_rank |= other_from.as_bytes()[1] == from.as_bytes()[1];
                        break;
                    }
                }
            }
        }

        if !ambiguous {
            String::new()
        } else if !same_file {
            from[0..1].to_string()
        } else if !same_rank {
            from[1..2].to_string()
        } else {
            from
        }
    }

    fn hash_zobrist(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.board.hash(&mut hasher);
        self.played.hash(&mut hasher);
        self.castle_rule.hash(&mut hasher);
        self.pass.hash(&mut hasher);

        return hasher.finish();
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
            Some(Piece::new(true, Type::Queen)),
            Some(Piece::new(true, Type::King)),
            Some(Piece::new(true, Type::Bishop)),
            Some(Piece::new(true, Type::Knight)),
            Some(Piece::new(true, Type::Rook)),
        ];
        let black = [
            Some(Piece::new(false, Type::Rook)),
            Some(Piece::new(false, Type::Knight)),
            Some(Piece::new(false, Type::Bishop)),
            Some(Piece::new(false, Type::Queen)),
            Some(Piece::new(false, Type::King)),
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
                king: Pos::at("e1"),
                // king: Pos::is(7, 3),
                _eat: HashMap::new(),
            },
            black: Data {
                king: Pos::at("e8"),
                // king: Pos::is(0, 3),
                _eat: HashMap::new(),
            },
            halfmove_clock: 0,
            castle_rule: [true; 4],
            pass: None,
            history: vec![],
            pos_history: vec![],
            current_hash: 0,
        }
    }

    /**
     * Outisde of board considered as None
     */
    pub fn get(&self, p: &Pos) -> Option<Piece> {
        if (0..SIZE * SIZE).contains(&(p.index() as i8)) {
            return self.board[p.index()];
        } else {
            return None;
        }
    }

    pub fn history(&self) -> &Vec<String> {
        return &self.history;
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
                    None => write!(f, " □")?, // do something with parity ■
                }
            }
            writeln!(f, "")?;
        }

        write!(f, " ")?;
        for j in 0..SIZE {
            let col = if self.played { j } else { SIZE - 1 - j };
            write!(f, " {}", (b'a' + col as u8) as char)?;
        }
        writeln!(f, "")?;

        if self.check(true) {
            writeln!(f, "white in check")?;
        }
        if self.check(false) {
            writeln!(f, "black in check")?;
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
#[path = "tests/test.rs"]
mod test;

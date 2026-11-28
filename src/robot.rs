use std::{num::FpCategory::Infinite, println};

use crate::{
    chess::{Piece, Pos, SIZE, Type},
    engin::*,
};

const KNIGHT_TABLE: [[f32; 8]; 8] = [
    [-0.50, -0.40, -0.30, -0.30, -0.30, -0.30, -0.40, -0.50],
    [-0.40, -0.20, 0.00, 0.00, 0.00, 0.00, -0.20, -0.40],
    [-0.30, 0.00, 0.10, 0.15, 0.15, 0.10, 0.00, -0.30],
    [-0.30, 0.05, 0.15, 0.20, 0.20, 0.15, 0.05, -0.30],
    [-0.30, 0.00, 0.15, 0.20, 0.20, 0.15, 0.00, -0.30],
    [-0.30, 0.05, 0.10, 0.15, 0.15, 0.10, 0.05, -0.30],
    [-0.40, -0.20, 0.00, 0.05, 0.05, 0.00, -0.20, -0.40],
    [-0.50, -0.40, -0.30, -0.30, -0.30, -0.30, -0.40, -0.50],
];

// PION
const PAWN_TABLE: [[f32; 8]; 8] = [
    [0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00],
    [0.50, 0.50, 0.50, 0.50, 0.50, 0.50, 0.50, 0.50],
    [0.10, 0.10, 0.20, 0.30, 0.30, 0.20, 0.10, 0.10],
    [0.05, 0.05, 0.10, 0.25, 0.25, 0.10, 0.05, 0.05],
    [0.00, 0.00, 0.00, 0.20, 0.20, 0.00, 0.00, 0.00],
    [0.05, -0.05, -0.10, 0.00, 0.00, -0.10, -0.05, 0.05],
    [0.05, 0.10, 0.10, -0.20, -0.20, 0.10, 0.10, 0.05],
    [0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00],
];

// FOU
const BISHOP_TABLE: [[f32; 8]; 8] = [
    [-0.20, -0.10, -0.10, -0.10, -0.10, -0.10, -0.10, -0.20],
    [-0.10, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, -0.10],
    [-0.10, 0.00, 0.05, 0.10, 0.10, 0.05, 0.00, -0.10],
    [-0.10, 0.05, 0.05, 0.10, 0.10, 0.05, 0.05, -0.10],
    [-0.10, 0.00, 0.10, 0.10, 0.10, 0.10, 0.00, -0.10],
    [-0.10, 0.10, 0.10, 0.10, 0.10, 0.10, 0.10, -0.10],
    [-0.10, 0.05, 0.00, 0.00, 0.00, 0.00, 0.05, -0.10],
    [-0.20, -0.10, -0.10, -0.10, -0.10, -0.10, -0.10, -0.20],
];

// TOUR
const ROOK_TABLE: [[f32; 8]; 8] = [
    [0.00, 0.00, 0.00, 0.05, 0.05, 0.00, 0.00, 0.00],
    [0.05, 0.10, 0.10, 0.10, 0.10, 0.10, 0.10, 0.05],
    [-0.05, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, -0.05],
    [-0.05, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, -0.05],
    [-0.05, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, -0.05],
    [-0.05, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, -0.05],
    [-0.05, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, -0.05],
    [0.00, 0.00, 0.00, 0.05, 0.05, 0.00, 0.00, 0.00],
];

// DAME
const QUEEN_TABLE: [[f32; 8]; 8] = [
    [-0.20, -0.10, -0.10, -0.05, -0.05, -0.10, -0.10, -0.20],
    [-0.10, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, -0.10],
    [-0.10, 0.00, 0.05, 0.05, 0.05, 0.05, 0.00, -0.10],
    [-0.05, 0.00, 0.05, 0.05, 0.05, 0.05, 0.00, -0.05],
    [0.00, 0.00, 0.05, 0.05, 0.05, 0.05, 0.00, -0.05],
    [-0.10, 0.05, 0.05, 0.05, 0.05, 0.05, 0.00, -0.10],
    [-0.10, 0.00, 0.05, 0.00, 0.00, 0.00, 0.00, -0.10],
    [-0.20, -0.10, -0.10, -0.05, -0.05, -0.10, -0.10, -0.20],
];

// ROI - MILIEU DE PARTIE
const KING_TABLE: [[f32; 8]; 8] = [
    [-0.30, -0.40, -0.40, -0.50, -0.50, -0.40, -0.40, -0.30],
    [-0.30, -0.40, -0.40, -0.50, -0.50, -0.40, -0.40, -0.30],
    [-0.30, -0.40, -0.40, -0.50, -0.50, -0.40, -0.40, -0.30],
    [-0.30, -0.40, -0.40, -0.50, -0.50, -0.40, -0.40, -0.30],
    [-0.20, -0.30, -0.30, -0.40, -0.40, -0.30, -0.30, -0.20],
    [-0.10, -0.20, -0.20, -0.20, -0.20, -0.20, -0.20, -0.10],
    [0.20, 0.20, 0.00, 0.00, 0.00, 0.00, 0.20, 0.20],
    [0.20, 0.30, 0.10, 0.00, 0.00, 0.10, 0.30, 0.20],
];

pub struct Robot {}

impl Robot {
    pub fn new() -> Self {
        Self {}
    }

    pub fn do_mv(engin: &Engin) -> Move {
        return Move {
            from: Pos::is(0, 0),
            to: Pos::is(0, 0),
            prom: None,
        };
    }

    pub fn minmax(&self, depth: u8, engin: &mut Engin, camp: bool) -> Option<Move> {
        let mut best = None;
        let mut reach = -f32::INFINITY;
        for i in 0..SIZE {
            for j in 0..SIZE {
                let from = Pos::is(i, j);
                match engin.get(&from) {
                    Some(piece) => {
                        if !(piece.color ^ camp) {
                            for to_mv in engin.moves(&from) {
                                let ctx = engin.do_mv(&to_mv);
                                let tmp = self.minmax_(depth - 1, engin, !camp, camp);
                                if tmp > reach {
                                    best = Some(to_mv);
                                    reach = reach.max(tmp);
                                }
                                engin.undo_mv(&to_mv, ctx);
                            }
                        }
                    }
                    None => continue,
                }
            }
        }
        return best;
    }

    fn minmax_(&self, depth: u8, engin: &mut Engin, cp: bool, camp: bool) -> f32 {
        let mut reach = if cp == camp {
            -f32::INFINITY
        } else {
            f32::INFINITY
        };

        if engin.checkmate(cp) {
            return reach;
        }
        if engin.stalemate(cp) {
            return -5.0f32;
        }

        if depth == 0 {
            return self.evaluate_v3(engin, camp);
        }

        for i in 0..SIZE {
            for j in 0..SIZE {
                let from = Pos::is(i, j);
                match engin.get(&from) {
                    Some(piece) => {
                        if !(piece.color ^ cp) {
                            for to_mv in engin.moves(&from) {
                                let ctx = engin.do_mv(&to_mv);

                                let value = self.minmax_(depth - 1, engin, !cp, camp);

                                reach = if cp == camp {
                                    reach.max(value)
                                } else {
                                    reach.min(value)
                                };

                                engin.undo_mv(&to_mv, ctx);
                            }
                        }
                    }
                    None => continue,
                }
            }
        }

        return reach;
    }

    pub fn alpha_beta() {}

    /**
     * Just material
     */
    pub fn evaluate_v1(&self, engin: &Engin, camp: bool) -> f32 {
        let mut sum = 0.0f32;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if let Some(p) = engin.get(&Pos::is(i, j)) {
                    sum += if p.color == camp {
                        p.value()
                    } else {
                        -p.value()
                    };
                }
            }
        }
        return sum;
    }

    /**
     * Material and position
     */
    pub fn evaluate_v2(&self, engin: &Engin, camp: bool) -> f32 {
        let mut sum = 0.0f32;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if let Some(p) = engin.get(&Pos::is(i, j)) {
                    let y = if p.color { i as usize } else { 7 - i as usize };
                    let x = j as usize;
                    let pv = match p.kind {
                        crate::chess::Type::Rook => ROOK_TABLE[y][x],
                        crate::chess::Type::Knight => KNIGHT_TABLE[y][x],
                        crate::chess::Type::Bishop => BISHOP_TABLE[y][x],
                        crate::chess::Type::King => KING_TABLE[y][x],
                        crate::chess::Type::Queen => QUEEN_TABLE[y][x],
                        crate::chess::Type::Pawn => PAWN_TABLE[y][x],
                    };
                    let value = pv + p.value();
                    sum += if p.color == camp { value } else { -value };
                }
            }
        }
        return sum;
    }

    /**
     * Material and position and pawn structure
     */
    pub fn evaluate_v3(&self, engin: &Engin, camp: bool) -> f32 {
        let mut sum = 0.0f32;
        for i in 0..SIZE {
            for j in 0..SIZE {
                let head = Pos::is(i, j);
                if let Some(p) = engin.get(&head) {
                    let y = if p.color { i as usize } else { 7 - i as usize };
                    let x = j as usize;
                    let pv = match p.kind {
                        crate::chess::Type::Rook => ROOK_TABLE[y][x],
                        crate::chess::Type::Knight => KNIGHT_TABLE[y][x],
                        crate::chess::Type::Bishop => BISHOP_TABLE[y][x],
                        crate::chess::Type::King => KING_TABLE[y][x],
                        crate::chess::Type::Queen => QUEEN_TABLE[y][x],
                        crate::chess::Type::Pawn => {
                            PAWN_TABLE[y][x] + self.pawn_structure(engin, &head, camp)
                        }
                    };
                    let value = pv + p.value();

                    sum += if p.color == camp { value } else { -value };
                }
            }
        }
        return sum;
    }

    fn pawn_structure(&self, engin: &Engin, head: &Pos, camp: bool) -> f32 {
        const ISOLATE_PAWN: f32 = -0.15f32;
        const DOUBLED_PAWNS: f32 = -0.10f32;
        const PASSED_PAWN_RANGE: [f32; 6] = [0.0f32, 0.05f32, 0.15f32, 0.3f32, 0.6f32, 1.0f32];
        const CONNECTED_PAWNS: [f32; 2] = [0.05f32, 0.025f32]; // 0.5 base but /2, so two pawn aside done 0.5
        let mut score = 0.0f32;

        // isolation
        let mut noleft = true;
        let mut noright = true;
        let mut number_same_col = 0;
        let mut passed = true;
        let look = Pos::is(0, head.j);

        let index_row = if camp { SIZE - head.i - 1 } else { head.i };
        for i in 0..SIZE {
            noleft &= match engin.get(&look.from(i, -1)) {
                Some(Piece { color, kind }) => {
                    if !(color ^ camp) {
                        //ally
                        kind != Type::Pawn // return true if other than pawn
                    } else {
                        noleft
                    }
                }
                None => true,
            };
            noright &= match engin.get(&look.from(i, 1)) {
                Some(Piece { color, kind }) => {
                    if !(color ^ camp) {
                        //ally
                        kind != Type::Pawn
                    } else {
                        noright
                    }
                }
                None => true,
            };
            match engin.get(&look.from(i, 0)) {
                Some(Piece {
                    color: p,
                    kind: Type::Pawn,
                    ..
                }) => {
                    if !(p ^ camp) {
                        number_same_col += 1
                    }
                }
                _ => (),
            }
            let index_i = if camp { SIZE - i - 1 } else { i };
            if i > index_row {
                passed &= match engin.get(&look.from(index_i, -1)) {
                    Some(Piece {
                        color: p,
                        kind: Type::Pawn,
                        ..
                    }) => !(p ^ camp),
                    _ => passed,
                };
                passed &= match engin.get(&look.from(index_i, 0)) {
                    Some(Piece {
                        color: p,
                        kind: Type::Pawn,
                        ..
                    }) => !(p ^ camp),
                    _ => passed,
                };
                passed &= match engin.get(&look.from(index_i, 1)) {
                    Some(Piece {
                        color: p,
                        kind: Type::Pawn,
                        ..
                    }) => !(p ^ camp),
                    _ => passed,
                };
            }
        }
        if noleft && noright {
            score += ISOLATE_PAWN;
        }

        if number_same_col > 1 {
            score += DOUBLED_PAWNS;
        }

        let index_passed = if camp { SIZE - head.i - 2 } else { head.i - 1 } as usize;
        if passed {
            score += PASSED_PAWN_RANGE[index_passed]
        }

        let side = if camp { -1 } else { 1 };

        // is protecting
        if engin
            .get(&head.from(side, 1))
            .is_some_and(|p| !(p.color ^ camp) && p.kind == Type::Pawn)
        {
            score += CONNECTED_PAWNS[0];
        }
        if engin
            .get(&head.from(side, -1))
            .is_some_and(|p| !(p.color ^ camp) && p.kind == Type::Pawn)
        {
            score += CONNECTED_PAWNS[0];
        }

        // is protected
        if engin
            .get(&head.from(-side, 1))
            .is_some_and(|p| !(p.color ^ camp) && p.kind == Type::Pawn)
        {
            score += CONNECTED_PAWNS[0];
        }
        if engin
            .get(&head.from(-side, -1))
            .is_some_and(|p| !(p.color ^ camp) && p.kind == Type::Pawn)
        {
            score += CONNECTED_PAWNS[0];
        }

        // aside
        if engin
            .get(&head.from(0, 1))
            .is_some_and(|p| !(p.color ^ camp) && p.kind == Type::Pawn)
        {
            score += CONNECTED_PAWNS[1];
        }
        if engin
            .get(&head.from(0, -1))
            .is_some_and(|p| !(p.color ^ camp) && p.kind == Type::Pawn)
        {
            score += CONNECTED_PAWNS[1];
        }

        return score;
    }

    /**
     * Material and position and pawn structure and mobility
     */
    pub fn evaluate_v4(&self, engin: &Engin, camp: bool) -> f32 {
        const MOBILITYS_CELL: [f32; 4] = [0.01f32, 0.01f32, 0.005f32, 0.002f32];

        let mut sum = 0.0f32;
        for i in 0..SIZE {
            for j in 0..SIZE {
                let head = Pos::is(i, j);
                if let Some(p) = engin.get(&head) {
                    let y = if p.color { i as usize } else { 7 - i as usize };
                    let x = j as usize;
                    let pv = match p.kind {
                        crate::chess::Type::Bishop => {
                            BISHOP_TABLE[y][x]
                                + engin.mobility(&head).iter().count() as f32 * MOBILITYS_CELL[0]
                        }
                        crate::chess::Type::Knight => {
                            KNIGHT_TABLE[y][x]
                                + engin.mobility(&head).iter().count() as f32 * MOBILITYS_CELL[1]
                        }
                        crate::chess::Type::Rook => {
                            ROOK_TABLE[y][x]
                                + engin.mobility(&head).iter().count() as f32 * MOBILITYS_CELL[2]
                        }
                        crate::chess::Type::Queen => {
                            QUEEN_TABLE[y][x]
                                + engin.mobility(&head).iter().count() as f32 * MOBILITYS_CELL[3]
                        }
                        crate::chess::Type::King => KING_TABLE[y][x],
                        crate::chess::Type::Pawn => {
                            PAWN_TABLE[y][x] + self.pawn_structure(engin, &head, camp)
                        }
                    };
                    let value = pv + p.value();

                    sum += if p.color == camp { value } else { -value };
                }
            }
        }
        return sum;
    }
}

use std::{num::FpCategory::Infinite, println};

use crate::{
    chess::{Pos, SIZE},
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
     * Material and position and 
     */
    pub fn evaluate_v3(&self, engin: &Engin, camp: bool) -> f32 {
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
}

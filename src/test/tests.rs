use super::*;


/**
 * Constructor
 */
impl Engin {
    fn empty(played: bool) -> Self {
        Self {
            board: [None; (SIZE * SIZE) as usize],
            played,
        }
    }
}

/**
 * Manipulation
 */

impl Engin {
    fn set(&mut self, p: Piece, to: &Pos) {
        self.board[to.index()] = Some(p);
    }
}

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

mod is_attack;

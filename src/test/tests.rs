use super::*;

/**
 * Constructor
 */
impl Engin {
    fn empty(played: bool) -> Self {
        Self {
            board: [None; (SIZE * SIZE) as usize],
            played,
            white: Data {
                king: Pos::is(7, 3),
                _eat: HashMap::new(),
            },
            black: Data {
                king: Pos::is(0, 3),
                _eat: HashMap::new(),
            },
            halfmove_clock: 0,
        }
    }
}

/**
 * Manipulation
 */

impl Engin {
    fn set(&mut self, p: Piece, to: &Pos) {
        self.board[to.index()] = Some(p);
        if p.kind == Type::King {
            if p.color {
                self.white.king = *to;
            } else {
                self.black.king = *to;
            }
        }
    }
}

fn assert_moves(chess: &mut Engin, from: &str, expected: &[&str]) {
    let moves = chess.moves(&Pos::at(from));

    assert_eq!(
        moves.len(),
        expected.len(),
        "Mauvais nombre de coups depuis {from}. Obtenus : {moves:?}"
    );

    for &square in expected {
        assert!(
            moves.iter().any(|mv| mv.to == Pos::at(square)),
            "Coup {from} -> {square} manquant. Obtenus : {moves:?}"
        );
    }
}

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

mod is_attack;

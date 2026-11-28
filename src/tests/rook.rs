
use super::*;

// ============================================================
// FONCTIONS UTILITAIRES
// ============================================================

fn rook(chess: &mut Engin, white: bool, square: &str) {
    chess.set(
        Piece::new(white, Type::Rook),
        &Pos::at(square),
    );
}

fn pawn(chess: &mut Engin, white: bool, square: &str) {
    chess.set(
        Piece::new(white, Type::Pawn),
        &Pos::at(square),
    );
}

// ============================================================
// POSITION INITIALE
// ============================================================

#[test]
fn initial_white_rooks() {
    let mut chess = Engin::classic(true);

    assert_moves(&mut chess, "a1", &[]);
    assert_moves(&mut chess, "h1", &[]);
}

#[test]
fn initial_black_rooks() {
    let mut chess = Engin::classic(true);

    assert_moves(&mut chess, "a8", &[]);
    assert_moves(&mut chess, "h8", &[]);
}

// ============================================================
// PLATEAU VIDE
// ============================================================

#[test]
fn white_rook_center() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d1", "d2", "d3",
            "d5", "d6", "d7", "d8",
            "a4", "b4", "c4",
            "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn black_rook_center() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, false, "d4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d1", "d2", "d3",
            "d5", "d6", "d7", "d8",
            "a4", "b4", "c4",
            "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn rook_a1() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "a1");

    assert_moves(
        &mut chess,
        "a1",
        &[
            "a2", "a3", "a4", "a5", "a6", "a7", "a8",
            "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        ],
    );
}

#[test]
fn rook_h1() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "h1");

    assert_moves(
        &mut chess,
        "h1",
        &[
            "h2", "h3", "h4", "h5", "h6", "h7", "h8",
            "a1", "b1", "c1", "d1", "e1", "f1", "g1",
        ],
    );
}

#[test]
fn rook_a8() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "a8");

    assert_moves(
        &mut chess,
        "a8",
        &[
            "a1", "a2", "a3", "a4", "a5", "a6", "a7",
            "b8", "c8", "d8", "e8", "f8", "g8", "h8",
        ],
    );
}

#[test]
fn rook_h8() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "h8");

    assert_moves(
        &mut chess,
        "h8",
        &[
            "h1", "h2", "h3", "h4", "h5", "h6", "h7",
            "a8", "b8", "c8", "d8", "e8", "f8", "g8",
        ],
    );
}

// ============================================================
// BORDS DU PLATEAU
// ============================================================

#[test]
fn rook_a4() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "a4");

    assert_moves(
        &mut chess,
        "a4",
        &[
            "a1", "a2", "a3", "a5", "a6", "a7", "a8",
            "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn rook_h4() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "h4");

    assert_moves(
        &mut chess,
        "h4",
        &[
            "h1", "h2", "h3", "h5", "h6", "h7", "h8",
            "a4", "b4", "c4", "d4", "e4", "f4", "g4",
        ],
    );
}

#[test]
fn rook_d1() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d1");

    assert_moves(
        &mut chess,
        "d1",
        &[
            "d2", "d3", "d4", "d5", "d6", "d7", "d8",
            "a1", "b1", "c1", "e1", "f1", "g1", "h1",
        ],
    );
}

#[test]
fn rook_d8() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d8");

    assert_moves(
        &mut chess,
        "d8",
        &[
            "d1", "d2", "d3", "d4", "d5", "d6", "d7",
            "a8", "b8", "c8", "e8", "f8", "g8", "h8",
        ],
    );
}

// ============================================================
// OBSTACLES ALLIÉS
// ============================================================

#[test]
fn rook_blocked_above_by_ally() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, true, "d6");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d1", "d2", "d3", "d5",
            "a4", "b4", "c4", "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn rook_blocked_below_by_ally() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, true, "d2");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d3", "d5", "d6", "d7", "d8",
            "a4", "b4", "c4", "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn rook_blocked_left_by_ally() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, true, "b4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "c4",
            "e4", "f4", "g4", "h4",
            "d1", "d2", "d3", "d5", "d6", "d7", "d8",
        ],
    );
}

#[test]
fn rook_blocked_right_by_ally() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, true, "f4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "a4", "b4", "c4", "e4",
            "d1", "d2", "d3", "d5", "d6", "d7", "d8",
        ],
    );
}

#[test]
fn rook_surrounded_by_allies() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");

    for square in ["d5", "d3", "c4", "e4"] {
        pawn(&mut chess, true, square);
    }

    assert_moves(&mut chess, "d4", &[]);
}

// ============================================================
// CAPTURES ENNEMIES
// ============================================================

#[test]
fn rook_capture_above() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, false, "d6");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d1", "d2", "d3", "d5", "d6",
            "a4", "b4", "c4", "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn rook_capture_below() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, false, "d2");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d2", "d3", "d5", "d6", "d7", "d8",
            "a4", "b4", "c4", "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn rook_capture_left() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, false, "b4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "b4", "c4", "e4", "f4", "g4", "h4",
            "d1", "d2", "d3", "d5", "d6", "d7", "d8",
        ],
    );
}

#[test]
fn rook_capture_right() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, false, "f4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "a4", "b4", "c4", "e4", "f4",
            "d1", "d2", "d3", "d5", "d6", "d7", "d8",
        ],
    );
}

#[test]
fn rook_surrounded_by_enemies() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");

    for square in ["d5", "d3", "c4", "e4"] {
        pawn(&mut chess, false, square);
    }

    assert_moves(&mut chess, "d4", &["d5", "d3", "c4", "e4"]);
}

// ============================================================
// OBSTACLES MULTIPLES
// ============================================================

#[test]
fn rook_multiple_allied_obstacles() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");

    for square in ["d6", "d2", "b4", "f4"] {
        pawn(&mut chess, true, square);
    }

    assert_moves(
        &mut chess,
        "d4",
        &["d5", "d3", "c4", "e4"],
    );
}

#[test]
fn rook_multiple_enemy_obstacles() {
    let mut chess = Engin::empty(true);
    
    rook(&mut chess, true, "d4");

    for square in ["d6", "d2", "b4", "f4"] {
        pawn(&mut chess, false, square);
    }

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d5", "d6", "d3", "d2",
            "c4", "b4", "e4", "f4",
        ],
    );

}

#[test]
fn rook_mixed_obstacles() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");

    pawn(&mut chess, true, "d6");
    pawn(&mut chess, false, "d2");
    pawn(&mut chess, true, "b4");
    pawn(&mut chess, false, "f4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d5",
            "d3", "d2",
            "c4",
            "e4", "f4",
        ],
    );
}

// ============================================================
// NE PAS TRAVERSER LES PIÈCES
// ============================================================

#[test]
fn rook_cannot_jump_over_ally() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, true, "d5");
    pawn(&mut chess, false, "d7");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d1", "d2", "d3",
            "a4", "b4", "c4", "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn rook_cannot_jump_over_enemy() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, false, "d7");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d1", "d2", "d3", "d5",
            "a4", "b4", "c4", "e4", "f4", "g4", "h4",
        ],
    );
}

#[test]
fn rook_cannot_capture_piece_behind_enemy() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, true, "d4");
    pawn(&mut chess, false, "f4");
    pawn(&mut chess, false, "h4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "a4", "b4", "c4", "e4", "f4",
            "d1", "d2", "d3", "d5", "d6", "d7", "d8",
        ],
    );
}

// ============================================================
// SYMÉTRIE DES COULEURS
// ============================================================

#[test]
fn rook_same_moves_for_both_colors() {
    let mut white = Engin::empty(true);
    rook(&mut white, true, "e4");

    let mut black = Engin::empty(true);
    rook(&mut black, false, "e4");

    let white_moves = white.moves(&Pos::at("e4"));
    let black_moves = black.moves(&Pos::at("e4"));

    assert_eq!(white_moves.len(), black_moves.len());

    for pos in white_moves {
        assert!(black_moves.contains(&pos));
    }
}

#[test]
fn black_rook_with_allied_and_enemy_obstacles() {
    let mut chess = Engin::empty(true);
    rook(&mut chess, false, "d4");

    pawn(&mut chess, false, "d6");
    pawn(&mut chess, true, "d2");
    pawn(&mut chess, false, "b4");
    pawn(&mut chess, true, "f4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            "d5",
            "d3", "d2",
            "c4",
            "e4", "f4",
        ],
    );
}

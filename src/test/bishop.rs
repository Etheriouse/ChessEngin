
use super::*;

// ============================================================
// FONCTIONS UTILITAIRES
// ============================================================

fn assert_moves(chess: &Engin, from: &str, expected: &[&str]) {
    let moves = chess.moves(&Pos::at(from));

    assert_eq!(
        moves.len(),
        expected.len(),
        "Mauvais nombre de coups depuis {from}. Obtenus : {moves:?}"
    );

    for &square in expected {
        assert!(
            moves.contains(&Pos::at(square)),
            "Coup {from} -> {square} manquant. Obtenus : {moves:?}"
        );
    }
}

fn bishop(chess: &mut Engin, white: bool, square: &str) {
    chess.set(
        Piece::new(white, Type::Bishop),
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
fn initial_white_bishops() {
    let chess = Engin::classic(true);

    assert_moves(&chess, "c1", &[]);
    assert_moves(&chess, "f1", &[]);
}

#[test]
fn initial_black_bishops() {
    let chess = Engin::classic(true);

    assert_moves(&chess, "c8", &[]);
    assert_moves(&chess, "f8", &[]);
}

// ============================================================
// PLATEAU VIDE
// ============================================================

#[test]
fn white_bishop_center() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5", "f6", "g7", "h8",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn black_bishop_center() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, false, "d4");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5", "f6", "g7", "h8",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_e5() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "e5");

    assert_moves(
        &chess,
        "e5",
        &[
            "a1", "b2", "c3", "d4",
            "f6", "g7", "h8",
            "b8", "c7", "d6",
            "f4", "g3", "h2",
        ],
    );
}

// ============================================================
// QUATRE COINS
// ============================================================

#[test]
fn bishop_a1() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "a1");

    assert_moves(
        &chess,
        "a1",
        &["b2", "c3", "d4", "e5", "f6", "g7", "h8"],
    );
}

#[test]
fn bishop_h1() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "h1");

    assert_moves(
        &chess,
        "h1",
        &["g2", "f3", "e4", "d5", "c6", "b7", "a8"],
    );
}

#[test]
fn bishop_a8() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "a8");

    assert_moves(
        &chess,
        "a8",
        &["b7", "c6", "d5", "e4", "f3", "g2", "h1"],
    );
}

#[test]
fn bishop_h8() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "h8");

    assert_moves(
        &chess,
        "h8",
        &["g7", "f6", "e5", "d4", "c3", "b2", "a1"],
    );
}

// ============================================================
// BORDS DU PLATEAU
// ============================================================

#[test]
fn bishop_a4() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "a4");

    assert_moves(
        &chess,
        "a4",
        &[
            "b5", "c6", "d7", "e8",
            "b3", "c2", "d1",
        ],
    );
}

#[test]
fn bishop_h4() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "h4");

    assert_moves(
        &chess,
        "h4",
        &[
            "g5", "f6", "e7", "d8",
            "g3", "f2", "e1",
        ],
    );
}

#[test]
fn bishop_d1() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d1");

    assert_moves(
        &chess,
        "d1",
        &[
            "c2", "b3", "a4",
            "e2", "f3", "g4", "h5",
        ],
    );
}

#[test]
fn bishop_d8() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d8");

    assert_moves(
        &chess,
        "d8",
        &[
            "c7", "b6", "a5",
            "e7", "f6", "g5", "h4",
        ],
    );
}

// ============================================================
// OBSTACLES ALLIÉS
// ============================================================

#[test]
fn bishop_blocked_up_right_by_ally() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, true, "f6");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_blocked_up_left_by_ally() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, true, "b6");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5", "f6", "g7", "h8",
            "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_blocked_down_right_by_ally() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, true, "f2");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5", "f6", "g7", "h8",
            "a7", "b6", "c5",
            "e3",
        ],
    );
}

#[test]
fn bishop_blocked_down_left_by_ally() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, true, "b2");

    assert_moves(
        &chess,
        "d4",
        &[
            "c3",
            "e5", "f6", "g7", "h8",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_surrounded_by_allies() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");

    for square in ["c3", "c5", "e3", "e5"] {
        pawn(&mut chess, true, square);
    }

    assert_moves(&chess, "d4", &[]);
}

// ============================================================
// CAPTURES ENNEMIES
// ============================================================

#[test]
fn bishop_capture_up_right() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, false, "f6");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5", "f6",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_capture_up_left() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, false, "b6");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5", "f6", "g7", "h8",
            "c5", "b6",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_capture_down_right() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, false, "f2");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5", "f6", "g7", "h8",
            "a7", "b6", "c5",
            "e3", "f2",
        ],
    );
}

#[test]
fn bishop_capture_down_left() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, false, "b2");

    assert_moves(
        &chess,
        "d4",
        &[
            "c3", "b2",
            "e5", "f6", "g7", "h8",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_surrounded_by_enemies() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");

    for square in ["c3", "c5", "e3", "e5"] {
        pawn(&mut chess, false, square);
    }

    assert_moves(&chess, "d4", &["c3", "c5", "e3", "e5"]);
}

// ============================================================
// OBSTACLES MULTIPLES
// ============================================================

#[test]
fn bishop_multiple_allied_obstacles() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");

    for square in ["b2", "b6", "f2", "f6"] {
        pawn(&mut chess, true, square);
    }

    assert_moves(
        &chess,
        "d4",
        &["c3", "c5", "e3", "e5"],
    );
}

#[test]
fn bishop_multiple_enemy_obstacles() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");

    for square in ["b2", "b6", "f2", "f6"] {
        pawn(&mut chess, false, square);
    }

    assert_moves(
        &chess,
        "d4",
        &[
            "c3", "b2",
            "c5", "b6",
            "e3", "f2",
            "e5", "f6",
        ],
    );
}

#[test]
fn bishop_mixed_obstacles() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");

    pawn(&mut chess, true, "b2");
    pawn(&mut chess, false, "b6");
    pawn(&mut chess, true, "f2");
    pawn(&mut chess, false, "f6");

    assert_moves(
        &chess,
        "d4",
        &[
            "c3",
            "c5", "b6",
            "e3",
            "e5", "f6",
        ],
    );
}

// ============================================================
// NE PAS TRAVERSER LES PIÈCES
// ============================================================

#[test]
fn bishop_cannot_jump_over_ally() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, true, "e5");
    pawn(&mut chess, false, "g7");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_cannot_jump_over_enemy() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, false, "e5");
    pawn(&mut chess, false, "g7");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

#[test]
fn bishop_cannot_capture_piece_behind_enemy() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");
    pawn(&mut chess, false, "f6");
    pawn(&mut chess, false, "h8");

    assert_moves(
        &chess,
        "d4",
        &[
            "a1", "b2", "c3",
            "e5", "f6",
            "a7", "b6", "c5",
            "e3", "f2", "g1",
        ],
    );
}

// ============================================================
// COULEUR DES CASES
// ============================================================

#[test]
fn bishop_stays_on_same_square_color() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, true, "d4");

    let moves = chess.moves(&Pos::at("d4"));
    let from = Pos::at("d4");

    assert_eq!(moves.len(), 13);

    for pos in moves {
        assert_eq!(
            (from.i + from.j).rem_euclid(2),
            (pos.i + pos.j).rem_euclid(2),
            "Le fou a changé de couleur de case"
        );
    }
}

// ============================================================
// SYMÉTRIE DES COULEURS
// ============================================================

#[test]
fn bishop_same_moves_for_both_colors() {
    let mut white = Engin::empty(true);
    bishop(&mut white, true, "e4");

    let mut black = Engin::empty(true);
    bishop(&mut black, false, "e4");

    let white_moves = white.moves(&Pos::at("e4"));
    let black_moves = black.moves(&Pos::at("e4"));

    assert_eq!(white_moves.len(), black_moves.len());

    for pos in white_moves {
        assert!(black_moves.contains(&pos));
    }
}

#[test]
fn black_bishop_with_allied_and_enemy_obstacles() {
    let mut chess = Engin::empty(true);
    bishop(&mut chess, false, "d4");

    pawn(&mut chess, false, "b2");
    pawn(&mut chess, true, "b6");
    pawn(&mut chess, false, "f2");
    pawn(&mut chess, true, "f6");

    assert_moves(
        &chess,
        "d4",
        &[
            "c3",
            "c5", "b6",
            "e3",
            "e5", "f6",
        ],
    );
}

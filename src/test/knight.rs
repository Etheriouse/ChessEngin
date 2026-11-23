
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

fn knight(chess: &mut Engin, white: bool, square: &str) {
    chess.set(
        Piece::new(white, Type::Knight),
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
fn initial_white_knights() {
    let chess = Engin::classic(true);

    assert_moves(&chess, "b1", &["a3", "c3"]);
    assert_moves(&chess, "g1", &["f3", "h3"]);
}

#[test]
fn initial_black_knights() {
    let chess = Engin::classic(true);

    assert_moves(&chess, "b8", &["a6", "c6"]);
    assert_moves(&chess, "g8", &["f6", "h6"]);
}

// ============================================================
// CAVALIER AU CENTRE
// ============================================================

#[test]
fn white_knight_center() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d4");

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5",
            "c2", "c6",
            "e2", "e6",
            "f3", "f5",
        ],
    );
}

#[test]
fn black_knight_center() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, false, "d4");

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5",
            "c2", "c6",
            "e2", "e6",
            "f3", "f5",
        ],
    );
}

#[test]
fn knight_e5_eight_moves() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "e5");

    assert_moves(
        &chess,
        "e5",
        &[
            "c4", "c6",
            "d3", "d7",
            "f3", "f7",
            "g4", "g6",
        ],
    );
}

// ============================================================
// QUATRE COINS
// ============================================================

#[test]
fn knight_a1() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "a1");

    assert_moves(&chess, "a1", &["b3", "c2"]);
}

#[test]
fn knight_h1() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "h1");

    assert_moves(&chess, "h1", &["f2", "g3"]);
}

#[test]
fn knight_a8() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "a8");

    assert_moves(&chess, "a8", &["b6", "c7"]);
}

#[test]
fn knight_h8() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "h8");

    assert_moves(&chess, "h8", &["f7", "g6"]);
}

// ============================================================
// BORDS DU PLATEAU
// ============================================================

#[test]
fn knight_a4() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "a4");

    assert_moves(&chess, "a4", &["b2", "b6", "c3", "c5"]);
}

#[test]
fn knight_h4() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "h4");

    assert_moves(&chess, "h4", &["f3", "f5", "g2", "g6"]);
}

#[test]
fn knight_d1() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d1");

    assert_moves(&chess, "d1", &["b2", "c3", "e3", "f2"]);
}

#[test]
fn knight_d8() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d8");

    assert_moves(&chess, "d8", &["b7", "c6", "e6", "f7"]);
}

// ============================================================
// CASES PROCHES DES COINS
// ============================================================

#[test]
fn knight_b1() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "b1");

    assert_moves(&chess, "b1", &["a3", "c3", "d2"]);
}

#[test]
fn knight_g1() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "g1");

    assert_moves(&chess, "g1", &["e2", "f3", "h3"]);
}

#[test]
fn knight_b8() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "b8");

    assert_moves(&chess, "b8", &["a6", "c6", "d7"]);
}

#[test]
fn knight_g8() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "g8");

    assert_moves(&chess, "g8", &["e7", "f6", "h6"]);
}

// ============================================================
// CAPTURES
// ============================================================

#[test]
fn white_knight_capture_one_enemy() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d4");
    pawn(&mut chess, false, "f5");

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5", "c2", "c6",
            "e2", "e6", "f3", "f5",
        ],
    );
}

#[test]
fn black_knight_capture_one_enemy() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, false, "d4");
    pawn(&mut chess, true, "f5");

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5", "c2", "c6",
            "e2", "e6", "f3", "f5",
        ],
    );
}

#[test]
fn white_knight_surrounded_by_enemies() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d4");

    for square in [
        "b3", "b5", "c2", "c6",
        "e2", "e6", "f3", "f5",
    ] {
        pawn(&mut chess, false, square);
    }

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5", "c2", "c6",
            "e2", "e6", "f3", "f5",
        ],
    );
}

// ============================================================
// PIÈCES ALLIÉES
// ============================================================

#[test]
fn knight_cannot_capture_ally() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d4");
    pawn(&mut chess, true, "f5");

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5", "c2", "c6",
            "e2", "e6", "f3",
        ],
    );
}

#[test]
fn black_knight_cannot_capture_ally() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, false, "d4");
    pawn(&mut chess, false, "f5");

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5", "c2", "c6",
            "e2", "e6", "f3",
        ],
    );
}

#[test]
fn knight_surrounded_by_allies() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d4");

    for square in [
        "b3", "b5", "c2", "c6",
        "e2", "e6", "f3", "f5",
    ] {
        pawn(&mut chess, true, square);
    }

    assert_moves(&chess, "d4", &[]);
}

#[test]
fn knight_mixed_allies_and_enemies() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d4");

    pawn(&mut chess, true, "b3");
    pawn(&mut chess, true, "c2");
    pawn(&mut chess, false, "e6");
    pawn(&mut chess, false, "f5");

    assert_moves(
        &chess,
        "d4",
        &["b5", "c6", "e2", "e6", "f3", "f5"],
    );
}

// ============================================================
// LE CAVALIER SAUTE PAR-DESSUS LES PIÈCES
// ============================================================

#[test]
fn knight_jumps_over_allies() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d4");

    for square in [
        "c3", "d3", "e3",
        "c4",       "e4",
        "c5", "d5", "e5",
    ] {
        pawn(&mut chess, true, square);
    }

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5", "c2", "c6",
            "e2", "e6", "f3", "f5",
        ],
    );
}

#[test]
fn knight_jumps_over_enemies() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "d4");

    for square in [
        "c3", "d3", "e3",
        "c4",       "e4",
        "c5", "d5", "e5",
    ] {
        pawn(&mut chess, false, square);
    }

    assert_moves(
        &chess,
        "d4",
        &[
            "b3", "b5", "c2", "c6",
            "e2", "e6", "f3", "f5",
        ],
    );
}

// ============================================================
// COMBINAISONS DE BLOCAGES
// ============================================================

#[test]
fn knight_only_one_available_move() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "a1");
    pawn(&mut chess, true, "b3");

    assert_moves(&chess, "a1", &["c2"]);
}

#[test]
fn knight_no_available_moves_in_corner() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "a1");
    pawn(&mut chess, true, "b3");
    pawn(&mut chess, true, "c2");

    assert_moves(&chess, "a1", &[]);
}

#[test]
fn knight_can_capture_with_other_moves_blocked() {
    let mut chess = Engin::empty(true);
    knight(&mut chess, true, "a1");
    pawn(&mut chess, true, "b3");
    pawn(&mut chess, false, "c2");

    assert_moves(&chess, "a1", &["c2"]);
}

// ============================================================
// SYMÉTRIE BLANC / NOIR
// ============================================================

#[test]
fn knight_same_moves_for_both_colors() {
    let mut white = Engin::empty(true);
    knight(&mut white, true, "e4");

    let mut black = Engin::empty(true);
    knight(&mut black, false, "e4");

    let white_moves = white.moves(&Pos::at("e4"));
    let black_moves = black.moves(&Pos::at("e4"));

    assert_eq!(white_moves.len(), black_moves.len());

    for pos in white_moves {
        assert!(black_moves.contains(&pos));
    }
}

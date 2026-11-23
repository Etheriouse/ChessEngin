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
fn initial_pawns() {
    let chess = Engin::classic(true);

    for j in 0..SIZE {
        let white_moves = chess.moves(&Pos::is(6, j));

        assert_eq!(white_moves.len(), 2);
        assert!(white_moves.contains(&Pos::is(5, j)));
        assert!(white_moves.contains(&Pos::is(4, j)));

        let black_moves = chess.moves(&Pos::is(1, j));

        assert_eq!(black_moves.len(), 2);
        assert!(black_moves.contains(&Pos::is(2, j)));
        assert!(black_moves.contains(&Pos::is(3, j)));
    }
}

// ============================================================
// DÉPLACEMENTS SIMPLES
// ============================================================

#[test]
fn white_pawn_one_step() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");

    assert_moves(&chess, "d4", &["d5"]);
}

#[test]
fn black_pawn_one_step() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");

    assert_moves(&chess, "d5", &["d4"]);
}

#[test]
fn white_pawn_initial_two_steps() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "e2");

    assert_moves(&chess, "e2", &["e3", "e4"]);
}

#[test]
fn black_pawn_initial_two_steps() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "e7");

    assert_moves(&chess, "e7", &["e6", "e5"]);
}

#[test]
fn white_pawn_cannot_double_step_after_start() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "e3");

    assert_moves(&chess, "e3", &["e4"]);
}

#[test]
fn black_pawn_cannot_double_step_after_start() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "e6");

    assert_moves(&chess, "e6", &["e5"]);
}

// ============================================================
// BLOCAGES
// ============================================================

#[test]
fn white_pawn_blocked_by_ally() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, true, "d5");

    assert_moves(&chess, "d4", &[]);
}

#[test]
fn white_pawn_blocked_by_enemy() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "d5");

    assert_moves(&chess, "d4", &[]);
}

#[test]
fn black_pawn_blocked_by_ally() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, false, "d4");

    assert_moves(&chess, "d5", &[]);
}

#[test]
fn black_pawn_blocked_by_enemy() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "d4");

    assert_moves(&chess, "d5", &[]);
}

// ============================================================
// DOUBLE DÉPLACEMENT BLOQUÉ
// ============================================================

#[test]
fn white_initial_first_square_blocked() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "e2");
    pawn(&mut chess, false, "e3");

    assert_moves(&chess, "e2", &[]);
}

#[test]
fn white_initial_second_square_blocked() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "e2");
    pawn(&mut chess, false, "e4");

    assert_moves(&chess, "e2", &["e3"]);
}

#[test]
fn black_initial_first_square_blocked() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "e7");
    pawn(&mut chess, true, "e6");

    assert_moves(&chess, "e7", &[]);
}

#[test]
fn black_initial_second_square_blocked() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "e7");
    pawn(&mut chess, true, "e5");

    assert_moves(&chess, "e7", &["e6"]);
}

// ============================================================
// CAPTURES
// ============================================================

#[test]
fn white_pawn_capture_left() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "c5");

    assert_moves(&chess, "d4", &["d5", "c5"]);
}

#[test]
fn white_pawn_capture_right() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "e5");

    assert_moves(&chess, "d4", &["d5", "e5"]);
}

#[test]
fn white_pawn_capture_both_sides() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "c5");
    pawn(&mut chess, false, "e5");

    assert_moves(&chess, "d4", &["d5", "c5", "e5"]);
}

#[test]
fn black_pawn_capture_left() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "c4");

    assert_moves(&chess, "d5", &["d4", "c4"]);
}

#[test]
fn black_pawn_capture_right() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "e4");

    assert_moves(&chess, "d5", &["d4", "e4"]);
}

#[test]
fn black_pawn_capture_both_sides() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "c4");
    pawn(&mut chess, true, "e4");

    assert_moves(&chess, "d5", &["d4", "c4", "e4"]);
}

// ============================================================
// CAPTURES INTERDITES
// ============================================================

#[test]
fn white_pawn_cannot_capture_allies() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, true, "c5");
    pawn(&mut chess, true, "e5");

    assert_moves(&chess, "d4", &["d5"]);
}

#[test]
fn black_pawn_cannot_capture_allies() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, false, "c4");
    pawn(&mut chess, false, "e4");

    assert_moves(&chess, "d5", &["d4"]);
}

#[test]
fn white_pawn_cannot_move_diagonally_without_capture() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");

    assert_moves(&chess, "d4", &["d5"]);
}

#[test]
fn black_pawn_cannot_move_diagonally_without_capture() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");

    assert_moves(&chess, "d5", &["d4"]);
}

// ============================================================
// CAPTURES MÊME SI LE PION EST BLOQUÉ DEVANT
// ============================================================

#[test]
fn white_pawn_blocked_but_can_capture() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, false, "c5");
    pawn(&mut chess, false, "e5");

    assert_moves(&chess, "d4", &["c5", "e5"]);
}

#[test]
fn black_pawn_blocked_but_can_capture() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, true, "c4");
    pawn(&mut chess, true, "e4");

    assert_moves(&chess, "d5", &["c4", "e4"]);
}

// ============================================================
// BORDS DU PLATEAU
// ============================================================

#[test]
fn white_pawns_on_edges() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "a4");
    pawn(&mut chess, true, "h4");

    assert_moves(&chess, "a4", &["a5"]);
    assert_moves(&chess, "h4", &["h5"]);
}

#[test]
fn black_pawns_on_edges() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "a5");
    pawn(&mut chess, false, "h5");

    assert_moves(&chess, "a5", &["a4"]);
    assert_moves(&chess, "h5", &["h4"]);
}

#[test]
fn white_pawn_capture_on_a_file() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "a4");
    pawn(&mut chess, false, "b5");

    assert_moves(&chess, "a4", &["a5", "b5"]);
}

#[test]
fn white_pawn_capture_on_h_file() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "h4");
    pawn(&mut chess, false, "g5");

    assert_moves(&chess, "h4", &["h5", "g5"]);
}

#[test]
fn black_pawn_capture_on_a_file() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "a5");
    pawn(&mut chess, true, "b4");

    assert_moves(&chess, "a5", &["a4", "b4"]);
}

#[test]
fn black_pawn_capture_on_h_file() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "h5");
    pawn(&mut chess, true, "g4");

    assert_moves(&chess, "h5", &["h4", "g4"]);
}

// ============================================================
// CAPTURES DEPUIS LA POSITION INITIALE
// ============================================================

#[test]
fn white_initial_pawn_can_capture() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d2");
    pawn(&mut chess, false, "c3");
    pawn(&mut chess, false, "e3");

    assert_moves(&chess, "d2", &["d3", "d4", "c3", "e3"]);
}

#[test]
fn black_initial_pawn_can_capture() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d7");
    pawn(&mut chess, true, "c6");
    pawn(&mut chess, true, "e6");

    assert_moves(&chess, "d7", &["d6", "d5", "c6", "e6"]);
}

// ============================================================
// DERNIÈRE RANGÉE AVANT PROMOTION
// ============================================================

#[test]
fn white_pawn_reaches_last_rank() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d7");

    assert_moves(&chess, "d7", &["d8"]);
}

#[test]
fn black_pawn_reaches_last_rank() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d2");

    assert_moves(&chess, "d2", &["d1"]);
}

#[test]
fn white_pawn_can_capture_on_last_rank() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, true, "d7");
    pawn(&mut chess, false, "c8");
    pawn(&mut chess, false, "e8");

    assert_moves(&chess, "d7", &["d8", "c8", "e8"]);
}

#[test]
fn black_pawn_can_capture_on_last_rank() {
    let mut chess = Engin::empty(true);
    pawn(&mut chess, false, "d2");
    pawn(&mut chess, true, "c1");
    pawn(&mut chess, true, "e1");

    assert_moves(&chess, "d2", &["d1", "c1", "e1"]);
}

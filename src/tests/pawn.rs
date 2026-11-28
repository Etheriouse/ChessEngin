// Ce fichier teste les coups légaux des pions (Vec<Move>), promotions incluses.
// Les plateaux artificiels contiennent deux rois pour que check() soit valide.
// Les prises en passant sont à tester séparément quand elles seront implémentées.

use super::*;

// ============================================================
// FONCTIONS UTILITAIRES
// ============================================================

fn pawn(chess: &mut Engin, white: bool, square: &str) {
    chess.set(Piece::new(white, Type::Pawn), &Pos::at(square));
}

fn knight(chess: &mut Engin, white: bool, square: &str) {
    chess.set(Piece::new(white, Type::Knight), &Pos::at(square));
}

// Pour tester moves(), il faut que la position du roi du camp testé soit valide.
// Rois éloignés des pions testés pour ne pas restreindre les coups attendus.
fn empty_with_kings() -> Engin {
    let mut chess = Engin::empty(true);
    chess.set(Piece::new(true, Type::King), &Pos::at("h1"));
    chess.set(Piece::new(false, Type::King), &Pos::at("a8"));
    // On synchronise explicitement les positions suivies par check().
    chess.white.king = Pos::at("h1");
    chess.black.king = Pos::at("a8");
    chess
}

// Pour les coups ordinaires : une destination correspond à un seul Move.
fn assert_pawn_moves(chess: &mut Engin, from: &str, expected: &[&str]) {
    let moves = chess.moves(&Pos::at(from));

    assert_eq!(
        moves.len(),
        expected.len(),
        "Nombre incorrect de coups depuis {from}. Obtenus : {moves:?}"
    );

    for &square in expected {
        assert!(
            moves.iter().any(|mv| mv.to == Pos::at(square) && mv.prom.is_none()),
            "Coup ordinaire {from} -> {square} manquant. Obtenus : {moves:?}"
        );
    }
}

// Pour chaque case de promotion, il doit y avoir exactement 4 Move : Q, R, B, N.
fn assert_promotions(chess: &mut Engin, from: &str, destinations: &[&str]) {
    let moves = chess.moves(&Pos::at(from));
    assert_eq!(
        moves.len(),
        destinations.len() * 4,
        "Nombre incorrect de promotions depuis {from}. Obtenus : {moves:?}"
    );

    for &square in destinations {
        for kind in [Type::Queen, Type::Rook, Type::Bishop, Type::Knight] {
            let count = moves
                .iter()
                .filter(|mv| mv.to == Pos::at(square) && mv.prom == Some(kind))
                .count();
            assert_eq!(
                count, 1,
                "Promotion {from} -> {square} en {kind:?} absente ou dupliquée. Obtenus : {moves:?}"
            );
        }
    }
}

// ============================================================
// POSITION INITIALE
// ============================================================

#[test]
fn initial_pawns() {
    let mut chess = Engin::classic(true);

    for j in 0..SIZE {
        let white_moves = chess.moves(&Pos::is(6, j));

        assert_eq!(white_moves.len(), 2);
        assert!(white_moves.iter().any(|mv| mv.to == Pos::is(5, j)));
        assert!(white_moves.iter().any(|mv| mv.to == Pos::is(4, j)));

        let black_moves = chess.moves(&Pos::is(1, j));

        assert_eq!(black_moves.len(), 2);
        assert!(black_moves.iter().any(|mv| mv.to == Pos::is(2, j)));
        assert!(black_moves.iter().any(|mv| mv.to == Pos::is(3, j)));
    }
}

// ============================================================
// DÉPLACEMENTS SIMPLES
// ============================================================

#[test]
fn white_pawn_one_step() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");

    assert_pawn_moves(&mut chess, "d4", &["d5"]);
}

#[test]
fn black_pawn_one_step() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");

    assert_pawn_moves(&mut chess, "d5", &["d4"]);
}

#[test]
fn white_pawn_initial_two_steps() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "e2");

    assert_pawn_moves(&mut chess, "e2", &["e3", "e4"]);
}

#[test]
fn black_pawn_initial_two_steps() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "e7");

    assert_pawn_moves(&mut chess, "e7", &["e6", "e5"]);
}

#[test]
fn white_pawn_cannot_double_step_after_start() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "e3");

    assert_pawn_moves(&mut chess, "e3", &["e4"]);
}

#[test]
fn black_pawn_cannot_double_step_after_start() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "e6");

    assert_pawn_moves(&mut chess, "e6", &["e5"]);
}

// ============================================================
// BLOCAGES
// ============================================================

#[test]
fn white_pawn_blocked_by_ally() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, true, "d5");

    assert_pawn_moves(&mut chess, "d4", &[]);
}

#[test]
fn white_pawn_blocked_by_enemy() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "d5");

    assert_pawn_moves(&mut chess, "d4", &[]);
}

#[test]
fn black_pawn_blocked_by_ally() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, false, "d4");

    assert_pawn_moves(&mut chess, "d5", &[]);
}

#[test]
fn black_pawn_blocked_by_enemy() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "d4");

    assert_pawn_moves(&mut chess, "d5", &[]);
}

// ============================================================
// DOUBLE DÉPLACEMENT BLOQUÉ
// ============================================================

#[test]
fn white_initial_first_square_blocked() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "e2");
    pawn(&mut chess, false, "e3");

    assert_pawn_moves(&mut chess, "e2", &[]);
}

#[test]
fn white_initial_second_square_blocked() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "e2");
    pawn(&mut chess, false, "e4");

    assert_pawn_moves(&mut chess, "e2", &["e3"]);
}

#[test]
fn black_initial_first_square_blocked() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "e7");
    pawn(&mut chess, true, "e6");

    assert_pawn_moves(&mut chess, "e7", &[]);
}

#[test]
fn black_initial_second_square_blocked() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "e7");
    pawn(&mut chess, true, "e5");

    assert_pawn_moves(&mut chess, "e7", &["e6"]);
}

// ============================================================
// CAPTURES
// ============================================================

#[test]
fn white_pawn_capture_left() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "c5");

    assert_pawn_moves(&mut chess, "d4", &["d5", "c5"]);
}

#[test]
fn white_pawn_capture_right() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "e5");

    assert_pawn_moves(&mut chess, "d4", &["d5", "e5"]);
}

#[test]
fn white_pawn_capture_both_sides() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "c5");
    pawn(&mut chess, false, "e5");

    assert_pawn_moves(&mut chess, "d4", &["d5", "c5", "e5"]);
}

#[test]
fn black_pawn_capture_left() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "c4");

    assert_pawn_moves(&mut chess, "d5", &["d4", "c4"]);
}

#[test]
fn black_pawn_capture_right() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "e4");

    assert_pawn_moves(&mut chess, "d5", &["d4", "e4"]);
}

#[test]
fn black_pawn_capture_both_sides() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "c4");
    pawn(&mut chess, true, "e4");

    assert_pawn_moves(&mut chess, "d5", &["d4", "c4", "e4"]);
}

// ============================================================
// CAPTURES INTERDITES
// ============================================================

#[test]
fn white_pawn_cannot_capture_allies() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, true, "c5");
    pawn(&mut chess, true, "e5");

    assert_pawn_moves(&mut chess, "d4", &["d5"]);
}

#[test]
fn black_pawn_cannot_capture_allies() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, false, "c4");
    pawn(&mut chess, false, "e4");

    assert_pawn_moves(&mut chess, "d5", &["d4"]);
}

#[test]
fn white_pawn_cannot_move_diagonally_without_capture() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");

    assert_pawn_moves(&mut chess, "d4", &["d5"]);
}

#[test]
fn black_pawn_cannot_move_diagonally_without_capture() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");

    assert_pawn_moves(&mut chess, "d5", &["d4"]);
}

// ============================================================
// CAPTURES MÊME SI LE PION EST BLOQUÉ DEVANT
// ============================================================

#[test]
fn white_pawn_blocked_but_can_capture() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, false, "c5");
    pawn(&mut chess, false, "e5");

    assert_pawn_moves(&mut chess, "d4", &["c5", "e5"]);
}

#[test]
fn black_pawn_blocked_but_can_capture() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d5");
    pawn(&mut chess, true, "d4");
    pawn(&mut chess, true, "c4");
    pawn(&mut chess, true, "e4");

    assert_pawn_moves(&mut chess, "d5", &["c4", "e4"]);
}

// ============================================================
// BORDS DU PLATEAU
// ============================================================

#[test]
fn white_pawns_on_edges() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "a4");
    pawn(&mut chess, true, "h4");

    assert_pawn_moves(&mut chess, "a4", &["a5"]);
    assert_pawn_moves(&mut chess, "h4", &["h5"]);
}

#[test]
fn black_pawns_on_edges() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "a5");
    pawn(&mut chess, false, "h5");

    assert_pawn_moves(&mut chess, "a5", &["a4"]);
    assert_pawn_moves(&mut chess, "h5", &["h4"]);
}

#[test]
fn white_pawn_capture_on_a_file() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "a4");
    pawn(&mut chess, false, "b5");

    assert_pawn_moves(&mut chess, "a4", &["a5", "b5"]);
}

#[test]
fn white_pawn_capture_on_h_file() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "h4");
    pawn(&mut chess, false, "g5");

    assert_pawn_moves(&mut chess, "h4", &["h5", "g5"]);
}

#[test]
fn black_pawn_capture_on_a_file() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "a5");
    pawn(&mut chess, true, "b4");

    assert_pawn_moves(&mut chess, "a5", &["a4", "b4"]);
}

#[test]
fn black_pawn_capture_on_h_file() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "h5");
    pawn(&mut chess, true, "g4");

    assert_pawn_moves(&mut chess, "h5", &["h4", "g4"]);
}

// ============================================================
// CAPTURES DEPUIS LA POSITION INITIALE
// ============================================================

#[test]
fn white_initial_pawn_can_capture() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d2");
    pawn(&mut chess, false, "c3");
    pawn(&mut chess, false, "e3");

    assert_pawn_moves(&mut chess, "d2", &["d3", "d4", "c3", "e3"]);
}

#[test]
fn black_initial_pawn_can_capture() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d7");
    pawn(&mut chess, true, "c6");
    pawn(&mut chess, true, "e6");

    assert_pawn_moves(&mut chess, "d7", &["d6", "d5", "c6", "e6"]);
}

// ============================================================
// DERNIÈRE RANGÉE AVANT PROMOTION
// ============================================================

#[test]
fn white_pawn_reaches_last_rank() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d7");

    assert_promotions(&mut chess, "d7", &["d8"]);
}

#[test]
fn black_pawn_reaches_last_rank() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d2");

    assert_promotions(&mut chess, "d2", &["d1"]);
}

#[test]
fn white_pawn_can_capture_on_last_rank() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d7");
    knight(&mut chess, false, "c8");
    knight(&mut chess, false, "e8");

    assert_promotions(&mut chess, "d7", &["d8", "c8", "e8"]);
}

#[test]
fn black_pawn_can_capture_on_last_rank() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d2");
    knight(&mut chess, true, "c1");
    knight(&mut chess, true, "e1");

    assert_promotions(&mut chess, "d2", &["d1", "c1", "e1"]);
}

// ============================================================
// PROMOTIONS : BLOCAGES, BORDS ET ABSENCE DE PROMOTION ANTICIPÉE
// ============================================================

#[test]
fn white_promotion_blocked_straight() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d7");
    knight(&mut chess, false, "d8");
    assert_promotions(&mut chess, "d7", &[]);
}

#[test]
fn black_promotion_blocked_straight() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d2");
    knight(&mut chess, true, "d1");
    assert_promotions(&mut chess, "d2", &[]);
}

#[test]
fn white_promotion_on_h_file() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "h7");
    knight(&mut chess, false, "g8");
    assert_promotions(&mut chess, "h7", &["h8", "g8"]);
}

#[test]
fn black_promotion_on_a_file() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "a2");
    knight(&mut chess, true, "b1");
    assert_promotions(&mut chess, "a2", &["a1", "b1"]);
}

#[test]
fn white_no_promotion_before_last_rank() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, true, "d6");
    assert_pawn_moves(&mut chess, "d6", &["d7"]);
}

#[test]
fn black_no_promotion_before_last_rank() {
    let mut chess = empty_with_kings();
    pawn(&mut chess, false, "d3");
    assert_pawn_moves(&mut chess, "d3", &["d2"]);
}

// ============================================================
// PROMOTIONS : DO / UNDO POUR CHACUNE DES 4 PIÈCES
// ============================================================

fn assert_promotion_roundtrip(white: bool, from: &str, to: &str, capture: bool) {
    let mut chess = empty_with_kings();
    pawn(&mut chess, white, from);
    if capture {
        knight(&mut chess, !white, to);
    }

    let white_king_before = chess.white.king;
    let black_king_before = chess.black.king;
    let halfmove_before = chess.halfmove_clock;

    for kind in [Type::Queen, Type::Rook, Type::Bishop, Type::Knight] {
        let mv = Move {
            from: Pos::at(from),
            to: Pos::at(to),
            prom: Some(kind),
        };
        let undo = chess.do_mv(&mv);

        assert!(chess.board[mv.from.index()].is_none());
        assert!(
            chess.board[mv.to.index()]
                .is_some_and(|p| p.color == white && p.kind == kind),
            "La promotion {mv:?} doit placer la pièce choisie"
        );
        assert_eq!(chess.halfmove_clock, 0);

        chess.undo_mv(&mv, undo);

        assert!(
            chess.board[mv.from.index()]
                .is_some_and(|p| p.color == white && p.kind == Type::Pawn),
            "undo_mv() doit remettre le pion d'origine après {mv:?}"
        );
        if capture {
            assert!(
                chess.board[mv.to.index()]
                    .is_some_and(|p| p.color != white && p.kind == Type::Knight),
                "undo_mv() doit restaurer la pièce capturée après {mv:?}"
            );
        } else {
            assert!(chess.board[mv.to.index()].is_none());
        }
        assert_eq!(chess.white.king, white_king_before);
        assert_eq!(chess.black.king, black_king_before);
        assert_eq!(chess.halfmove_clock, halfmove_before);
    }
}

#[test]
fn white_promotion_do_undo() {
    assert_promotion_roundtrip(true, "d7", "d8", false);
}

#[test]
fn black_promotion_do_undo() {
    assert_promotion_roundtrip(false, "d2", "d1", false);
}

#[test]
fn white_capture_promotion_do_undo() {
    assert_promotion_roundtrip(true, "d7", "c8", true);
}

#[test]
fn black_capture_promotion_do_undo() {
    assert_promotion_roundtrip(false, "d2", "e1", true);
}

// maj with pass later
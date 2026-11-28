// Tests des echecs, echecs et mat, et pats (sans regles de nulle).
// API attendue : check(color), checkmate(color), stalemate(color).
// true = blancs ; false = noirs.
//
// IMPORTANT : les positions sont volontairement construites avec deux rois.
// Le helper synchronise les cases de roi utilisees par Engin::check().
// Ces tests utilisent des positions classiques (pas de roque ni de prise en passant).

use super::*;

// ============================================================
// HELPERS
// ============================================================

fn put(chess: &mut Engin, white: bool, kind: Type, square: &str) {
    chess.set(Piece::new(white, kind), &Pos::at(square));
}

fn position(white_king: &str, black_king: &str) -> Engin {
    let mut chess = Engin::empty(true);
    put(&mut chess, true, Type::King, white_king);
    put(&mut chess, false, Type::King, black_king);

    // Pour eviter qu'un plateau construit avec empty() utilise les
    // positions de roi par defaut lors de check().
    chess.white.king = Pos::at(white_king);
    chess.black.king = Pos::at(black_king);
    chess
}

// ============================================================
// CHECK : DETECTION DES ECHECS
// ============================================================

#[test]
fn check_no_attack_on_either_king() {
    let chess = position("e1", "e8");
    assert!(!chess.check(true));
    assert!(!chess.check(false));
}

#[test]
fn check_white_king_attacked_by_black_rook() {
    let mut chess = position("e1", "a8");
    put(&mut chess, false, Type::Rook, "e8");

    assert!(chess.check(true));
    assert!(!chess.check(false));
}

#[test]
fn check_black_king_attacked_by_white_rook() {
    let mut chess = position("a8", "e8");
    // Le roi blanc est sur a8 (position artificielle sans echec entre rois).
    put(&mut chess, true, Type::Rook, "e1");

    assert!(chess.check(false));
    assert!(!chess.check(true));
}

#[test]
fn check_white_king_attacked_by_black_bishop() {
    let mut chess = position("e1", "h8");
    put(&mut chess, false, Type::Bishop, "b4"); // b4-c3-d2-e1
    assert!(chess.check(true));
}

#[test]
fn check_black_king_attacked_by_white_bishop() {
    let mut chess = position("a1", "e8");
    put(&mut chess, true, Type::Bishop, "b5"); // b5-c6-d7-e8
    assert!(chess.check(false));
}

#[test]
fn check_white_king_attacked_by_black_queen() {
    let mut chess = position("e1", "h8");
    put(&mut chess, false, Type::Queen, "e7");
    assert!(chess.check(true));
}

#[test]
fn check_black_king_attacked_by_white_queen() {
    let mut chess = position("a1", "e8");
    put(&mut chess, true, Type::Queen, "e2");
    assert!(chess.check(false));
}

#[test]
fn check_white_king_attacked_by_black_knight() {
    let mut chess = position("e1", "h8");
    put(&mut chess, false, Type::Knight, "f3");
    assert!(chess.check(true));
}

#[test]
fn check_black_king_attacked_by_white_knight() {
    let mut chess = position("a1", "e8");
    put(&mut chess, true, Type::Knight, "f6");
    assert!(chess.check(false));
}

#[test]
fn check_white_king_attacked_by_black_pawn() {
    let mut chess = position("e1", "h8");
    put(&mut chess, false, Type::Pawn, "d2"); // Le pion noir attaque e1.
    assert!(chess.check(true));
}

#[test]
fn check_black_king_attacked_by_white_pawn() {
    let mut chess = position("a1", "e8");
    put(&mut chess, true, Type::Pawn, "d7"); // Le pion blanc attaque e8.
    assert!(chess.check(false));
}

#[test]
fn check_pawn_does_not_attack_forward() {
    let mut chess = position("e1", "h8");
    put(&mut chess, false, Type::Pawn, "e2"); // Devant le roi, pas en diagonale.
    assert!(!chess.check(true));
}

#[test]
fn check_white_pawn_does_not_attack_backward() {
    let mut chess = position("a1", "e6");
    put(&mut chess, true, Type::Pawn, "d7"); // d7 attaque c8/e8, pas e6.
    assert!(!chess.check(false));
}

#[test]
fn check_rook_attack_blocked_by_white_piece() {
    let mut chess = position("e1", "a8");
    put(&mut chess, false, Type::Rook, "e8");
    put(&mut chess, true, Type::Pawn, "e2");
    assert!(!chess.check(true));
}

#[test]
fn check_rook_attack_blocked_by_black_piece() {
    let mut chess = position("e1", "a8");
    put(&mut chess, false, Type::Rook, "e8");
    put(&mut chess, false, Type::Pawn, "e5");
    assert!(!chess.check(true));
}

#[test]
fn check_bishop_attack_blocked() {
    let mut chess = position("e1", "h8");
    put(&mut chess, false, Type::Bishop, "b4");
    put(&mut chess, true, Type::Pawn, "d2");
    assert!(!chess.check(true));
}

#[test]
fn check_knight_attack_ignores_obstacles() {
    let mut chess = position("e1", "h8");
    put(&mut chess, false, Type::Knight, "f3");
    // Les pions ne peuvent pas bloquer le saut d'un cavalier.
    put(&mut chess, true, Type::Pawn, "e2");
    put(&mut chess, false, Type::Pawn, "f2");
    assert!(chess.check(true));
}

#[test]
fn check_multiple_attackers() {
    let mut chess = position("e1", "h8");
    put(&mut chess, false, Type::Rook, "e8");
    put(&mut chess, false, Type::Knight, "f3");
    assert!(chess.check(true));
}

// ============================================================
// CHECKMATE : EN ECHEC ET AUCUN COUP LEGAL
// ============================================================

#[test]
fn checkmate_white_king_corner_two_rooks() {
    let mut chess = position("h1", "a8");
    put(&mut chess, false, Type::Rook, "h8");
    put(&mut chess, false, Type::Rook, "g8");

    assert!(chess.check(true));
    assert!(chess.checkmate(true));
    assert!(!chess.stalemate(true));
}

#[test]
fn checkmate_black_king_corner_two_rooks() {
    let mut chess = position("h1", "a8");
    put(&mut chess, true, Type::Rook, "a1");
    put(&mut chess, true, Type::Rook, "b1");

    assert!(chess.check(false));
    assert!(chess.checkmate(false));
    assert!(!chess.stalemate(false));
}

#[test]
fn checkmate_white_king_against_supported_queen() {
    let mut chess = position("h1", "f3");
    put(&mut chess, false, Type::Queen, "g2");

    assert!(chess.check(true));
    assert!(chess.checkmate(true));
}

#[test]
fn checkmate_other_piece_with_irrelevant_moves_does_not_save_king() {
    let mut chess = position("h1", "a8");
    put(&mut chess, false, Type::Rook, "h8");
    put(&mut chess, false, Type::Rook, "g8");
    put(&mut chess, true, Type::Pawn, "a2"); // Peut avancer, mais ne sauve pas le roi.

    assert!(chess.checkmate(true));
}

#[test]
fn checkmate_false_when_king_can_escape_check() {
    let mut chess = position("e1", "a8");
    put(&mut chess, false, Type::Rook, "e8");

    assert!(chess.check(true));
    assert!(!chess.checkmate(true));
}

#[test]
fn checkmate_false_when_piece_can_block_check() {
    let mut chess = position("h1", "a8");
    put(&mut chess, false, Type::Rook, "h8");
    put(&mut chess, false, Type::Rook, "g8");
    put(&mut chess, true, Type::Rook, "f2"); // Peut jouer f2-h2, interposition.

    assert!(chess.check(true));
    assert!(!chess.checkmate(true));
}

#[test]
fn checkmate_false_when_checking_piece_can_be_captured() {
    let mut chess = position("h1", "a8");
    put(&mut chess, false, Type::Queen, "g2");
    // La dame noire est sans protection : le roi peut prendre en g2.

    assert!(chess.check(true));
    assert!(!chess.checkmate(true));
}

#[test]
fn checkmate_false_if_king_is_not_in_check() {
    let chess_position = position("e1", "e8");
    let mut chess = chess_position;
    assert!(!chess.checkmate(true));
    assert!(!chess.checkmate(false));
}

// ============================================================
// STALEMATE : PAS EN ECHEC ET AUCUN COUP LEGAL
// ============================================================

#[test]
fn stalemate_white_king_corner() {
    // Blancs : Ra1. Noirs : Rc2, Db3.
    // a1 n'est pas attaque, mais a2, b1, b2 le sont.
    let mut chess = position("a1", "c2");
    put(&mut chess, false, Type::Queen, "b3");

    assert!(!chess.check(true));
    assert!(chess.stalemate(true));
    assert!(!chess.checkmate(true));
}

#[test]
fn stalemate_black_king_corner() {
    // Noirs : Rh8. Blancs : Rf7, Dg6.
    // h8 n'est pas attaque, mais g8, g7, h7 le sont.
    let mut chess = position("f7", "h8");
    put(&mut chess, true, Type::Queen, "g6");

    assert!(!chess.check(false));
    assert!(chess.stalemate(false));
    assert!(!chess.checkmate(false));
}

#[test]
fn stalemate_false_when_king_can_move() {
    let mut chess = position("h1", "a8");
    assert!(!chess.check(true));
    assert!(!chess.stalemate(true));
    assert!(!chess.stalemate(false));
}

#[test]
fn stalemate_false_when_another_piece_can_move() {
    let mut chess = position("a1", "c2");
    put(&mut chess, false, Type::Queen, "b3");
    put(&mut chess, true, Type::Pawn, "h2"); // h2-h3 est un coup legal.

    assert!(!chess.check(true));
    assert!(!chess.stalemate(true));
}

#[test]
fn stalemate_false_during_checkmate() {
    let mut chess = position("h1", "a8");
    put(&mut chess, false, Type::Rook, "h8");
    put(&mut chess, false, Type::Rook, "g8");

    assert!(chess.checkmate(true));
    assert!(!chess.stalemate(true));
}

#[test]
fn stalemate_false_during_check_with_escape() {
    let mut chess = position("e1", "a8");
    put(&mut chess, false, Type::Rook, "e8");

    assert!(chess.check(true));
    assert!(!chess.checkmate(true));
    assert!(!chess.stalemate(true));
}
// Tests des coups légaux du roi (Vec<Move>), dans le style des tests des pions.
// Les positions artificielles contiennent toujours les deux rois.
use super::*;

// ============================================================
// FONCTIONS UTILITAIRES
// ============================================================

fn king(chess: &mut Engin, white: bool, square: &str) {
    let pos = Pos::at(square);
    chess.set(Piece::new(white, Type::King), &pos);
    if white {
        chess.white.king = pos;
    } else {
        chess.black.king = pos;
    }
}

fn rook(chess: &mut Engin, white: bool, square: &str) {
    chess.set(Piece::new(white, Type::Rook), &Pos::at(square));
}

fn bishop(chess: &mut Engin, white: bool, square: &str) {
    chess.set(Piece::new(white, Type::Bishop), &Pos::at(square));
}

fn knight(chess: &mut Engin, white: bool, square: &str) {
    chess.set(Piece::new(white, Type::Knight), &Pos::at(square));
}

fn pawn(chess: &mut Engin, white: bool, square: &str) {
    chess.set(Piece::new(white, Type::Pawn), &Pos::at(square));
}

fn queen(chess: &mut Engin, white: bool, square: &str) {
    chess.set(Piece::new(white, Type::Queen), &Pos::at(square));
}

fn empty_with_kings(white: &str, black: &str) -> Engin {
    let mut chess = Engin::empty(true);
    // Désactive les roques pour ne tester que les mouvements ordinaires.
    chess.castle_rule = [false; 4];
    king(&mut chess, true, white);
    king(&mut chess, false, black);
    chess
}

fn assert_king_moves(chess: &mut Engin, from: &str, expected: &[&str]) {
    let moves = chess.moves(&Pos::at(from));

    assert_eq!(
        moves.len(),
        expected.len(),
        "Nombre incorrect de coups depuis {from}. Obtenus : {moves:?}"
    );

    for &square in expected {
        assert!(
            moves.iter().any(|mv| mv.to == Pos::at(square) && mv.prom.is_none()),
            "Coup du roi {from} -> {square} manquant. Obtenus : {moves:?}"
        );
    }
}

// ============================================================
// DÉPLACEMENTS SIMPLES ET BORDS DU PLATEAU
// ============================================================

#[test]
fn white_king_center() {
    let mut chess = empty_with_kings("e4", "a8");
    assert_king_moves(&mut chess, "e4", &["d3", "e3", "f3", "d4", "f4", "d5", "e5", "f5"]);
}

#[test]
fn black_king_center() {
    let mut chess = empty_with_kings("a1", "e5");
    assert_king_moves(&mut chess, "e5", &["d4", "e4", "f4", "d5", "f5", "d6", "e6", "f6"]);
}

#[test]
fn white_king_corner() {
    let mut chess = empty_with_kings("a1", "h8");
    assert_king_moves(&mut chess, "a1", &["a2", "b1", "b2"]);
}

#[test]
fn black_king_corner() {
    let mut chess = empty_with_kings("a1", "h8");
    assert_king_moves(&mut chess, "h8", &["g7", "h7", "g8"]);
}

#[test]
fn white_king_edge() {
    let mut chess = empty_with_kings("a4", "h8");
    assert_king_moves(&mut chess, "a4", &["a3", "b3", "b4", "a5", "b5"]);
}

// ============================================================
// PIÈCES ALLIÉES ET CAPTURES
// ============================================================

#[test]
fn king_cannot_move_onto_allies() {
    let mut chess = empty_with_kings("a1", "h8");
    pawn(&mut chess, true, "a2");
    knight(&mut chess, true, "b1");
    assert_king_moves(&mut chess, "a1", &["b2"]);
}

#[test]
fn king_surrounded_by_allies() {
    let mut chess = empty_with_kings("e4", "a8");
    for square in ["d3", "e3", "f3", "d4", "f4", "d5", "e5", "f5"] {
        pawn(&mut chess, true, square);
    }
    assert_king_moves(&mut chess, "e4", &[]);
}

#[test]
fn king_can_capture_undefended_rook() {
    let mut chess = empty_with_kings("e4", "a8");
    rook(&mut chess, false, "d5");
    // La tour contrôle d3, d4, e5 et f5 ; elle n'est pas protégée en d5.
    assert_king_moves(&mut chess, "e4", &["e3", "f3", "f4", "d5"]);
}

#[test]
fn king_cannot_capture_protected_rook() {
    let mut chess = empty_with_kings("e4", "h8");
    rook(&mut chess, false, "d5");
    bishop(&mut chess, false, "f7"); // Protège la tour d5.
    assert_king_moves(&mut chess, "e4", &["e3", "f3", "f4"]);
}

// ============================================================
// CASES ATTAQUÉES ET ÉCHEC
// ============================================================

#[test]
fn king_cannot_move_into_rook_attack() {
    let mut chess = empty_with_kings("e1", "a8");
    rook(&mut chess, false, "e8");
    assert_king_moves(&mut chess, "e1", &["d1", "f1", "d2", "f2"]);
}

#[test]
fn king_cannot_move_next_to_enemy_king() {
    let mut chess = empty_with_kings("e4", "e6");
    assert_king_moves(&mut chess, "e4", &["d3", "e3", "f3", "d4", "f4"]);
}

#[test]
fn king_cannot_move_into_pawn_attack() {
    let mut chess = empty_with_kings("e3", "h8");
    pawn(&mut chess, false, "f5"); // Attaque e4 et g4.
    assert_king_moves(&mut chess, "e3", &["d2", "e2", "f2", "d3", "f3", "d4", "f4"]);
}

#[test]
fn black_king_cannot_move_into_pawn_attack() {
    let mut chess = empty_with_kings("h1", "d6");
    pawn(&mut chess, true, "d4"); // Attaque c5 et e5.
    assert_king_moves(&mut chess, "d6", &["d5", "c6", "e6", "c7", "d7", "e7"]);
}

#[test]
fn king_cannot_move_into_knight_attack() {
    let mut chess = empty_with_kings("e4", "a8");
    knight(&mut chess, false, "f6"); // Attaque d5 parmi les destinations du roi.
    assert_king_moves(&mut chess, "e4", &["d3", "e3", "f3", "d4", "f4", "e5", "f5"]);
}

// ============================================================
// ÉCHEC ET MAT / PAT
// ============================================================

#[test]
fn king_checkmated_by_two_rooks() {
    let mut chess = empty_with_kings("a8", "h1");
    rook(&mut chess, false, "a1");
    rook(&mut chess, false, "b1");
    assert_king_moves(&mut chess, "a8", &[]);
    assert!(chess.check(true));
    assert!(chess.checkmate(true));
}

#[test]
fn king_stalemated() {
    let mut chess = empty_with_kings("a8", "c6");
    queen(&mut chess, false, "b6");
    assert_king_moves(&mut chess, "a8", &[]);
    assert!(!chess.check(true));
    assert!(chess.stalemate(true));
}

// ============================================================
// ROQUES
// ============================================================

#[test]
fn white_king_can_castle_both_sides() {
    let mut chess = empty_with_kings("e1", "e8");
    rook(&mut chess, true, "a1");
    rook(&mut chess, true, "h1");
    chess.castle_rule = [true; 4];
    assert_king_moves(&mut chess, "e1", &["d1", "f1", "d2", "e2", "f2", "c1", "g1"]);
}

#[test]
fn king_cannot_castle_through_check() {
    let mut chess = empty_with_kings("e1", "e8");
    rook(&mut chess, true, "a1");
    rook(&mut chess, true, "h1");
    rook(&mut chess, false, "f8"); // Attaque f1 (et f2).
    chess.castle_rule = [true; 4];
    assert_king_moves(&mut chess, "e1", &["d1", "d2", "e2", "c1"]);
}

#[test]
fn king_cannot_castle_while_in_check() {
    let mut chess = empty_with_kings("e1", "b8");
    rook(&mut chess, true, "a1");
    rook(&mut chess, true, "h1");
    rook(&mut chess, false, "e8");
    chess.castle_rule = [true; 4];

    println!("{}", chess);
    println!("{:?}", chess.moves(&Pos::at("e1")));

    assert_king_moves(&mut chess, "e1", &["d1", "f1", "d2", "f2"]);
}

// ============================================================
// DO / UNDO DU ROI
// ============================================================

#[test]
fn king_do_undo_restores_position() {
    let mut chess = empty_with_kings("e4", "a8");
    let king_before = chess.white.king;
    let halfmove_before = chess.halfmove_clock;
    let castle_before = chess.castle_rule;

    let mv = Move {
        from: Pos::at("e4"),
        to: Pos::at("f4"),
        prom: None,
    };
    let ctx = chess.do_mv(&mv);

    assert!(chess.board[mv.from.index()].is_none());
    assert!(chess.board[mv.to.index()].is_some_and(|p| p.color && p.kind == Type::King));
    assert_eq!(chess.white.king, Pos::at("f4"));

    chess.undo_mv(&mv, ctx);

    assert!(chess.board[mv.from.index()].is_some_and(|p| p.color && p.kind == Type::King));
    assert!(chess.board[mv.to.index()].is_none());
    assert_eq!(chess.white.king, king_before);
    assert_eq!(chess.halfmove_clock, halfmove_before);
    assert_eq!(chess.castle_rule, castle_before);
}
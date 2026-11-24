use super::*;

// Tests des déplacements géométriques de la dame.
// Pas de vérification des échecs, du roque ou de la sécurité du roi.

fn piece(chess: &mut Engin, white: bool, kind: Type, square: &str) {
    chess.set(Piece::new(white, kind), &Pos::at(square));
}

// Propriété centrale : sur une même position, les destinations de la dame
// sont exactement celles de la tour réunies à celles du fou.
fn assert_queen_is_rook_plus_bishop(from: &str, white: bool, obstacles: &[(&str, bool)]) {
    let mut queen_board = Engin::empty(true);
    let mut rook_board = Engin::empty(true);
    let mut bishop_board = Engin::empty(true);

    for &(square, color) in obstacles {
        piece(&mut queen_board, color, Type::Pawn, square);
        piece(&mut rook_board, color, Type::Pawn, square);
        piece(&mut bishop_board, color, Type::Pawn, square);
    }

    piece(&mut queen_board, white, Type::Queen, from);
    piece(&mut rook_board, white, Type::Rook, from);
    piece(&mut bishop_board, white, Type::Bishop, from);

    let queen_moves = queen_board.moves(&Pos::at(from));
    let rook_moves = rook_board.moves(&Pos::at(from));
    let bishop_moves = bishop_board.moves(&Pos::at(from));

    let mut expected = rook_moves;
    expected.extend(bishop_moves);

    assert_eq!(
        queen_moves.len(),
        expected.len(),
        "Nombre de coups de dame incorrect depuis {from}"
    );

    for pos in expected {
        assert!(
            queen_moves.contains(&pos),
            "Destination manquante depuis {from} : {pos:?}"
        );
    }
}

#[test]
fn queen_initially_blocked() {
    let mut chess = Engin::classic(true);
    assert_moves(&mut chess, "d1", &[]);
    assert_moves(&mut chess, "d8", &[]);
}

#[test]
fn queen_center_empty_board() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Queen, "d4");

    assert_moves(
        &mut chess,
        "d4",
        &[
            // Tour : 14 cases
            "d1", "d2", "d3", "d5", "d6", "d7", "d8", "a4", "b4", "c4", "e4", "f4", "g4", "h4",
            // Fou : 13 cases
            "a1", "b2", "c3", "e5", "f6", "g7", "h8", "a7", "b6", "c5", "e3", "f2", "g1",
        ],
    );
}

#[test]
fn queen_corner_a1() {
    assert_queen_is_rook_plus_bishop("a1", true, &[]);
}

#[test]
fn queen_corner_h8() {
    assert_queen_is_rook_plus_bishop("h8", false, &[]);
}

#[test]
fn queen_edge_a4() {
    assert_queen_is_rook_plus_bishop("a4", true, &[]);
}

#[test]
fn queen_edge_d8() {
    assert_queen_is_rook_plus_bishop("d8", false, &[]);
}

#[test]
fn queen_surrounded_by_allies() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Queen, "d4");

    for square in ["c3", "c4", "c5", "d3", "d5", "e3", "e4", "e5"] {
        piece(&mut chess, true, Type::Pawn, square);
    }

    assert_moves(&mut chess, "d4", &[]);
}

#[test]
fn queen_surrounded_by_enemies() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Queen, "d4");

    for square in ["c3", "c4", "c5", "d3", "d5", "e3", "e4", "e5"] {
        piece(&mut chess, false, Type::Pawn, square);
    }

    assert_moves(
        &mut chess,
        "d4",
        &["c3", "c4", "c5", "d3", "d5", "e3", "e4", "e5"],
    );
}

#[test]
fn queen_mixed_obstacles_white() {
    assert_queen_is_rook_plus_bishop(
        "d4",
        true,
        &[
            ("d6", true),
            ("d2", false),
            ("b4", true),
            ("f4", false),
            ("b2", true),
            ("b6", false),
            ("f2", true),
            ("f6", false),
        ],
    );
}

#[test]
fn queen_mixed_obstacles_black() {
    assert_queen_is_rook_plus_bishop(
        "e5",
        false,
        &[
            ("e7", false),
            ("e3", true),
            ("c5", true),
            ("g5", false),
            ("c3", false),
            ("c7", true),
            ("g3", true),
            ("g7", false),
        ],
    );
}

#[test]
fn queen_cannot_jump_over_pieces() {
    let mut chess = Engin::empty(true);

    piece(&mut chess, true, Type::Queen, "d4");
    piece(&mut chess, false, Type::Pawn, "d5");
    piece(&mut chess, false, Type::Pawn, "d7");
    piece(&mut chess, true, Type::Pawn, "e5");
    piece(&mut chess, false, Type::Pawn, "g7");

    let moves = chess.moves(&Pos::at("d4"));

    let contains = |square: &str| moves.iter().any(|mv| mv.to == Pos::at(square));

    assert!(contains("d5"));
    assert!(!contains("d6"));
    assert!(!contains("d7"));
    assert!(!contains("e5"));
    assert!(!contains("f6"));
    assert!(!contains("g7"));
}

#[test]
fn queen_both_colors_same_geometry() {
    assert_queen_is_rook_plus_bishop("e4", true, &[]);
    assert_queen_is_rook_plus_bishop("e4", false, &[]);
}

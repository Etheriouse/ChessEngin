use super::*;

// Convention : is_attack(&case, camp) retourne true si la case
// est attaquée par au moins une pièce de couleur != camp.
// On ne vérifie pas ici si le coup est légal vis-à-vis du roi.

fn piece(chess: &mut Engin, white: bool, kind: Type, at: &str) {
    chess.set(Piece::new(white, kind), &Pos::at(at));
}

fn attacked(chess: &Engin, square: &str, camp: bool, expected: bool) {
    assert_eq!(
        chess.is_attack(&Pos::at(square), camp),
        expected,
        "Attaque incorrecte sur {square}, camp={camp}, attendu={expected}",
    );
}

#[test]
fn empty_board_no_attacks() {
    let chess = Engin::empty(true);
    for square in ["a1", "d4", "e5", "h8"] {
        attacked(&chess, square, true, false);
        attacked(&chess, square, false, false);
    }
}

#[test]
fn rook_attacks_on_rank_and_file() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Rook, "d4");
    for square in ["d1", "d2", "d3", "d5", "d6", "d7", "d8", "a4", "b4", "c4", "e4", "f4", "g4", "h4"] {
        attacked(&chess, square, true, true);
    }
    for square in ["c3", "e5", "a1", "h8"] {
        attacked(&chess, square, true, false);
    }
}

#[test]
fn rook_color_matters() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Rook, "d4");
    attacked(&chess, "d7", false, true);
    attacked(&chess, "d7", true, false);
}

#[test]
fn rook_stopped_by_friendly_blocker() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Rook, "d4");
    piece(&mut chess, false, Type::Pawn, "d6");
    attacked(&chess, "d5", true, true);
    attacked(&chess, "d6", true, true);
    attacked(&chess, "d7", true, false);
}

#[test]
fn rook_stopped_by_enemy_blocker() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Rook, "d4");
    piece(&mut chess, true, Type::Pawn, "d6");
    attacked(&chess, "d5", true, true);
    attacked(&chess, "d6", true, true);
    attacked(&chess, "d7", true, false);
}

#[test]
fn bishop_attacks_all_diagonals() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Bishop, "d4");
    for square in ["a1", "b2", "c3", "e5", "f6", "g7", "h8", "a7", "b6", "c5", "e3", "f2", "g1"] {
        attacked(&chess, square, true, true);
    }
    for square in ["d5", "d3", "e4", "c4", "h4"] {
        attacked(&chess, square, true, false);
    }
}

#[test]
fn bishop_color_matters() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Bishop, "d4");
    attacked(&chess, "g7", false, true);
    attacked(&chess, "g7", true, false);
}

#[test]
fn bishop_stopped_by_friendly_blocker() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Bishop, "d4");
    piece(&mut chess, false, Type::Pawn, "f6");
    attacked(&chess, "e5", true, true);
    attacked(&chess, "f6", true, true);
    attacked(&chess, "g7", true, false);
}

#[test]
fn bishop_stopped_by_enemy_blocker() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Bishop, "d4");
    piece(&mut chess, true, Type::Pawn, "f6");
    attacked(&chess, "e5", true, true);
    attacked(&chess, "f6", true, true);
    attacked(&chess, "g7", true, false);
}

#[test]
fn queen_combines_rook_and_bishop() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Queen, "d4");
    for square in ["d1", "d8", "a4", "h4", "a1", "h8", "a7", "g1", "e5", "c4"] {
        attacked(&chess, square, true, true);
    }
    for square in ["b3", "f5", "g6"] {
        attacked(&chess, square, true, false);
    }
}

#[test]
fn queen_stopped_in_one_direction_only() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Queen, "d4");
    piece(&mut chess, true, Type::Pawn, "d6");
    piece(&mut chess, true, Type::Pawn, "f6");
    attacked(&chess, "d5", true, true);
    attacked(&chess, "d7", true, false);
    attacked(&chess, "e5", true, true);
    attacked(&chess, "g7", true, false);
    attacked(&chess, "h4", true, true);
    attacked(&chess, "b2", true, true);
}

#[test]
fn knight_attacks_eight_destinations() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Knight, "d4");
    for square in ["b3", "b5", "c2", "c6", "e2", "e6", "f3", "f5"] {
        attacked(&chess, square, true, true);
    }
    for square in ["d5", "c3", "d6", "f4"] {
        attacked(&chess, square, true, false);
    }
}

#[test]
fn knight_color_matters() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Knight, "d4");
    attacked(&chess, "f5", false, true);
    attacked(&chess, "f5", true, false);
}

#[test]
fn knight_jumps_over_pieces() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Knight, "d4");
    for square in ["c3", "d3", "e3", "c4", "e4", "c5", "d5", "e5"] {
        piece(&mut chess, false, Type::Pawn, square);
    }
    for square in ["b3", "b5", "c2", "c6", "e2", "e6", "f3", "f5"] {
        attacked(&chess, square, true, true);
    }
}

#[test]
fn knight_corner_does_not_wrap() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Knight, "a1");
    attacked(&chess, "b3", true, true);
    attacked(&chess, "c2", true, true);
    for square in ["h2", "h3", "a2", "b2", "c3"] {
        attacked(&chess, square, true, false);
    }
}

#[test]
fn white_pawn_attacks_diagonally_forward() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Pawn, "d4");
    attacked(&chess, "c5", false, true);
    attacked(&chess, "e5", false, true);
    for square in ["d5", "d3", "c3", "e3", "d6"] {
        attacked(&chess, square, false, false);
    }
}

#[test]
fn black_pawn_attacks_diagonally_forward() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Pawn, "d5");
    attacked(&chess, "c4", true, true);
    attacked(&chess, "e4", true, true);
    for square in ["d4", "d6", "c6", "e6", "d3"] {
        attacked(&chess, square, true, false);
    }
}

#[test]
fn pawn_attacks_empty_squares() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Pawn, "d4");
    attacked(&chess, "c5", false, true);
    attacked(&chess, "e5", false, true);
}

#[test]
fn pawn_attacks_occupied_squares_even_if_friendly() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Pawn, "d4");
    piece(&mut chess, true, Type::Pawn, "e5");
    attacked(&chess, "e5", false, true);
}

#[test]
fn pawn_edge_does_not_wrap() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Pawn, "a4");
    piece(&mut chess, false, Type::Pawn, "h5");
    attacked(&chess, "b5", false, true);
    attacked(&chess, "h5", false, false);
    attacked(&chess, "g4", true, true);
    attacked(&chess, "a4", true, false);
}

#[test]
fn king_attacks_adjacent_squares() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::King, "d4");
    for square in ["c3", "c4", "c5", "d3", "d5", "e3", "e4", "e5"] {
        attacked(&chess, square, true, true);
    }
    for square in ["b4", "d6", "f4", "b2"] {
        attacked(&chess, square, true, false);
    }
}

#[test]
fn king_color_matters() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::King, "d4");
    attacked(&chess, "e5", false, true);
    attacked(&chess, "e5", true, false);
}

#[test]
fn king_corner_attacks_only_three_squares() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::King, "a1");
    for square in ["a2", "b1", "b2"] {
        attacked(&chess, square, true, true);
    }
    for square in ["h1", "h2", "a3", "c2"] {
        attacked(&chess, square, true, false);
    }
}

#[test]
fn king_attacks_friendly_occupied_square() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::King, "d4");
    piece(&mut chess, false, Type::Pawn, "e5");
    attacked(&chess, "e5", true, true);
}

#[test]
fn multiple_attackers_are_detected() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Rook, "d8");
    piece(&mut chess, false, Type::Bishop, "a1");
    piece(&mut chess, false, Type::Knight, "f5");
    attacked(&chess, "d4", true, true);
}

#[test]
fn friendly_piece_does_not_count_as_enemy_attacker() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, true, Type::Rook, "d8");
    attacked(&chess, "d4", true, false);
    attacked(&chess, "d4", false, true);
}

#[test]
fn nearest_piece_blocks_farther_slider() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Rook, "d8");
    piece(&mut chess, true, Type::Pawn, "d6");
    attacked(&chess, "d4", true, false);
}

#[test]
fn pawn_and_knight_can_attack_same_square() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Pawn, "e5");
    piece(&mut chess, false, Type::Knight, "f5");
    attacked(&chess, "d4", true, true);
}

#[test]
fn rook_attack_is_not_diagonal() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Rook, "a1");
    attacked(&chess, "h8", true, false);
}

#[test]
fn bishop_attack_is_not_straight() {
    let mut chess = Engin::empty(true);
    piece(&mut chess, false, Type::Bishop, "a1");
    attacked(&chess, "a8", true, false);
    attacked(&chess, "h1", true, false);
}
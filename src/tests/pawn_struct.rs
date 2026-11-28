use super::*;
use crate::{robot::Robot};

// ============================================================
// UTILITAIRES
// ============================================================

fn empty_with_kings() -> Engin {
    let mut chess = Engin::empty(true);

    chess.set(Piece::new(true, Type::King), &Pos::at("h1"));
    chess.set(Piece::new(false, Type::King), &Pos::at("a8"));

    chess.white.king = Pos::at("h1");
    chess.black.king = Pos::at("a8");

    chess
}

fn pawn(chess: &mut Engin, white: bool, pos: &str) {
    chess.set(Piece::new(white, Type::Pawn), &Pos::at(pos));
}

// À ADAPTER : remplacer par l'appel à ton evaluate_v2().
fn eval(chess: &Engin, camp: bool) -> f32 {
    let robot = Robot::new();
    return robot.evaluate_v3(chess, camp);
}

// ============================================================
// MATERIEL
// ============================================================

#[test]
fn more_material_is_better() {
    let base = empty_with_kings();
    let mut advantage = base.clone();

    advantage.set(
        Piece::new(true, Type::Knight),
        &Pos::at("e4"),
    );

    assert!(
        eval(&advantage, true) > eval(&base, true),
        "Un cavalier supplémentaire doit améliorer le score"
    );
}

// ============================================================
// PIONS CONNECTES
// ============================================================

#[test]
fn connected_pawns_are_better() {
    let mut isolated = empty_with_kings();
    let mut connected = empty_with_kings();

    // Deux pions isolés
    pawn(&mut isolated, true, "c4");
    pawn(&mut isolated, true, "e4");

    // Deux pions en phalange
    pawn(&mut connected, true, "d4");
    pawn(&mut connected, true, "e4");

    assert!(
        eval(&connected, true) > eval(&isolated, true),
        "Les pions connectés devraient être favorisés"
    );
}

// ============================================================
// PIONS DOUBLES
// ============================================================

#[test]
fn doubled_pawns_are_worse() {
    let mut doubled = empty_with_kings();
    let mut normal = empty_with_kings();

    // Deux pions sur la même colonne
    pawn(&mut doubled, true, "d3");
    pawn(&mut doubled, true, "d4");

    // Deux pions sur des colonnes différentes
    pawn(&mut normal, true, "e3");
    pawn(&mut normal, true, "d4");

    assert!(
        eval(&normal, true) > eval(&doubled, true),
        "Les pions doublés devraient être pénalisés"
    );
}

// ============================================================
// PIONS PASSES
// ============================================================

#[test]
fn advanced_passed_pawn_is_better() {
    let mut pawn_d4 = empty_with_kings();
    let mut pawn_d5 = empty_with_kings();

    pawn(&mut pawn_d4, true, "d4");
    pawn(&mut pawn_d5, true, "d5");

    assert!(
        eval(&pawn_d5, true) > eval(&pawn_d4, true),
        "Un pion passé plus avancé devrait être favorisé"
    );
}

// ============================================================
// POINT DE VUE DE L'EVALUATION
// ============================================================

#[test]
fn material_advantage_depends_on_camp() {
    let mut chess = empty_with_kings();

    chess.set(
        Piece::new(true, Type::Queen),
        &Pos::at("e4"),
    );

    assert!(
        eval(&chess, true) > eval(&chess, false),
        "L'avantage matériel blanc doit favoriser les blancs"
    );
}

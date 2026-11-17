
pub mod chess_lib;
pub mod engin;

use std::println;
use crate::{chess_lib::chess_lib::{Piece, PieceKind, Pos}, engin::Engin};

// todo a faire pour la pass
// - mettre a jour la pass si pawn moved en 2
// - mettre a jour la pass si pass != None et que autre piece est jouer ou pawn jour ailleurs
// https://fr.wikipedia.org/wiki/En_passant_(%C3%A9checs)



fn main() {

    let piece = Piece{color: true, kind: PieceKind::Knight};

    let a8 = Pos::new("e7");
    let a1 = Pos::new_c(4, 4);


    println!("{}", piece);
    println!("a8: {} a1: {}", match a8 {
        Some(p) => p,
        None => {
            println!("undefined pos");
            return;
        }
    }, a1);

    let rook = Pos::new("g2").unwrap();
    let mut chess = Engin::classic(true);
    println!("{}", chess);
    // chess.place(Piece::new(true, PieceKind::Bishop), &rook);
    
    let moves = chess.get_moves(&rook);
    let show = moves
    .iter()
    .map(|p| p.to_string()) // Appelle automatiquement votre implémentation de Display
    .collect::<Vec<_>>()
    .join(", ");
    println!("possible move for {}: [{:?}]", rook, show)
    
}

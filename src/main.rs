
pub mod chess_lib;
pub mod engin;

use std::println;
use crate::{chess_lib::chess_lib::{Piece, PieceKind, Pos}, engin::Engin};


fn main() {
    println!("Hello, world!");

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

    let chess = Engin::classic(true);
    println!("{}", chess);
    
}

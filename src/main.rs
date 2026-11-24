use std::println;

use crate::{chess::Pos, engin::{Engin, Move}};


mod chess;
mod engin;

fn main() {

    let mut engin = Engin::classic(true);
    
    let mv = Move { from: Pos::at("a2"), to: Pos::at("a4"), prom: None};
    engin.do_mv(&mv);
    let mv = Move { from: Pos::at("a4"), to: Pos::at("a5"), prom: None};
    engin.do_mv(&mv);
    let mv = Move { from: Pos::at("a5"), to: Pos::at("a6"), prom: None};
    engin.do_mv(&mv);
    let mv = Move { from: Pos::at("a6"), to: Pos::at("b7"), prom: None};
    engin.do_mv(&mv);
    println!("{}", engin);
    println!("{:?}", engin.moves(&Pos::at("b2")));

}
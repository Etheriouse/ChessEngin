use std::println;

use crate::{chess::Pos, engin::Engin};


mod chess;
mod engin;

fn main() {

    let engin = Engin::classic(true);
    println!("{}", engin);
    println!("{:?}", engin.moves(&Pos::at("a2")));

}
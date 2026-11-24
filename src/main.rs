use std::println;

use crate::{chess::Pos, engin::{Engin, Move}};


mod chess;
mod engin;

fn main() {

    let engin = Engin::classic(true);

    
    println!("{}", engin);

}
use std::{println, time::Instant};

use crate::{
    chess::Pos,
    engin::{Engin, Move},
    robot::Robot,
};

mod chess;
mod engin;
mod robot;

fn main() {
    let mut engin = Engin::classic(true);
    let robot = Robot::new();

    // // let before = engin.clone();

    // // let mv = Move {
    // //     from: Pos::at("a2"),
    // //     to: Pos::at("a4"),
    // //     prom: None,
    // // };

    // // let ctx = engin.mv(&mv, white, true);
    // // engin.undo_mv(&mv, ctx);

    // // assert_eq!(engin, before);

    let white = true;
    let black = false;

    let turn = 100;
    let mut who_play = white;
    let mut end = 0;

    for i in 0..turn {
        end = i;
        let start = Instant::now();
        let mv = robot.minmax(3, &mut engin, who_play);
        let d = start.elapsed();
        println!(
            "{} thinking: {:?}",
            if who_play { "white" } else { "black" },
            d
        );
        match mv {
            Some(mve) => engin.mv(&mve, who_play, true),
            None => break,
        }

        if engin.draw_possible() {
            println!("possible draw, repetition > 3");
        }

        if engin.auto_draw() {
            println!("auto draw, repetition > 5");
            break;
        }

        if engin.fifty_move_rule() {
            println!("auto draw, fifty move rule");
            break;
        }

        println!("{:?}", engin.history().last());
        who_play = !who_play;
    }

    println!("{}", engin);
    println!("end: {} halfturn", end);
}

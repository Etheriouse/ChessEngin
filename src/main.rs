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
    let start = Instant::now();
    robot.evaluate_v1(&engin, white);
    let duration1 = start.elapsed();

    let start = Instant::now();
    robot.evaluate_v2(&engin, white);
    let duration2 = start.elapsed();

    let start = Instant::now();
    robot.evaluate_v3(&engin, white);
    let duration3 = start.elapsed();

    let start = Instant::now();
    robot.evaluate_v4(&engin, white);
    let duration4 = start.elapsed();

    println!("time evaluate_v1: {:?}", duration1);
    println!("time evaluate_v2: {:?}", duration2);
    println!("time evaluate_v3: {:?}", duration3);
    println!("time evaluate_v4: {:?}", duration4);

    let turn = 50;
    let mut who_play = white;
    let mut end = 0;
    let mut half_mv = String::new();

    for i in 0..turn {
        end = i;
        let start = Instant::now();
        let mv = robot.minmax(if who_play { 2 } else { 3 }, &mut engin, who_play);
        let d = start.elapsed();
        println!(
            "{} thinking: {:?}",
            if who_play { "white" } else { "black" },
            d
        );
        match mv {
            Some(mve) => engin.mv(&mve, who_play, true),
            None => {
                println!("nothing to play for {}", if who_play { "white" } else {"black"});
                break;
            },
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

        // println!("{:?}", if engin.history().last() );
        match engin.history().last() {
            Some(mv) => half_mv += mv.to_string().as_str(),
            None => (),
        }
        half_mv += " ";

        if !who_play && i > 0 {
            println!("{}: {}", i / 2, half_mv);
            half_mv = "".to_string();
        }
        println!("{}", engin);
        who_play = !who_play;
    }
    
    println!("mate white: {}", engin.checkmate(white));
    println!("mate black: {}", engin.checkmate(!white));
    println!("pat white: {}", engin.stalemate(white));
    println!("pat black: {}", engin.stalemate(!white));
    println!("end: {} halfturn", end);
}

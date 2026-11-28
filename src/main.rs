use std::{
    io::Write,
    ops::Sub,
    println,
    time::{Duration, Instant},
};

use crate::{
    chess::Pos,
    engin::{Engin, Move},
    robot::Robot,
};

mod chess;
mod engin;
mod robot;

fn main() {
    // // let before = engin.clone();

    // // let mv = Move {
    // //     from: Pos::at("a2"),
    // //     to: Pos::at("a4"),
    // //     prom: None,
    // // };

    // // let ctx = engin.mv(&mv, white, true);
    // // engin.undo_mv(&mv, ctx);

    // // assert_eq!(engin, before);

    // let white = true;
    // let start = Instant::now();
    // robot.evaluate_v1(&engin, white);
    // let duration1 = start.elapsed();

    // let start = Instant::now();
    // robot.evaluate_v2(&engin, white);
    // let duration2 = start.elapsed();

    // let start = Instant::now();
    // robot.evaluate_v3(&engin, white);
    // let duration3 = start.elapsed();

    // let start = Instant::now();
    // robot.evaluate_v4(&engin, white);
    // let duration4 = start.elapsed();

    // println!("time evaluate_v1: {:?}", duration1);
    // println!("time evaluate_v2: {:?}", duration2);
    // println!("time evaluate_v3: {:?}", duration3);
    // println!("time evaluate_v4: {:?}", duration4);

    // white mat by 2 white 3 black thinking power
    // 8 □ □ □ □ □ ♖ ♔ □
    // 7 ♙ ♙ ♙ □ □ □ ♙ ♙
    // 6 □ □ □ □ ♖ □ □ □
    // 5 □ □ □ □ ♙ □ □ □
    // 4 □ ♗ ♛ ♘ ♟ □ □ □
    // 3 □ □ □ □ ♟ □ □ □
    // 2 ♟ ♟ □ □ □ □ ♟ ♕
    // 1 □ □ □ ♜ □ ♚ □ □
    //   a b c d e f g h

    let mut engin = Engin::classic(true);
    let robot = Robot::new();
    let strenght = 4;

    let max_turn = 200;
    let mut who_play = true;
    let mut half_mv = String::new();
    let mut black_time = Duration::new(60 * 20, 0);
    let mut white_time = Duration::new(60 * 20, 0);

    let humain = true;

    let mut mv;
    let mut d;
    for i in 0..(max_turn + 1) {
        println!("{}", engin);
        if !(humain ^ who_play) {
            loop {
                let mut input = String::new();

                print!("Move: ");
                std::io::stdout().flush().unwrap();

                let start = Instant::now();
                std::io::stdin().read_line(&mut input).unwrap();
                d = start.elapsed();

                let input = input.trim();

                if input.len() != 4
                    || !input.as_bytes()[0..4]
                        .chunks_exact(2)
                        .all(|s| (b'a'..=b'h').contains(&s[0]) && (b'1'..=b'8').contains(&s[1]))
                {
                    println!("Invalid move format!");
                    continue;
                }

                let from = Pos::at(&input[0..2]);
                let to = Pos::at(&input[2..4]);

                let tmp = Move {
                    from,
                    to,
                    prom: None,
                };

                if engin.moves(&from).contains(&tmp) {
                    mv = Some(tmp);
                    break;
                }

                println!("Illegal move!");
            }
        } else {
            let start = Instant::now();
            mv = robot.minmax(strenght, &mut engin, !humain);
            d = start.elapsed();
        }
        if who_play {
            white_time = white_time.saturating_sub(d);
        } else {
            black_time = black_time.saturating_sub(d);
        }

        match mv {
            Some(mve) => engin.mv(&mve, who_play, true),
            None => {
                println!(
                    "nothing to play for {}",
                    if who_play { "white" } else { "black" }
                );
                break;
            }
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

        if engin.checkmate(!who_play) {
            println!("mate");
            break;
        }

        if engin.stalemate(!who_play) {
            println!("mate");
            break;
        }

        if white_time.is_zero() {
            println!("time white");
            break;
        }

        if black_time.is_zero() {
            println!("time black");
            break;
        }



        match engin.history().last() {
            Some(mv) => half_mv += mv.to_string().as_str(),
            None => (),
        }
        half_mv += " ";

        if !who_play && i > 0 {
            println!("{}: {}", i / 2, half_mv);
            println!(
                "time: w: {}min {}s b: {}min {}s",
                white_time.as_secs() / 60,
                white_time.as_secs() % 60,
                black_time.as_secs() / 60,
                black_time.as_secs() % 60
            );
            half_mv = "".to_string();
        }
        who_play = !who_play;
    }
}

pub fn ia_vs_ia() {
    let mut engin = Engin::classic(true);
    let robot = Robot::new();
    let white = true;
    let turn = 300;
    let mut who_play = white;
    let mut half_mv = String::new();
    let mut black_time = Duration::new(60 * 20, 0);
    let mut white_time = Duration::new(60 * 20, 0);

    for i in 0..turn {
        let start = Instant::now();
        let mv = robot.minmax(3, &mut engin, who_play);
        let d = start.elapsed();
        println!(
            "{} thinking: {:.3?}",
            if who_play { "white" } else { "black" },
            d
        );
        if who_play {
            white_time = white_time.sub(d);
        } else {
            black_time = black_time.sub(d);
        }
        match mv {
            Some(mve) => engin.mv(&mve, who_play, true),
            None => {
                println!(
                    "nothing to play for {}",
                    if who_play { "white" } else { "black" }
                );
                break;
            }
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

        println!("{}", engin);
        if !who_play && i > 0 {
            println!("{}: {}", i / 2, half_mv);
            println!(
                "time: w: {}min {}s b: {}min {}s",
                white_time.as_secs() / 60,
                white_time.as_secs() % 60,
                black_time.as_secs() / 60,
                black_time.as_secs() % 60
            );
            half_mv = "".to_string();
        }
        who_play = !who_play;
    }

    println!("mate white: {}", engin.checkmate(white));
    println!("mate black: {}", engin.checkmate(!white));
    println!("pat white: {}", engin.stalemate(white));
    println!("pat black: {}", engin.stalemate(!white));
    println!("history: {:?}", engin.history());
}

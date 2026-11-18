pub mod chess_lib;
pub mod engin;

use crate::{
    chess_lib::chess_lib::{Piece, PieceKind, Pos},
    engin::Engin,
};
use std::{borrow, ops::Deref, println};

// todo a faire pour la pass
// - mettre a jour la pass si pawn moved en 2
// - mettre a jour la pass si pass != None et que autre piece est jouer ou pawn jour ailleurs
// https://fr.wikipedia.org/wiki/En_passant_(%C3%A9checs)

use sola_raylib::prelude::*;

fn main() {
    let piece = Piece {
        color: true,
        kind: PieceKind::Knight,
    };

    let a8 = Pos::new("e7");
    let a1 = Pos::new_c(4, 4);

    println!("{}", piece);
    println!(
        "a8: {} a1: {}",
        match a8 {
            Some(p) => p,
            None => {
                println!("undefined pos");
                return;
            }
        },
        a1
    );

    let rook = Pos::new("g2").unwrap();
    let mut chess = Engin::classic(true);
    chess.update_control();
    println!("{}", chess);
    // chess.place(Piece::new(true, PieceKind::Bishop), &rook);

    let moves = chess.get_moves(&rook);
    let show = moves
        .iter()
        .map(|p| p.to_string()) // Appelle automatiquement votre implémentation de Display
        .collect::<Vec<_>>()
        .join(", ");
    println!("possible move for {}: [{:?}]", rook, show);

    // Application
    let background = Color::new(32, 32, 32, 255);

    // Échiquier
    let light = Color::new(201, 185, 154, 255);
    let dark = Color::new(56, 56, 56, 255);

    // Pièces
    let white_piece = Color::new(226, 222, 212, 255);
    let black_piece = Color::new(98, 102, 106, 255);

    let white_controle = Color::new(255, 255, 255, 70);
    let black_controle = Color::new(0, 0, 0, 70);

    // Interactions
    let possible = Color::new(139, 175, 131, 255);
    let last_move = Color::new(166, 160, 120, 255);
    let selected = Color::new(214, 189, 131, 255);
    let check = Color::new(182, 92, 92, 255);
    let checkmate = Color::new(132, 47, 54, 255);
    let stalemate = Color::new(112, 132, 145, 150); // #708491

    // textes
    let text_primary = Color::new(226, 222, 212, 255); // #E2DED4
    let text_secondary = Color::new(160, 158, 150, 255); // #A09E96
    let text_disabled = Color::new(100, 100, 96, 255); // #646460

    // Accent
    let text_accent = Color::new(201, 185, 154, 255); // #C9B99A

    let view_control = true;

    let mut width: i32 = 1600;
    let mut height: i32 = 900;

    let texture_size = 256.0;
    let mut sprite_size: i32 =
        (((if width > height { height } else { width }) / 8) as f32 * 0.90) as i32;
    let mut xstart_chess = (width - sprite_size * 8) / 2;
    let mut ystart_chess = (height - (sprite_size * 8)) / 2;

    // let mut xstart_eat_you = ();

    let (mut rl, thread) = sola_raylib::init()
        .size(width, height)
        .title("Chess")
        .resizable()
        .build();

    let t_king: Texture2D = rl
        .load_texture(&thread, "assets/king.png")
        .expect("Impossible de charger la texture");
    let t_queen: Texture2D = rl
        .load_texture(&thread, "assets/queen.png")
        .expect("Impossible de charger la texture");
    let t_rook: Texture2D = rl
        .load_texture(&thread, "assets/rook.png")
        .expect("Impossible de charger la texture");
    let t_bishop: Texture2D = rl
        .load_texture(&thread, "assets/bishop.png")
        .expect("Impossible de charger la texture");
    let t_knight: Texture2D = rl
        .load_texture(&thread, "assets/knight.png")
        .expect("Impossible de charger la texture");
    let t_pawn: Texture2D = rl
        .load_texture(&thread, "assets/pawn.png")
        .expect("Impossible de charger la texture");

    rl.set_target_fps(30);

    let mut from: Option<Pos> = None;
    let mut turn = true; // white alway start
    // true => ia false => humain
    let opp = false;

    // println!("white king: {}", chess.data(true).king);
    // chess
    //     .modify_data(true)
    //     .eat
    //     .insert(Piece::new(false, PieceKind::Bishop), 1);
    // chess
    //     .modify_data(true)
    //     .eat
    //     .insert(Piece::new(false, PieceKind::Knight), 1);
    // chess
    //     .modify_data(false)
    //     .eat
    //     .insert(Piece::new(true, PieceKind::Bishop), 1);

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            height = rl.get_render_height();
            width = rl.get_render_width();
            sprite_size =
                (((if width > height { height } else { width }) / 8) as f32 * 0.90) as i32;
            xstart_chess = (width - sprite_size * 8) / 2;
            ystart_chess = (height - (sprite_size * 8)) / 2;
        }

        let pos = rl.get_mouse_position();

        let j = ((pos.x - xstart_chess as f32) / (sprite_size as f32)) as i8;
        let i = ((pos.y - ystart_chess as f32) / (sprite_size as f32)) as i8;

        match from {
            None => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    // check si can from

                    if match chess.get(&Pos::new_c(i, j)) {
                        Some(piece) => !(piece.color ^ if opp { chess.played } else { turn }),
                        None => false,
                    } {
                        from = Some(Pos::new_c(i, j));
                    }
                }
            }
            Some(_) => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    if opp {
                        chess.move_piece(&mut from, &Pos::new_c(i, j), chess.played);
                        // resolver.move_piece(chess);
                    } else {
                        if chess.move_piece(&mut from, &Pos::new_c(i, j), turn) {
                            turn = !turn;
                        }
                    }
                }
            }
        }

        // if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        //     // println!("click {}", Pos::new_c(8 - i, j));

        //     match from {
        //         Some(p) => {
        //             chess.move_piece(&p, &Pos::new_c(i, j));
        //             from = None;
        //         }
        //         None => from = Some(Pos::new_c(i, j)),
        //     }
        // }

        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(background);

        draw.draw_text(
            "you:",
            10,
            10,
            (sprite_size as f32 * 0.70) as i32,
            text_primary,
        );

        let mut entries_white: Vec<_> = chess.data(chess.played).eat.iter().collect();

        entries_white.sort_by_key(|(piece, _)| match piece.kind {
            PieceKind::King => 1,
            PieceKind::Queen => 2,
            PieceKind::Rook => 3,
            PieceKind::Bishop => 4,
            PieceKind::Knight => 5,
            PieceKind::Pawn => 6,
        });

        let mut i_w = 0;
        for (piece, value) in entries_white {
            draw.draw_texture_pro(
                match piece.kind {
                    PieceKind::King => &t_king,
                    PieceKind::Queen => &t_queen,
                    PieceKind::Rook => &t_rook,
                    PieceKind::Bishop => &t_bishop,
                    PieceKind::Knight => &t_knight,
                    PieceKind::Pawn => &t_pawn,
                },
                Rectangle::new(0.0, 0.0, texture_size, texture_size),
                Rectangle::new(
                    (sprite_size as f32 * 0.5) as f32,
                    (i_w as i32 * (sprite_size + 20)) as f32 + (height as f32 * 0.15),
                    sprite_size as f32,
                    sprite_size as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                if piece.color {
                    white_piece
                } else {
                    black_piece
                },
            );
            draw.draw_text(
                &value.to_string(),
                ((sprite_size as f32) * 1.5) as i32,
                ((i_w as i32 * (sprite_size + 20)) as f32 + (height as f32 * 0.15)) as i32,
                (sprite_size as f32 * 0.75) as i32,
                text_primary,
            );
            i_w += 1;
        }

        draw.draw_text(
            "they:",
            10 + xstart_chess + sprite_size * 8 as i32,
            10,
            (sprite_size as f32 * 0.70) as i32,
            text_primary,
        );

        let mut entries_black: Vec<_> = chess.data(!chess.played).eat.iter().collect();

        entries_black.sort_by_key(|(piece, _)| match piece.kind {
            PieceKind::King => 1,
            PieceKind::Queen => 2,
            PieceKind::Rook => 3,
            PieceKind::Bishop => 4,
            PieceKind::Knight => 5,
            PieceKind::Pawn => 6,
        });

        let mut i_b: i32 = 0;
        for (piece, value) in entries_black {
            draw.draw_texture_pro(
                match piece.kind {
                    PieceKind::King => &t_king,
                    PieceKind::Queen => &t_queen,
                    PieceKind::Rook => &t_rook,
                    PieceKind::Bishop => &t_bishop,
                    PieceKind::Knight => &t_knight,
                    PieceKind::Pawn => &t_pawn,
                },
                Rectangle::new(0.0, 0.0, texture_size, texture_size),
                Rectangle::new(
                    (xstart_chess as f32 + (sprite_size as f32 * 8.5)) as f32,
                    // (xstart_chess as f32 - sprite_size as f32 * 2.0) as f32,
                    (i_b as i32 * (sprite_size + 20)) as f32 + (height as f32 * 0.15),
                    sprite_size as f32,
                    sprite_size as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                if piece.color {
                    white_piece
                } else {
                    black_piece
                },
            );
            draw.draw_text(
                &value.to_string(),
                (xstart_chess as f32 + (sprite_size as f32) * 9.5) as i32,
                ((i_b as i32 * (sprite_size + 20)) as f32 + (height as f32 * 0.15)) as i32,
                (sprite_size as f32 * 0.75) as i32,
                text_primary,
            );
            i_b += 1;
        }

        // for (key, value) in chess.data(true).eat.iter() {}

        //chess.data(true)
        for i in 0..8 {
            draw.draw_text(
                &(i + 1).to_string(),
                xstart_chess - (sprite_size as f32 * 0.6) as i32,
                i as i32 * sprite_size + ystart_chess + (sprite_size as f32 * 0.2) as i32,
                50,
                text_secondary,
            );
            for j in 0..8 {
                let mut king_case_check = false;
                let mut king_case_checkmate = false;
                let mut king_case_stalemate = false;
                match chess.get(&Pos::new_c(i, j)) {
                    Some(p) => match p.kind {
                        PieceKind::King => {
                            king_case_check = chess.check(p.color);
                            king_case_checkmate = chess.checkmate(p.color);
                            king_case_stalemate = chess.stalemate(p.color);
                        }
                        _ => (),
                    },
                    None => (),
                }
                draw.draw_rectangle(
                    j as i32 * sprite_size + xstart_chess,
                    (i as i32 * sprite_size) + ystart_chess,
                    sprite_size,
                    sprite_size,
                    if match from {
                        None => false,
                        Some(p) => {
                            (p.i == i && p.j == j)
                                && match chess.get(&p) {
                                    Some(piece) => {
                                        !(piece.color ^ if opp { chess.played } else { turn })
                                    }
                                    None => false,
                                }
                        }
                    } {
                        selected
                    } else if king_case_check {
                        check
                    } else if king_case_checkmate {
                        checkmate
                    } else if king_case_stalemate {
                        stalemate
                    } else {
                        if (i + j) % 2 == 0 { light } else { dark }
                    },
                );

                if view_control {
                    if chess.is_controled(true, &Pos::new_c(i, j)) {
                        draw.draw_rectangle(
                    j as i32 * sprite_size + xstart_chess,
                    (i as i32 * sprite_size) + ystart_chess,
                    sprite_size,
                    sprite_size, white_controle);
                    }
                    if chess.is_controled(false, &Pos::new_c(i, j)) {
                        draw.draw_rectangle(
                    j as i32 * sprite_size + xstart_chess,
                    (i as i32 * sprite_size) + ystart_chess,
                    sprite_size,
                    sprite_size, black_controle);
                    }
               
                }

                match chess.get(&Pos::new_c(i, j)) {
                    Some(p) => draw.draw_texture_pro(
                        match p.kind {
                            PieceKind::King => &t_king,
                            PieceKind::Queen => &t_queen,
                            PieceKind::Rook => &t_rook,
                            PieceKind::Bishop => &t_bishop,
                            PieceKind::Knight => &t_knight,
                            PieceKind::Pawn => &t_pawn,
                        },
                        Rectangle::new(0.0, 0.0, texture_size, texture_size),
                        Rectangle::new(
                            (j as i32 * sprite_size + xstart_chess) as f32,
                            ((i as i32 * sprite_size) + ystart_chess) as f32,
                            sprite_size as f32,
                            sprite_size as f32,
                        ),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        if p.color { white_piece } else { black_piece },
                    ),
                    None => (),
                }
            }
        }

        for i in 0..8 {
            draw.draw_text(
                &((b'a' + i as u8) as char).to_string(),
                xstart_chess + (sprite_size as f32 * 0.4) as i32 + sprite_size * i,
                ystart_chess + sprite_size * 8,
                50,
                text_secondary,
            );
        }
    }
}

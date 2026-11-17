pub mod chess_lib;
pub mod engin;

use crate::{
    chess_lib::chess_lib::{Piece, PieceKind, Pos},
    engin::Engin,
};
use std::println;

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

    // Interactions
    let possible = Color::new(139, 175, 131, 255);
    let last_move = Color::new(166, 160, 120, 255);
    let selected = Color::new(214, 189, 131, 255);
    let check = Color::new(182, 92, 92, 255);
    let checkmate = Color::new(132, 47, 54, 255);

    let sprite_size: i32 = 96;

    let (mut rl, thread) = sola_raylib::init().size(1600, 900).title("Chess").build();

    let t_king: Texture2D = rl
        .load_texture(&thread, "assets/piece/white/king.png")
        .expect("Impossible de charger la texture");
    let t_queen: Texture2D = rl
        .load_texture(&thread, "assets/piece/white/queen.png")
        .expect("Impossible de charger la texture");
    let t_rook: Texture2D = rl
        .load_texture(&thread, "assets/piece/white/rook.png")
        .expect("Impossible de charger la texture");
    let t_bishop: Texture2D = rl
        .load_texture(&thread, "assets/piece/white/bishop.png")
        .expect("Impossible de charger la texture");
    let t_knight: Texture2D = rl
        .load_texture(&thread, "assets/piece/white/knight.png")
        .expect("Impossible de charger la texture");
    let t_pawn: Texture2D = rl
        .load_texture(&thread, "assets/piece/white/pawn.png")
        .expect("Impossible de charger la texture");

    rl.set_target_fps(30);

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(background);

        for i in 0..8 {
            for j in 0..8 {
                draw.draw_rectangle(
                    j as i32 * sprite_size,
                    i as i32 * sprite_size,
                    sprite_size,
                    sprite_size,
                    if (i + j) % 2 == 0 { light } else { dark },
                );
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
                        Rectangle::new(0.0, 0.0, 32.0, 32.0),
                        Rectangle::new(
                            (j as i32 * sprite_size) as f32,
                            (i as i32 * sprite_size) as f32,
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
    }
}

mod caster;
mod framebuffer;
mod maze;
mod player;

use caster::cast_ray;
use framebuffer::Framebuffer;
use maze::{Maze, load_maze};
use player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;

const BLOCK_SIZE: usize = 40;

/// Pinta un bloque sólido de `size` x `size` con la esquina en (x0, y0).
fn draw_block(framebuffer: &mut Framebuffer, x0: i32, y0: i32, size: i32, color: Color) {
    framebuffer.set_current_color(color);
    for y in y0..y0 + size {
        for x in x0..x0 + size {
            framebuffer.set_pixel(x, y);
        }
    }
}

/// Dibuja el laberinto completo: cada caracter del archivo es una celda.
fn render_maze(framebuffer: &mut Framebuffer, maze: &Maze) {
    let wall_color = Color::new(80, 110, 200, 255);
    let goal_color = Color::new(60, 200, 100, 255);
    let block = BLOCK_SIZE as i32;

    for y in 0..maze.height() {
        for x in 0..maze.width() {
            let x0 = x as i32 * block;
            let y0 = y as i32 * block;

            if maze.is_wall(x, y) {
                draw_block(framebuffer, x0, y0, block, wall_color);
            } else if maze.get(x, y) == 'g' {
                draw_block(framebuffer, x0, y0, block, goal_color);
            }
        }
    }
}

/// Dibuja al jugador como un cuadrito centrado en su posición.
fn render_player(framebuffer: &mut Framebuffer, player: &Player) {
    let size = (BLOCK_SIZE as i32 / 2).max(2);
    draw_block(
        framebuffer,
        player.pos.x as i32 - size / 2,
        player.pos.y as i32 - size / 2,
        size,
        Color::new(255, 220, 0, 255),
    );
}

fn render(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    framebuffer.clear();
    render_maze(framebuffer, maze);

    // El rayo va antes que el jugador para que el cuadrito quede encima.
    let hit = cast_ray(framebuffer, maze, player, player.a, BLOCK_SIZE, true);
    render_player(framebuffer, player);

    println!(
        "Mirando a {:.2} rad, pared '{}' a {:.1} px",
        player.a, hit.impact, hit.distance
    );
}

fn main() {
    let maze = load_maze("maze.txt");

    let width = (maze.width() * BLOCK_SIZE) as i32;
    let height = (maze.height() * BLOCK_SIZE) as i32;

    let (start_x, start_y) = maze
        .player_start()
        .expect("El laberinto no tiene una 'p' para el jugador");
    let player = Player::at_cell(start_x, start_y, BLOCK_SIZE, PI / 3.0, PI / 3.0);

    let (mut window, thread) = raylib::init()
        .size(width, height)
        .title("Laberinto - vista 2D")
        .build();
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.set_background_color(Color::new(25, 25, 35, 255));
    render(&mut framebuffer, &maze, &player);

    println!(
        "Jugador en celda {:?}, meta en {:?}, fov {:.2} rad",
        (start_x, start_y),
        maze.goal(),
        player.fov
    );

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_S) {
            framebuffer.render_to_file("maze.png");
            println!("Captura guardada en maze.png");
        }
        framebuffer.swap_buffers(&mut window, &thread);
    }
}

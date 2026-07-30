mod framebuffer;
mod maze;

use framebuffer::Framebuffer;
use maze::{Maze, load_maze};
use raylib::prelude::*;

const BLOCK_SIZE: i32 = 40;

/// Pinta un bloque sólido de BLOCK_SIZE x BLOCK_SIZE con la esquina en (x0, y0).
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
    let player_color = Color::new(255, 220, 0, 255);

    for y in 0..maze.height() {
        for x in 0..maze.width() {
            let x0 = x as i32 * BLOCK_SIZE;
            let y0 = y as i32 * BLOCK_SIZE;

            if maze.is_wall(x, y) {
                draw_block(framebuffer, x0, y0, BLOCK_SIZE, wall_color);
            } else if maze.get(x, y) == 'g' {
                draw_block(framebuffer, x0, y0, BLOCK_SIZE, goal_color);
            }
        }
    }

    // El jugador se dibuja al final, más pequeño y centrado en su celda.
    if let Some((px, py)) = maze.player_start() {
        let margin = BLOCK_SIZE / 4;
        draw_block(
            framebuffer,
            px as i32 * BLOCK_SIZE + margin,
            py as i32 * BLOCK_SIZE + margin,
            BLOCK_SIZE - margin * 2,
            player_color,
        );
    }
}

fn main() {
    let maze = load_maze("maze.txt");

    let width = maze.width() as i32 * BLOCK_SIZE;
    let height = maze.height() as i32 * BLOCK_SIZE;

    let (mut window, thread) = raylib::init()
        .size(width, height)
        .title("Laberinto - vista 2D")
        .build();
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.set_background_color(Color::new(25, 25, 35, 255));
    framebuffer.clear();
    render_maze(&mut framebuffer, &maze);

    println!(
        "Laberinto de {}x{} celdas. Jugador: {:?}, meta: {:?}",
        maze.width(),
        maze.height(),
        maze.player_start(),
        maze.goal()
    );

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_S) {
            framebuffer.render_to_file("maze.png");
            println!("Captura guardada en maze.png");
        }
        framebuffer.swap_buffers(&mut window, &thread);
    }
}

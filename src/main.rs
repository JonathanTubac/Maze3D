mod caster;
mod framebuffer;
mod maze;
mod player;

use caster::cast_ray;
use framebuffer::Framebuffer;
use maze::{Maze, load_maze};
use player::{Player, process_events};
use raylib::prelude::*;
use std::f32::consts::PI;

const BLOCK_SIZE: usize = 40;
/// Rayos del abanico en la vista 2D. En 3D se lanza uno por columna.
const NUM_RAYS_2D: usize = 120;

/// Vista activa: el mapa desde arriba o la proyección desde los ojos del jugador.
#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Map2D,
    World3D,
}

/// Ángulo del rayo `i` de `total`, repartidos dentro del fov del jugador.
/// El primero sale en a - fov/2 y de ahí avanza una fracción del fov.
fn ray_angle(player: &Player, i: usize, total: usize) -> f32 {
    let current_ray = i as f32 / total as f32;
    player.a - (player.fov / 2.0) + (player.fov * current_ray)
}

/// Pinta un bloque sólido de `size` x `size` con la esquina en (x0, y0).
fn draw_block(framebuffer: &mut Framebuffer, x0: i32, y0: i32, size: i32, color: Color) {
    framebuffer.set_current_color(color);
    framebuffer.fill_rect(x0, y0, size, size);
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

/// Vista de arriba: el laberinto, el abanico de rayos y el jugador.
fn render_map2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    framebuffer.clear();
    render_maze(framebuffer, maze);

    for i in 0..NUM_RAYS_2D {
        let a = ray_angle(player, i, NUM_RAYS_2D);
        cast_ray(framebuffer, maze, player, a, BLOCK_SIZE, true);
    }

    // El jugador va al final para que el cuadrito quede encima de los rayos.
    render_player(framebuffer, player);
}

/// Vista en primera persona: un rayo por columna de pantalla y cada distancia
/// se convierte en la altura de esa columna de pared.
fn render_world3d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let half_height = height as f32 / 2.0;
    let num_rays = width as usize;

    // Distancia al plano de proyección: con esto una pared a BLOCK_SIZE de
    // distancia ocupa justo el alto de la pantalla.
    let distance_to_plane = (width as f32 / 2.0) / (player.fov / 2.0).tan();

    let sky = Color::new(25, 25, 45, 255);
    let floor = Color::new(50, 45, 40, 255);

    for i in 0..num_rays {
        let a = ray_angle(player, i, num_rays);
        let intersect = cast_ray(framebuffer, maze, player, a, BLOCK_SIZE, false);

        // Corrección de ojo de pez: la distancia útil es la proyectada sobre
        // la dirección de vista, no la del rayo.
        let d = (intersect.distance * (a - player.a).cos()).max(0.1);
        let stake_height = (BLOCK_SIZE as f32 / d) * distance_to_plane;

        let top = (half_height - stake_height / 2.0) as i32;
        let bottom = (half_height + stake_height / 2.0) as i32;

        // Las paredes lejanas se oscurecen para dar sensación de profundidad.
        let shade = (1.0 - (d / 600.0)).clamp(0.25, 1.0);
        let base = match intersect.impact {
            'g' => Color::new(60, 200, 100, 255),
            '|' => Color::new(70, 95, 175, 255), // verticales un poco más oscuras
            _ => Color::new(95, 125, 215, 255),
        };
        let wall = Color::new(
            (base.r as f32 * shade) as u8,
            (base.g as f32 * shade) as u8,
            (base.b as f32 * shade) as u8,
            255,
        );

        // Tres tramos por columna: cielo, pared y piso.
        let x = i as i32;
        let top = top.clamp(0, height);
        let bottom = bottom.clamp(0, height);

        framebuffer.set_current_color(sky);
        framebuffer.fill_rect(x, 0, 1, top);
        framebuffer.set_current_color(wall);
        framebuffer.fill_rect(x, top, 1, bottom - top);
        framebuffer.set_current_color(floor);
        framebuffer.fill_rect(x, bottom, 1, height - bottom);
    }
}

fn render(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, mode: Mode) {
    match mode {
        Mode::Map2D => render_map2d(framebuffer, maze, player),
        Mode::World3D => render_world3d(framebuffer, maze, player),
    }
}

fn main() {
    let maze = load_maze("maze.txt");

    let width = (maze.width() * BLOCK_SIZE) as i32;
    let height = (maze.height() * BLOCK_SIZE) as i32;

    let (start_x, start_y) = maze
        .player_start()
        .expect("El laberinto no tiene una 'p' para el jugador");
    let mut player = Player::at_cell(start_x, start_y, BLOCK_SIZE, PI / 3.0, PI / 3.0);

    let (mut window, thread) = raylib::init()
        .size(width, height)
        .title("Laberinto")
        .build();
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.set_background_color(Color::new(25, 25, 35, 255));
    let mut mode = Mode::Map2D;

    println!(
        "Jugador en celda {:?}, meta en {:?}, fov {:.2} rad",
        (start_x, start_y),
        maze.goal(),
        player.fov
    );
    println!("W/S: avanzar | A/D: girar | M: cambiar 2D/3D | F1: guardar maze.png");

    while !window.window_should_close() {
        process_events(&mut player, &window, &maze, BLOCK_SIZE);

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == Mode::Map2D {
                Mode::World3D
            } else {
                Mode::Map2D
            };
        }

        render(&mut framebuffer, &maze, &player, mode);

        if window.is_key_pressed(KeyboardKey::KEY_F1) {
            framebuffer.render_to_file("maze.png");
            println!(
                "Captura en maze.png | pos ({:.0}, {:.0}) a {:.2} rad",
                player.pos.x, player.pos.y, player.a
            );
        }

        framebuffer.swap_buffers(&mut window, &thread);
    }
}

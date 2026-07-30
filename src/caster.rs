use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::*;

/// Resultado de lanzar un rayo: qué tan lejos quedó la pared y qué caracter era.
pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

/// Avanza un rayo desde el jugador en la dirección `a` hasta topar con una
/// pared. Si `draw_line` es true va pintando el camino recorrido.
pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    framebuffer.set_current_color(Color::new(230, 230, 240, 255));

    loop {
        // Descomposición trigonométrica: a distancia d sobre el ángulo a,
        // el avance es (d*cos a) en x y (d*sin a) en y.
        let x = player.pos.x + d * a.cos();
        let y = player.pos.y + d * a.sin();

        // Salir si el rayo se va del framebuffer (el mapa siempre tiene borde
        // de pared, pero esto evita un ciclo infinito si algún día no lo tiene).
        if x < 0.0 || y < 0.0 || x >= framebuffer.width as f32 || y >= framebuffer.height as f32 {
            return Intersect {
                distance: d,
                impact: ' ',
            };
        }

        // De pixeles a celda del laberinto.
        let i = x as usize / block_size;
        let j = y as usize / block_size;

        if maze.is_wall(i, j) {
            return Intersect {
                distance: d,
                impact: maze.get(i, j),
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as i32, y as i32);
        }

        d += 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::Maze;

    #[test]
    fn ray_hits_wall_at_expected_distance() {
        let cells: Vec<Vec<char>> = ["+--+", "|  |", "+--+"]
            .iter()
            .map(|r| r.chars().collect())
            .collect();
        let maze = Maze::new(cells);
        let mut fb = Framebuffer::new(40, 30);
        // Centro de la celda (1,1) con bloques de 10 px.
        let player = Player::at_cell(1, 1, 10, 0.0, 0.0);

        let hit = cast_ray(&mut fb, &maze, &player, 0.0, 10, false);
        assert_eq!(hit.impact, '|');
        assert!((hit.distance - 15.0).abs() < 1.5, "d = {}", hit.distance);

        // Hacia arriba (-PI/2) topa con la pared horizontal de arriba.
        let hit = cast_ray(&mut fb, &maze, &player, -std::f32::consts::PI / 2.0, 10, false);
        assert_eq!(hit.impact, '-');
        assert!((hit.distance - 5.0).abs() < 1.5, "d = {}", hit.distance);
    }
}

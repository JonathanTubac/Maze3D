use raylib::prelude::*;

/// El jugador es el punto de vista del mundo: dónde está y hacia dónde ve.
pub struct Player {
    /// Posición en pixeles dentro del framebuffer, no en celdas.
    pub pos: Vector2,
    /// Ángulo de vista en radianes (hacia dónde apunta la cabeza).
    pub a: f32,
    /// Campo de visión en radianes, se usará al proyectar en 3D.
    pub fov: f32,
}

impl Player {
    pub fn new(pos: Vector2, a: f32, fov: f32) -> Self {
        Player { pos, a, fov }
    }

    /// Construye al jugador al centro de la celda (cell_x, cell_y) del mapa.
    pub fn at_cell(cell_x: usize, cell_y: usize, block_size: usize, a: f32, fov: f32) -> Self {
        let half = block_size as f32 / 2.0;
        Player::new(
            Vector2::new(
                cell_x as f32 * block_size as f32 + half,
                cell_y as f32 * block_size as f32 + half,
            ),
            a,
            fov,
        )
    }
}

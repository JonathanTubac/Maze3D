use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;
        Framebuffer {
            width,
            height,
            color_buffer: Image::gen_image_color(width, height, background_color),
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    /// Rellena un rectángulo de un solo color. Hace lo mismo que llamar
    /// set_pixel en cada posición, pero en una sola llamada a raylib en vez
    /// de una por pixel, que es lo que cuesta cuando se redibuja cada frame.
    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        if width <= 0 || height <= 0 {
            return;
        }
        self.color_buffer
            .draw_rectangle(x, y, width, height, self.current_color);
    }

    /// Guarda el contenido actual del framebuffer como imagen.
    pub fn render_to_file(&self, path: &str) {
        self.color_buffer.export_image(path);
    }

    /// Sube el framebuffer a una textura y la dibuja en la ventana.
    pub fn swap_buffers(&self, window: &mut RaylibHandle, thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}

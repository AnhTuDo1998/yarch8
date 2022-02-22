use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Renderer {
    canvas: WindowCanvas,
    scale: u32,
    bg_color: Color,
    fg_color: Color,
}

impl Renderer {
    pub fn new(canvas: WindowCanvas, scale: u32) -> Self {
        Renderer {
            canvas,
            scale,
            bg_color: Color::RGB(0, 0, 0),
            fg_color: Color::RGB(255, 255, 255),
        }
    }

    // Use for both render and clear screen
    pub fn render_screen(&mut self, disp_buff: &[[bool; 64]; 32]) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();

        // logic to display render from boolean matrix
        self.canvas.set_draw_color(self.fg_color);
        // Render row by row...
        for (y, row) in disp_buff.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                // Draw a pixel if it is true
                if *pixel {
                    self.canvas
                        .fill_rect(Rect::new(
                            (x * self.scale as usize) as i32,
                            (y * self.scale as usize) as i32,
                            self.scale,
                            self.scale,
                        ))
                        .unwrap();
                }
            }
        }

        self.canvas.present();
    }
}

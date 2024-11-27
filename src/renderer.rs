use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::ttf::Font;
use crate::game_state::{Point, GameContext, GameState, Direction};
use crate::{DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};

pub struct Renderer { canvas: WindowCanvas, texture_creator: TextureCreator<WindowContext> }

impl Renderer {
    pub fn new(window: Window ) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
        Ok(Renderer { canvas , texture_creator})
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            (x + 1) * DOT_SIZE_IN_PXS,
            (y + 1) * DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS as u32,
            DOT_SIZE_IN_PXS as u32,
        ))?;
    
        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext, font: &Font<'_, 'static>) -> Result<(), String> {
        self.draw_background(context);
        self.draw_walls()?;
        self.draw_text(context, font)?;
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.canvas.present();
    
        Ok(())
    }

    pub fn draw_background(&mut self, context: &GameContext) {
        let color = match context.state {
            GameState::Running => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30,30,30),
            GameState::GameOver => Color::RGB(255,255,255)
        };

        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn draw_walls(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(100, 100, 100));
        self.canvas.fill_rect(Rect::new(0, 0, ((GRID_X_SIZE + 1) * DOT_SIZE_IN_PXS) as u32, DOT_SIZE_IN_PXS as u32))?;
        self.canvas.fill_rect(Rect::new(0, 0, DOT_SIZE_IN_PXS as u32, ((GRID_Y_SIZE + 1) * DOT_SIZE_IN_PXS) as u32))?;
        self.canvas.fill_rect(Rect::new(0, (GRID_Y_SIZE + 1) * DOT_SIZE_IN_PXS, ((GRID_X_SIZE + 1) * DOT_SIZE_IN_PXS) as u32, DOT_SIZE_IN_PXS as u32))?;
        self.canvas.fill_rect(Rect::new((GRID_X_SIZE + 1) * DOT_SIZE_IN_PXS, 0, DOT_SIZE_IN_PXS as u32, ((GRID_Y_SIZE + 2) * DOT_SIZE_IN_PXS) as u32))?;

        // Draw score in the top left wall
        
    
        Ok(())
    }

    pub fn draw_text(&mut self, context: &GameContext, font: &Font<'_, 'static>) -> Result<(), String> {
        let score_text = format!("Score: {}", context.score);
    
        // Calculate the appropriate font size
        let target_height = 20; // The desired height in pixels
        let font_size = font.height();
    
        // Adjust font size dynamically
        let scale_factor = target_height as f32 / font_size as f32;
    
        let surface = font
            .render(&score_text)
            .blended(Color::RGB(255, 255, 255)) // White text
            .map_err(|e| e.to_string())?;
    
        // Calculate the new dimensions
        let scaled_width = (surface.width() as f32 * scale_factor) as u32;
        let scaled_height = (surface.height() as f32 * scale_factor) as u32;
    
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
    
        let target = Rect::new(10, 0, scaled_width, scaled_height);
        self.canvas.copy(&texture, None, Some(target))?;

        if context.highscore.is_none() {
            return Ok(());
        }
    
        let highscore_text = format!("Highscore: {}", context.highscore.unwrap());

        let surface = font
            .render(&highscore_text)
            .blended(Color::RGB(255, 255, 255)) // White text
            .map_err(|e| e.to_string())?;

        let scaled_width = (surface.width() as f32 * scale_factor) as u32;

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = Rect::new(310, 0, scaled_width, scaled_height);
        self.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }

    pub fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for p in &context.snake {
            self.draw_dot(p)?;
        }
        
        let Point(mut x_coor, mut y_coor) = context.snake[0];
        x_coor += 1;
        y_coor += 1;
        let eyes = [Point(x_coor * DOT_SIZE_IN_PXS + 2, y_coor * DOT_SIZE_IN_PXS + 2),
            Point(x_coor * DOT_SIZE_IN_PXS + 12, y_coor * DOT_SIZE_IN_PXS + 2),
            Point(x_coor * DOT_SIZE_IN_PXS + 2, y_coor * DOT_SIZE_IN_PXS + 12),
            Point(x_coor * DOT_SIZE_IN_PXS + 12, y_coor * DOT_SIZE_IN_PXS + 12)
        ];
        let eyes_to_draw = match context.snake_dir {
            Direction::Up => vec![&eyes[0], &eyes[1]],
            Direction::Down => vec![&eyes[2], &eyes[3]],
            Direction::Left => vec![&eyes[0], &eyes[2]],
            Direction::Right => vec![&eyes[1], &eyes[3]],
        };

        self.canvas.set_draw_color(Color::WHITE);
        for eye in &eyes_to_draw {
            self.canvas.fill_rect(Rect::new(eye.0, eye.1, 6, 6))?;
            
        }

        self.canvas.set_draw_color(Color::BLACK);
        for eye in &eyes_to_draw {
            self.canvas.fill_rect(Rect::new(eye.0 + 2, eye.1 + 2, 2, 2))?;
        }

        Ok(())
    }

    pub fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;

        Ok(())
    }
}
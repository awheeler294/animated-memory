use ggez::{
    event::{self, EventHandler, KeyCode}, graphics::{self, DrawMode, Point2, Rect, Text, TextFragment}
};

pub const MAIN_MENU_TITLE: &str = "Animated Memory";
pub const PAUSE_MENU_TITLE: &str = "Paused";

pub const NEW_GAME: &str = "New Game";
pub const RESUME: &str = "Resume";
pub const MAIN_MENU: &str = "Main Menu";
pub const EXIT: &str = "Exit";

const V_PADDING: f32 = 35.0;

use crate::color_scheme::ColorPalette;

pub struct MainMenu<'a> {
    menu: Menu<'a>,
    show_resume: bool,
}

impl<'a> MainMenu<'a> {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(MAIN_MENU_TITLE, &[NEW_GAME, EXIT])
                .shade_menu_items(true),
            show_resume: false,
        }
    }

    pub fn selected_item(&self) -> &str {
        self.menu.selected_item()
    }

    pub fn show_resume(&mut self, show: bool) {
        if show != self.show_resume {
            if show {
                self.menu.menu_items.insert(0, RESUME);
            } else {
                self.menu.menu_items.remove(0);
            }

            self.show_resume = show;
        }
    }
}

impl<'a> event::EventHandler for MainMenu<'a> {
    fn update(&mut self, ctx: &mut ggez::Context, gctx: &mut event::GraphicsContext) -> Result<(), ggez::GameError> {
        self.menu.update(ctx, gctx)?;

        Ok(())        
    }

    fn draw(&mut self, ctx: &mut ggez::Context, gctx: &mut event::GraphicsContext) -> Result<(), ggez::GameError> {
        self.menu.draw(ctx, gctx)?;

        Ok(())        
    }

    fn key_down_event(
            &mut self,
            ctx: &mut ggez::Context,
            gctx: &mut event::GraphicsContext,
            keycode: KeyCode,
            keymods: event::KeyMods,
            repeat: bool,
        ) {
        self.menu.key_down_event(ctx, gctx, keycode, keymods, repeat)
    }
}

pub struct Menu<'a> {
    title: &'a str,
    menu_items: Vec<&'a str>,
    shade_background: bool,
    shade_menu_items: bool,

    selected_index: usize,
}

impl<'a> Menu<'a> {
    pub fn new(title: &'a str, menu_items: &[&'a str]) -> Self {
        Self {
            title,
            menu_items: menu_items.to_vec(),
            shade_background: false,
            shade_menu_items: false,
            selected_index: 0
        }
    }

    pub fn shade_background(mut self, val: bool) -> Self {
        self.shade_background = val;

        self
    }

    pub fn shade_menu_items(mut self, val: bool) -> Self {
        self.shade_menu_items = val;

        self
    }

    pub fn reset_selection(&mut self) {
        self.selected_index = 0;
    }

    pub fn selected_item(&self) -> &str {
        &self.menu_items[self.selected_index]
    }

    fn next_selection(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.menu_items.len()
    }

    fn prev_selection(&mut self) {
        self.selected_index = {
            if self.selected_index == 0 {
                self.menu_items.len() - 1
            } else {
                self.selected_index - 1
            }
        }
    }
}

impl<'a> EventHandler for Menu<'a> {
    fn update(&mut self, _ctx: &mut ggez::Context, _quad_ctx: &mut event::GraphicsContext) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut ggez::Context,
            _quad_ctx: &mut event::GraphicsContext,
            keycode: KeyCode,
            _keymods: event::KeyMods,
            _repeat: bool,
        ) {
        match keycode {
            KeyCode::Up => self.prev_selection(),
            KeyCode::Down => self.next_selection(),
            _ => (),
        }
        
    }

    fn draw(&mut self, ctx: &mut ggez::Context, gctx: &mut event::GraphicsContext) -> Result<(), ggez::GameError> {

        let (screen_width, screen_height) = graphics::drawable_size(gctx);

        if self.shade_background {
            let shade = graphics::MeshBuilder::new()
                    .rectangle(
                        DrawMode::fill(), 
                        Rect::new(
                            0.0,
                            0.0,
                            screen_width,
                            screen_height,
                        ), 
                        ColorPalette::TransparentBg.into(),
                    )?
                    .build(ctx, gctx)?;

                graphics::draw(ctx, gctx, &shade, (Point2::new(0.0, 0.0),))?;
        }

        let mut position = Point2::new(screen_width, screen_height / 3.0);

        let rendered = Text::new(
            TextFragment::new(self.title)
                .scale(96.0)
                .color(ColorPalette::Fg)
        );

        position.x = screen_width / 2.0 - rendered.width(ctx) / 2.0;

        graphics::draw(
            ctx,
            gctx,
            &rendered,
            (position,),
        )?;

        position.y += rendered.height(ctx) + V_PADDING * 3.0;

        for (i, menu_item) in self.menu_items.iter().enumerate() {
            let color = {
                if i == self.selected_index {
                    ColorPalette::BrightYellow
                } else {
                    ColorPalette::Fg
                }
            };

            let rendered = Text::new(
                TextFragment::new(*menu_item)
                    .scale(48.0)
                    .color(color)
            );

            position.x = screen_width / 2.0 - rendered.width(ctx) / 2.0;

            if i == self.selected_index {
                
                // draw selection box
                
                let image = graphics::MeshBuilder::new()
                    .rectangle(
                        DrawMode::stroke(3.0), 
                        Rect::new(
                            -5.0,
                            -5.0,
                            rendered.width(ctx) + 10.0,
                            rendered.height(ctx) + 10.0,
                        ), 
                        color.into(),
                    )?
                    .build(ctx, gctx)?;

                graphics::draw(ctx, gctx, &image, (position,))?;
            }

            graphics::draw(ctx, gctx, &rendered, (position,))?;

            position.y += rendered.height(ctx) + V_PADDING;
        }


        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_selection() {
        let menu_items = vec![
            "Item 1", 
            "Item 2", 
            "Item 3",
        ];

        let mut menu = Menu::new("Test Title", &menu_items);

        assert_eq!(menu.selected_index, 0);
        assert_eq!(menu.selected_item(), menu_items[0]);

        menu.next_selection();

        assert_eq!(menu.selected_index, 1);
        assert_eq!(menu.selected_item(), menu_items[1]);

        menu.next_selection();

        assert_eq!(menu.selected_index, 2);
        assert_eq!(menu.selected_item(), menu_items[2]);

        menu.next_selection();

        assert_eq!(menu.selected_index, 0);
        assert_eq!(menu.selected_item(), menu_items[0]);
    }

    #[test]
    fn test_prev_selection() {
        let menu_items = vec![
            "Item 1", 
            "Item 2", 
            "Item 3",
        ];

        let mut menu = Menu::new("Test Title", &menu_items);

        assert_eq!(menu.selected_index, 0);
        assert_eq!(menu.selected_item(), menu_items[0]);

        menu.prev_selection();

        assert_eq!(menu.selected_index, 2);
        assert_eq!(menu.selected_item(), menu_items[2]);

        menu.prev_selection();

        assert_eq!(menu.selected_index, 1);
        assert_eq!(menu.selected_item(), menu_items[1]);

        menu.prev_selection();

        assert_eq!(menu.selected_index, 0);
        assert_eq!(menu.selected_item(), menu_items[0]);
    }
}

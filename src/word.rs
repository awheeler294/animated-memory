use std::char;

use good_web_game::{
    Context,
    event, 
    GameResult, 
    GameError,
    graphics::{
        self,
        Point2,
        Text, 
        TextFragment,
        Vector2,
    },
    input::keyboard::KeyCode,
};

use keyframe::{functions::{EaseInOut, Linear}, AnimationSequence, Keyframe };

use crate::{
    ColorPalette, 
    TweenableColor
};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WordState {
    Active,
    Typed,
    Dead,
}

pub struct Word {
    pub state: WordState,
    pub num_typed: usize,

    word: Vec<char>,
    position: Point2,
    velocity: Vector2,
    color: ColorPalette,
    death_animation: AnimationSequence<TweenableColor>,
}

impl Word {
    pub fn new(word: &str, position: Point2, velocity: Vector2) -> Self {
        let animation_duration = 1.0;
        let mut death_animation = AnimationSequence::new();
        let _ = death_animation.insert(Keyframe::new(ColorPalette::Red.into(), 0.0, Linear));
        let _ = death_animation.insert(Keyframe::new(ColorPalette::Fg0.into(), animation_duration * 0.05, Linear));
        let _ = death_animation.insert(Keyframe::new(ColorPalette::Blue.into(), animation_duration * 0.45, EaseInOut));
        let _ = death_animation.insert(Keyframe::new(ColorPalette::Bg.into(), animation_duration, EaseInOut));

        Self { 
            word: word.chars().collect(), 
            num_typed: 0, 
            position, 
            velocity,
            color: ColorPalette::Fg,
            state: WordState::Active,
            death_animation,
            // death_animation: keyframes![
            //     (Color::from(ColorPalette::BrightYellow), 0.0, Linear),
            //     (Color::from(ColorPalette::Fg0), animation_duration * 0.05, Linear),
            // ],
        } 
    }

    pub fn with_color(mut self, color: ColorPalette) -> Self {
        self.color = color;

        self
    }

    pub fn update(&mut self, ctx: &mut Context, _gctx: &mut event::GraphicsContext, key_pressed: Option<KeyCode>) -> GameResult {
        if self.state == WordState::Typed && self.death_animation.finished() {
            self.state = WordState::Dead;
        }

        if self.state == WordState::Typed {
            self.death_animation.advance_by(ggez::timer::delta(ctx).as_secs_f64());
        }

        if let Some(next_ch) = self.word.get(self.num_typed) {
            if let Some(key_pressed) = key_pressed {
                let key_code = ch_to_keycode(*next_ch)
                    .ok_or_else(|| GameError::CustomError(format!("unmapped character: {next_ch}")))?;

                if key_pressed == key_code {
                    self.num_typed += 1;
                }

            }
            self.position += self.velocity;

        } else if self.state == WordState::Active {
            self.state = WordState::Typed;
        }
        
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, gctx: &mut event::GraphicsContext) -> GameResult {
        let typed_color = match self.state {
            WordState::Active => ColorPalette::Bg4.into(),
            WordState::Typed => self.death_animation.now_strict().unwrap_or_else(|| ColorPalette::Bg.into()),
            WordState::Dead => ColorPalette::Bg.into(),
        };

        let untyped_color = self.color;

        let typed = 
            TextFragment::new(self.word[0..self.num_typed].iter().collect::<String>())
            .scale(24.0)
            .color(typed_color);

        let mut rendered = Text::new(typed);
        rendered.add(
            TextFragment::new(self.word[self.num_typed..].iter().collect::<String>())
                .scale(24.0)
                .color(untyped_color)
        );

        // rendered.add(
        //     TextFragment::new(format!(" state: {:#?}", self.state)).color(ColorPalette::Fg4)
        // );

        let centered_position = Point2::new(
            self.position.x - rendered.width(ctx) / 2.0,
            self.position.y - rendered.height(ctx) / 2.0
        );
        graphics::draw(ctx, gctx, &rendered, (centered_position,))?;

        Ok(())
    }

}


fn ch_to_keycode(ch: char) -> Option<KeyCode> {
    match ch {
        '0' => Some(KeyCode::Key0),
        '1' => Some(KeyCode::Key1),
        '2' => Some(KeyCode::Key2),
        '3' => Some(KeyCode::Key3),
        '4' => Some(KeyCode::Key4),
        '5' => Some(KeyCode::Key5),
        '6' => Some(KeyCode::Key6),
        '7' => Some(KeyCode::Key7),
        '8' => Some(KeyCode::Key8),
        '9' => Some(KeyCode::Key9),
        'a' => Some(KeyCode::A),
        'b' => Some(KeyCode::B),
        'c' => Some(KeyCode::C),
        'd' => Some(KeyCode::D),
        'e' => Some(KeyCode::E),
        'f' => Some(KeyCode::F),
        'g' => Some(KeyCode::G),
        'h' => Some(KeyCode::H),
        'i' => Some(KeyCode::I),
        'j' => Some(KeyCode::J),
        'k' => Some(KeyCode::K),
        'l' => Some(KeyCode::L),
        'm' => Some(KeyCode::M),
        'n' => Some(KeyCode::N),
        'o' => Some(KeyCode::O),
        'p' => Some(KeyCode::P),
        'q' => Some(KeyCode::Q),
        'r' => Some(KeyCode::R),
        's' => Some(KeyCode::S),
        't' => Some(KeyCode::T),
        'u' => Some(KeyCode::U),
        'v' => Some(KeyCode::V),
        'w' => Some(KeyCode::W),
        'x' => Some(KeyCode::X),
        'y' => Some(KeyCode::Y),
        'z' => Some(KeyCode::Z),
        _ => None
    }
}



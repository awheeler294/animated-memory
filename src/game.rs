use std::{
    char,
    collections::HashSet, 
    f32::{self, consts::PI},
};

use good_web_game::{
    Context,
    event::{
        self,
        EventHandler,
    }, 
    GameResult, 
    GameError,
    graphics::{
        self,
        DrawMode,
        Point2,
        Text, 
        TextFragment,
        Vector2,
    },
    input::keyboard::{KeyCode, pressed_keys}, 
};
use keyframe::{functions::{EaseInOut, Linear}, AnimationSequence, Keyframe };
use rand::{prelude::SliceRandom, Rng, thread_rng};

use crate::{
    menu::{MainMenu, Menu, EXIT, MAIN_MENU, NEW_GAME, PAUSE_MENU_TITLE, RESUME}, 
    ColorPalette, 
    TweenableColor
};


pub enum GameState {
    Active,
    MainMenu,
    Paused,
}

use GameState::*;

pub struct GameManager<'a> {
    game_state: GameState,
    game: Game,
    main_menu: MainMenu<'a>,
    pause_menu: Menu<'a>
}

impl<'a> GameManager<'a> {
    pub fn new() -> Self {
        Self {
            game_state: MainMenu,
            game: Game::new(0.0, 0.0),
            main_menu: MainMenu::new(),
            pause_menu: Menu::new(PAUSE_MENU_TITLE, &[RESUME, MAIN_MENU, EXIT]).shade_background(true),
        }
    }
}

impl<'a> good_web_game::event::EventHandler for GameManager<'a> {

    fn update(&mut self, ctx: &mut Context, gctx: &mut event::GraphicsContext) -> Result<(), GameError> {
        match self.game_state {
            Active => self.game.update(ctx, gctx),
            MainMenu => self.main_menu.update(ctx, gctx),
            Paused => self.pause_menu.update(ctx, gctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context, gctx: &mut event::GraphicsContext) -> Result<(), GameError> {
        if let MainMenu = self.game_state {
            self.main_menu.draw(ctx, gctx)?;
        } else {
            self.game.draw(ctx, gctx)?;

            if let Paused = self.game_state {
                self.pause_menu.draw(ctx, gctx)?;
            }
        }

        // debug
        let (screen_width, screen_height) = graphics::drawable_size(gctx);

        let text = Text::new(TextFragment::new(format!("drawable_size: {screen_width}, {screen_height}")));
        graphics::draw(ctx, gctx, &text, (Point2::new(0.0, 0.0),))?;

        graphics::present(ctx, gctx)?;

        Ok(())
    }

    fn key_down_event(
            &mut self,
            ctx: &mut Context,
            gctx: &mut event::GraphicsContext,
            keycode: KeyCode,
            keymods: event::KeyMods,
            repeat: bool,
        ) {
        match self.game_state {
            
            Active => {
                if keycode == KeyCode::Escape {
                    self.game_state = Paused
                } else {
                    self.game.key_down_event(ctx, gctx, keycode, keymods, repeat)
                }
            },
            
            MainMenu => {
                if keycode == KeyCode::Enter {
                    let selected = self.main_menu.selected_item();
                    
                    if selected == NEW_GAME {
                        
                        let (screen_width, screen_height) = graphics::drawable_size(gctx);
                        self.game = Game::new(screen_width, screen_height);
                        self.game_state = Active;
                        self.main_menu.show_resume(true);

                    } else if selected == RESUME {
                        
                        self.game_state = Active;

                    } else if selected == EXIT {

                    }
                } else {
                    self.main_menu.key_down_event(ctx, gctx, keycode, keymods, repeat)
                }
            },

            Paused => {
                if keycode == KeyCode::Enter {
                    
                    let selected = self.pause_menu.selected_item();

                    if selected == RESUME {
                        self.game_state = Active;
                    } else if selected == EXIT {
                        
                    } else if selected == MAIN_MENU {
                        self.game_state = MainMenu
                    }

                    self.pause_menu.reset_selection();

                } else if keycode == KeyCode::Escape {
                    
                    self.game_state = Active;
                    
                    self.pause_menu.reset_selection();

                } else {
                    
                    self.pause_menu.key_down_event(ctx, gctx, keycode, keymods, repeat)
                }
            },
        }
        
    }
}

pub struct Game {
    player: Player,
    words: Vec<Word>,
    reset_typed: usize,
    keys_pressed: HashSet<KeyCode>,
}

impl Game {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {

        let player_radius = 4.0; 
        let player_position = Point2::new(screen_width / 2.0, screen_height - 30.0);
        
        let mut words = vec![];
        let radius = screen_height / 1.7;
        let center_x = screen_width / 2.0;
        let center_y = screen_height / 2.0 - 30.0;

        for (label, angle) in [
            ("0", 0.0), 
            ("15", 15.0), 
            ("30", 30.0), 
            ("45", 45.0), 
            ("60", 60.0), 
            ("75", 75.0), 
            ("90", 90.0), 
            ("105", 105.0), 
            ("120", 120.0), 
            ("135", 135.0), 
            ("150", 150.0), 
            ("165", 165.0), 
            ("180", 180.0), 
        ] {
            let theta = (angle - 180.0) * PI / 180.0;
            let x = radius * theta.cos() + center_x;
            let y = radius * theta.sin() + center_y * 2.0;

            let mut word = Word::new(label, Point2::new(x, y), Vector2::new(0.0, 0.0));
            word.color = ColorPalette::Bg2;
            words.push(word);
        }

        let mut word_list = Vec::from(WORD_LIST);
        word_list.shuffle(&mut thread_rng());

        for (i, word) in word_list.iter().enumerate() {
            let angle = rand::thread_rng().gen_range(0.0..=180.0);
            let rand_r = rand::thread_rng().gen_range(50.0..300.0);
            let r = radius + i as f32 * rand_r;
            let theta = (angle - 180.0) * PI / 180.0;
            let x = r * theta.cos() + center_x;
            let y = r * theta.sin() + center_y;

            words.push(Word::new(
                word, 
                Point2::new(x, y), 
                Vector2::new(
                    (player_position.x - x) / (500.0 + r / 2.0), 
                    (player_position.y - y) / (500.0 + r / 2.0)
                ))
            );
        }

        Self {
            player: Player::new(player_position, player_radius),
            words,
            reset_typed: 0,
            keys_pressed: HashSet::new(),
        }

    }
}

impl event::EventHandler for Game {
    fn update(&mut self, 
        ctx: &mut Context,
        gctx: &mut event::GraphicsContext,
    ) -> GameResult {

        let new_keypress = {
            let mut val = None;
            for key_code in pressed_keys(ctx) {
                if self.keys_pressed.contains(key_code) == false {
                    val = Some(key_code.clone());
                    break;
                }
            }
            
            val
        };

        self.keys_pressed = pressed_keys(ctx).clone();

        for word in self.words.iter_mut() {
            if self.reset_typed > 0 {
                if word.state == WordState::Active {
                    word.num_typed = 0;
                }
            } else {

                let old_state = word.state;

                word.update(ctx, gctx, new_keypress)?;
                
                if old_state == WordState::Active && word.state == WordState::Typed {
                    self.reset_typed = 2;
                    break;
                }
            }
        }

        self.reset_typed = self.reset_typed.saturating_sub(1);

        Ok(())
    }

    fn draw(&mut self, 
        ctx: &mut Context,
        gctx: &mut event::GraphicsContext,
    ) -> GameResult {

        graphics::clear(ctx, gctx, ColorPalette::Bg.into());

        for word in self.words.iter_mut() {
            word.draw(ctx, gctx)?;
        }

        self.player.draw(ctx, gctx)?;
        
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum WordState {
    Active,
    Typed,
    Dead,
}

struct Word {
    word: Vec<char>,
    num_typed: usize,
    position: Point2,
    velocity: Vector2,
    color: ColorPalette,
    state: WordState,
    death_animation: AnimationSequence<TweenableColor>,
}

impl Word {
    fn new(word: &str, position: Point2, velocity: Vector2) -> Self {
        let animation_duration = 1.0;
        let mut death_animation = AnimationSequence::new();
        let _ = death_animation.insert(Keyframe::new(ColorPalette::BrightYellow.into(), 0.0, Linear));
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

    fn update(&mut self, ctx: &mut Context, _gctx: &mut event::GraphicsContext, key_pressed: Option<KeyCode>) -> GameResult {
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

    fn draw(&mut self, ctx: &mut Context, gctx: &mut event::GraphicsContext) -> GameResult {
        let typed_color = match self.state {
            WordState::Active => ColorPalette::BrightYellow.into(),
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

struct Player {
    position: Point2,
    radius: f32,
    precision: f32,
}

impl Player {
    fn new(position: Point2, radius: f32) -> Self {
        Self { position, radius, precision: 0.01 }

    }
}

impl EventHandler for Player {
    fn update(&mut self, _ctx: &mut Context, _gctx: &mut event::GraphicsContext) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, gctx: &mut event::GraphicsContext) -> Result<(), GameError> {
        let image = graphics::MeshBuilder::new()
            .circle(DrawMode::fill(), Point2::new(-1.0 * self.radius,  -1.0 * self.radius), self.radius, self.precision, ColorPalette::Orange.into())?
            .build(ctx, gctx)?;

        graphics::draw(ctx, gctx, &image, (self.position,))?;

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

const WORD_LIST: [&str; 171] = [
    "and",		
    "are",		
    "ape",		
    "ace",		
    "act",		
    "ask",		
    "arm",		
    "age",		
    "ago",		
    "air",		
    "ate",		
    "all",		
    "but",		
    "bye",		
    "bad",		
    "big",		
    "bed",		
    "bat",		
    "boy",		
    "bus",		
    "bag",		
    "box",		
    "bit",		
    "bee",		
    "buy",		
    "bun",		
    "cub",		
    "cat",		
    "car",		
    "cut",		
    "cow",		
    "cry",		
    "cab",		
    "can",		
    "dad",		
    "dab",		
    "dam",		
    "did",		
    "dug",		
    "den",		
    "dot",		
    "dip",		
    "day",		
    "ear",		
    "eye",		
    "eat",		
    "end",		
    "elf",		
    "egg",		
    "far",		
    "fat",		
    "few",		
    "fan",		
    "fun",		
    "fit",		
    "fin",		
    "fox",		
    "fix",
    "fly",
    "fry",
    "for",
    "got",
    "get",
    "god",
    "gel",
    "gas",
    "hat",
    "hit",
    "has",
    "had",
    "how",
    "her",
    "his",
    "hen",
    "ink",
    "ice",
    "ill",
    "jab",
    "jug",
    "jet",
    "jam",
    "jar",
    "job",
    "jog",
    "kit",
    "key",
    "lot",
    "lit",
    "let",
    "lay",
    "mat",
    "man",
    "mad",
    "mug",
    "mix",
    "map",
    "mum",
    "mud",
    "mom",
    "may",
    "met",
    "net",
    "new",
    "nap",
    "now",
    "nod",
    "net",
    "not",
    "nut",
    "oar",
    "one",
    "out",
    "owl",
    "old",
    "own",
    "odd",
    "our",
    "pet",
    "pat",
    "peg",
    "paw",
    "pup",
    "pit",
    "put",
    "pot",
    "pop",
    "pin",
    "rat",
    "rag",
    "rub",
    "row",
    "rug",
    "run",
    "rap",
    "ram",
    "sow",
    "see",
    "saw",
    "set",
    "sit",
    "sir",
    "sat",
    "sob",
    "tap",
    "tip",
    "top",
    "tug",
    "tow",
    "toe",
    "tan",
    "ten",
    "two",
    "use",
    "van",
    "vet",
    "was",
    "wet",
    "win",
    "won",
    "wig",
    "war",
    "why",
    "who",
    "way",
    "wow",
    "you",
    "yes",
    "yak",
    "yet",
    "zip",
    "zap",
];

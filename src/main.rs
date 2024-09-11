extern crate good_web_game as ggez;

use core::f32;
use std::char;
use std::env;
use std::f32::consts::PI;
use std::path;

use getrandom::register_custom_getrandom;
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
        Color, 
        DrawMode,
        Point2,
        Text, 
        TextFragment,
        Vector2,
    },
    input::{
        self,
        keyboard::KeyCode
    },
};

use rand::{ prelude::SliceRandom, Rng, thread_rng };

fn fallback_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Ok(())
}

register_custom_getrandom!(fallback_getrandom);

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ColorPalette {
    Bg,
    Bg1,
    Bg2,
    Fg,
    Fg4,
    Blue,
    BrightYellow,
    Orange,
}

impl Into<Color> for ColorPalette {
    fn into(self) -> Color {
        match self {
            Self::Bg => Color::from_rgb(40, 40, 40),
            Self::Bg1 => Color::from_rgb(60, 56, 54),
            Self::Bg2 => Color::from_rgb(80, 73, 69),
            Self::Fg => Color::from_rgb(235, 219, 178),
            Self::Fg4 => Color::from_rgb(168, 153, 132),
            Self::Blue => Color::from_rgb(69, 133, 136),
            Self::BrightYellow => Color::from_rgb(250, 189, 47),
            Self::Orange => Color::from_rgb(214, 93, 14)
        }
    }
}

struct State {
    player: Player,
    words: Vec<Word>,
    reset_typed: usize,
}

impl event::EventHandler for State {
    fn update(&mut self, 
        ctx: &mut Context,
        gctx: &mut event::GraphicsContext,
    ) -> GameResult {

        // cant call len inside an iter_mut because borrow checker
        let len = self.words.len();

        for word in self.words.iter_mut() {
            if self.reset_typed > 0 {
                if word.state == WordState::Active {
                    word.num_typed = 0;
                }
                self.reset_typed -= 1;
            }

            let old_state = word.state;

            word.update(ctx, gctx)?;
            
            if old_state == WordState::Active && word.state == WordState::Typed {
                self.reset_typed = len;
            }
        }

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

        graphics::present(ctx, gctx)?;
        
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
    color: Option<ColorPalette>,
    state: WordState,
}

impl Word {
    fn new(word: &str, position: Point2, velocity: Vector2) -> Self {
        Self { 
            word: word.chars().collect(), 
            num_typed: 0, 
            position, 
            velocity,
            color: None,
            state: WordState::Active,
        } 
    }
}

impl EventHandler for Word {

    fn update(&mut self, ctx: &mut Context, _gctx: &mut event::GraphicsContext) -> GameResult {
        if let Some(next_ch) = self.word.get(self.num_typed) {
            let key_code = ch_to_keycode(*next_ch)
                .ok_or_else(|| GameError::CustomError(format!("unmapped character: {next_ch}")))?;

            if input::keyboard::is_key_pressed(ctx, key_code) {
                self.num_typed += 1;
            }

            self.position += self.velocity;

        } else if self.state == WordState::Active {
            self.state = WordState::Typed;
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, gctx: &mut event::GraphicsContext) -> GameResult {
        let typed_color = match self.state {
            WordState::Active => ColorPalette::BrightYellow,
            WordState::Typed => ColorPalette::Blue,
            WordState::Dead => ColorPalette::Bg,
        };
        let untyped_color = self.color.unwrap_or_else(|| ColorPalette::Fg);

        let typed = 
            TextFragment::new(self.word[0..self.num_typed].iter().collect::<String>())
            .color(typed_color);

        let mut rendered = Text::new(typed);
        rendered.add(
            TextFragment::new(self.word[self.num_typed..].iter().collect::<String>())
                .color(untyped_color)
        );

        // rendered.add(
        //     TextFragment::new(format!(" state: {:#?}", self.state)).color(ColorPalette::Fg4)
        // );

        graphics::draw(ctx, gctx, &rendered, (self.position,))?;

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
            .circle(DrawMode::fill(), self.position, self.radius, self.precision, ColorPalette::Orange.into())?
            .build(ctx, gctx)?;

        graphics::draw(ctx, gctx, &image, (self.position,))?;

        Ok(())
    }
}

fn main() -> GameResult{

    let player_radius = 4.0; 
    let player_position = Point2::new(SCREEN_WIDTH as f32 / 2.0 + player_radius, SCREEN_HEIGHT as f32 - 10.0);
    
    let mut words = vec![];
    let radius = SCREEN_HEIGHT as f32 - 20.0;
    let center_x = SCREEN_WIDTH as f32;
    let center_y = SCREEN_HEIGHT as f32 - 10.0;

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
        let y = radius * theta.sin() + center_y;

        let mut word = Word::new(label, Point2::new(x, y), Vector2::new(0.0, 0.0));
        word.color = Some(ColorPalette::Bg2);
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
            word, Point2::new(x, y), 
            Vector2::new(
                (player_position.x - x + center_x / 2.0)  / (500.0 + r / 2.0), 
                (player_position.y - y + center_y) / (500.0 + r / 2.0)
            ))
        );
    }

    let state = State {
        player: Player::new(player_position, player_radius),
        words,
        reset_typed: 0,
    };

    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let conf = ggez::conf::Conf::default()
        .window_width(SCREEN_WIDTH)
        .window_height(SCREEN_HEIGHT)
        .physical_root_dir(Some(resource_dir));

    ggez::start(
        conf,
        |mut _ctx, mut _gctx| Box::new(state),
    )

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

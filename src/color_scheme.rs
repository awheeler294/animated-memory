use ggez::graphics::Color;
use keyframe_derive::CanTween;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorPalette {
    Bg,
    Bg1,
    Bg2,
    Fg,
    Fg0,
    Fg4,
    Blue,
    BrightYellow,
    Orange,
}

impl ColorPalette {
    fn as_rgb(self) -> (u8, u8, u8) {
        match self {
            Self::Bg => (40, 40, 40),
            Self::Bg1 => (60, 56, 54),
            Self::Bg2 => (80, 73, 69),
            Self::Fg0 => (251, 241, 199),
            Self::Fg => (235, 219, 178),
            Self::Fg4 => (168, 153, 132),
            Self::Blue => (69, 133, 136),
            Self::BrightYellow => (250, 189, 47),
            Self::Orange => (214, 93, 14),
        }
    }
}

impl Into<Color> for ColorPalette {
    fn into(self) -> Color {
        let (r, g, b) = self.as_rgb();
        Color::from_rgb(r, g, b)
    }
}

impl Into<TweenableColor> for ColorPalette {
    fn into(self) -> TweenableColor {
        let (r, g, b) = self.as_rgb();
        
        let color = Color::from_rgb(r, g, b);

        TweenableColor {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

/// necessary because we can't implement CanTween for Color directly, as it's a foreign type
#[derive(CanTween, Clone, Copy)]
pub struct TweenableColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl From<TweenableColor> for Color {
    fn from(tc: TweenableColor) -> Self {
        Color::new(tc.r, tc.g, tc.b, tc.a)
    }
}

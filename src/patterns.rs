use std::collections::HashMap;

use colorgrad::Gradient;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Serialize, Clone, ToSchema, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Serialize, Clone, ToSchema, Deserialize)]
pub struct Chase {
    pub color: Color,
    pub width: usize,
}

#[derive(Debug, PartialEq, Serialize, Clone, ToSchema, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pattern {
    Spectral,

    Blues,
    Greens,
    Greys,
    Oranges,
    Purples,
    Reds,

    Turbo,
    Viridis,
    Inferno,
    Magma,
    Plasma,
    Cividis,
    Warm,
    Cool,
    CubeHelix,

    Sinebow,
    Rainbow,
}

#[derive(Debug, PartialEq, Serialize, Clone, ToSchema, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sequence {
    Solid(Color),
    Chase(Chase),
    Pattern(Pattern),
    MovingPattern(Pattern),
    CustomPattern(Vec<Color>),
    CustomMovingPattern(Vec<Color>),
}

#[derive(Debug, PartialEq, Serialize, Clone, ToSchema, Deserialize)]
pub struct TestSpec {
    pub tests: HashMap<String, Sequence>,
    pub step_ms: u64,
}

impl Pattern {
    pub fn as_gradient(&self) -> Box<dyn colorgrad::Gradient> {
        match self {
            Pattern::Spectral => Box::new(colorgrad::preset::spectral()),
            Pattern::Blues => Box::new(colorgrad::preset::blues()),
            Pattern::Greens => Box::new(colorgrad::preset::greens()),
            Pattern::Greys => Box::new(colorgrad::preset::greys()),
            Pattern::Oranges => Box::new(colorgrad::preset::oranges()),
            Pattern::Purples => Box::new(colorgrad::preset::purples()),
            Pattern::Reds => Box::new(colorgrad::preset::reds()),
            Pattern::Turbo => Box::new(colorgrad::preset::turbo()),
            Pattern::Viridis => Box::new(colorgrad::preset::viridis()),
            Pattern::Inferno => Box::new(colorgrad::preset::inferno()),
            Pattern::Magma => Box::new(colorgrad::preset::magma()),
            Pattern::Plasma => Box::new(colorgrad::preset::plasma()),
            Pattern::Cividis => Box::new(colorgrad::preset::cividis()),
            Pattern::Warm => Box::new(colorgrad::preset::warm()),
            Pattern::Cool => Box::new(colorgrad::preset::cool()),
            Pattern::CubeHelix => Box::new(colorgrad::preset::cubehelix_default()),
            Pattern::Sinebow => Box::new(colorgrad::preset::sinebow()),
            Pattern::Rainbow => Box::new(colorgrad::preset::rainbow()),
        }
    }
}

impl Sequence {
    pub fn as_vec(&self, len: usize) -> Vec<u8> {
        let mut data = Vec::<u8>::new();

        match self {
            Sequence::Solid(c) => {
                for _ in 0..len {
                    data.push(c.r);
                    data.push(c.g);
                    data.push(c.b);
                }
            }
            Sequence::Chase(c) => {
                let w = if c.width < len { c.width } else { len };

                for _ in 0..w {
                    data.push(c.color.r);
                    data.push(c.color.g);
                    data.push(c.color.b);
                }

                for _ in w..len {
                    data.push(0);
                    data.push(0);
                    data.push(0);
                }
            }
            Sequence::Pattern(p) | Sequence::MovingPattern(p) => {
                let grad = p.as_gradient();

                for c in grad.colors(len) {
                    let [r, g, b, _] = c.to_rgba8();
                    data.push(r);
                    data.push(g);
                    data.push(b);
                }
            }
            Sequence::CustomPattern(colors) | Self::CustomMovingPattern(colors) => {
                let colors = colors
                    .iter()
                    .map(|c| colorgrad::Color::from_linear_rgba8(c.r, c.g, c.b, 255))
                    .collect::<Vec<_>>();
                let grad = colorgrad::GradientBuilder::new()
                    .colors(&colors)
                    .build::<colorgrad::BasisGradient>();
                if let Ok(grad) = grad {
                    for c in grad.colors(len) {
                        let [r, g, b, _] = c.to_rgba8();
                        data.push(r);
                        data.push(g);
                        data.push(b);
                    }
                } else {
                    tracing::error!("Couldn't build gradient");
                }
            }
        }

        data
    }

    pub fn moves(&self) -> bool {
        match self {
            Sequence::Solid(_) | Sequence::Pattern(_) | Sequence::CustomPattern(_) => false,
            Sequence::Chase(_) | Sequence::MovingPattern(_) | Sequence::CustomMovingPattern(_) => {
                true
            }
        }
    }
}

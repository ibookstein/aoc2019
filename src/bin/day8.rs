use aoc2019::aoc_input::get_input;
use std::convert::{TryFrom, TryInto};
use std::fmt;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    Black,
    White,
    Transparent,
}

impl TryFrom<char> for Color {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Color::Black),
            '1' => Ok(Color::White),
            '2' => Ok(Color::Transparent),
            _ => Err("Invalid character encountered"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            Color::Black => ' ',
            Color::White => '*',
            Color::Transparent => '?',
        };
        write!(f, "{}", c)
    }
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

struct Layer {
    pixels: [Color; WIDTH * HEIGHT],
}

impl Layer {
    fn transparent() -> Layer {
        Layer {
            pixels: [Color::Transparent; WIDTH * HEIGHT],
        }
    }

    fn count_of(&self, color: Color) -> usize {
        self.pixels.iter().filter(|&d| *d == color).count()
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..HEIGHT {
            for p in &self.pixels[WIDTH * i..WIDTH * (i + 1)] {
                write!(f, "{}", p)?;
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_layers(input: &str) -> Vec<Layer> {
    let colors: Vec<Color> = input
        .trim()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect();

    let chunks = colors.chunks_exact(WIDTH * HEIGHT);
    assert!(chunks.remainder().is_empty());

    let mut layers = Vec::<Layer>::new();
    for chunk in chunks {
        let mut layer = Layer::transparent();
        layer.pixels.copy_from_slice(chunk);
        layers.push(layer);
    }

    layers
}

fn overlay_layers(layers: &[Layer]) -> Layer {
    let mut overlay = Layer::transparent();

    for layer in layers.iter() {
        for i in 0..layer.pixels.len() {
            if overlay.pixels[i] == Color::Transparent {
                overlay.pixels[i] = layer.pixels[i];
            }
        }
    }

    overlay
}

fn main() {
    let input = get_input(8);
    let layers = parse_layers(&input);

    let min_zeros_layer = layers
        .iter()
        .min_by_key(|layer| layer.count_of('0'.try_into().unwrap()))
        .unwrap();
    let ones_count = min_zeros_layer.count_of('1'.try_into().unwrap());
    let twos_count = min_zeros_layer.count_of('2'.try_into().unwrap());
    println!("Ones Count * Twos Count = {}", ones_count * twos_count);

    println!("Overlay of layers:");
    println!("{}", overlay_layers(&layers[..]));
}

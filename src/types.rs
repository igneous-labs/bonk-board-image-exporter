use std::ops::Add;

use anyhow::{anyhow, Result};
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl Add<Coord> for Coord {
    type Output = Self;
    fn add(self, rhs: Coord) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl TryFrom<String> for Coord {
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<Self> {
        let vs = v
            .split(',')
            .map(|v| v.parse::<u16>())
            .collect::<Result<Vec<_>, _>>()?;
        if vs.len() != 2 {
            Err(anyhow!("Wrong coordinate format: {}", v))
        } else {
            Ok(Self { x: vs[0], y: vs[1] })
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TryFrom<&[u8]> for Color {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 3 {
            Err(anyhow!("Wrong color format: {:?}", bytes))
        } else {
            Ok(Self {
                r: bytes[0],
                g: bytes[1],
                b: bytes[2],
            })
        }
    }
}

impl TryFrom<String> for Color {
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<Self> {
        let vs = v
            .split(',')
            .map(|v| v.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()?;
        if vs.len() != 3 {
            Err(anyhow!("Wrong color format: {}", v))
        } else {
            Ok(Self {
                r: vs[0],
                g: vs[1],
                b: vs[2],
            })
        }
    }
}

#[derive(Serialize)]
pub struct Pixel {
    pub coord: Coord,
    pub color: Color,
}

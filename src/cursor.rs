use square::Square;

#[derive(Debug, Clone, Copy)]
pub enum Cursor {
    Plain((f32, f32), [f32; 3]),
    Growing((f32, f32), f32, [f32; 3]),
}

impl Cursor {
    pub fn to_square(self) -> Square {
        match self {
            Cursor::Plain(xy, color) => Square {
                pos: xy,
                size: 0.05,
                color,
            },
            Cursor::Growing(xy, size, color) => Square {
                pos: xy,
                size,
                color,
            },
        }
    }
}

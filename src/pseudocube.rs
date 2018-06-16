use cursor::Cursor;
use gfx_props::Vertex;
use rand;
use square::Square;

#[derive(Debug)]
pub struct Pseudocube {
    squares: Vec<Square>,
    ratio: f32,
    cursor: Cursor,
}

impl Pseudocube {
    pub fn new() -> Self {
        Pseudocube {
            cursor: Cursor::Plain((0.0, 0.0), rand::random()),
            squares: vec![],
            ratio: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn add_square(&mut self, x: f32, y: f32, size: f32, color: [f32; 3]) {
        self.squares.push(Square {
            pos: (x, y),
            size,
            color,
        })
    }

    pub fn get_vertices_indices(&self) -> (Vec<Vertex>, Vec<u16>) {
        let (mut vs, mut is) = (vec![], vec![]);
        let cursor = self.cursor.to_square();

        for (i, sq) in self.squares.iter().chain(Some(&cursor)).enumerate() {
            let (pos, half) = (sq.pos, 0.5 * sq.size);
            let i = i as u16;

            let (hx, hy) = if self.ratio > 1.0 {
                (half / self.ratio, half)
            } else {
                (half, half * self.ratio)
            };

            vs.extend(&[
                Vertex {
                    pos: [pos.0 + hx, pos.1 - hy],
                    uv: [1.0, 1.0],
                    color: sq.color,
                },
                Vertex {
                    pos: [pos.0 - hx, pos.1 - hy],
                    uv: [0.0, 1.0],
                    color: sq.color,
                },
                Vertex {
                    pos: [pos.0 - hx, pos.1 + hy],
                    uv: [0.0, 0.0],
                    color: sq.color,
                },
                Vertex {
                    pos: [pos.0 + hx, pos.1 + hy],
                    uv: [1.0, 0.0],
                    color: sq.color,
                },
            ]);
            is.extend(&[4 * i, 4 * i + 1, 4 * i + 2, 4 * i + 2, 4 * i + 3, 4 * i]);
        }

        (vs, is)
    }

    pub fn update_ratio(&mut self, ratio: f32) {
        self.ratio = ratio;
    }

    pub fn update_cursor_position(&mut self, x: f32, y: f32) {
        let x = 2.0 * x - 1.0;
        let y = -2.0 * y + 1.0;

        self.cursor = match self.cursor {
            Cursor::Plain(_, color) => Cursor::Plain((x, y), color),
            Cursor::Growing(_, size, color) => Cursor::Growing((x, y), size, color),
        };
    }

    pub fn start_growing(&mut self) {
        if let Cursor::Plain(xy, color) = self.cursor {
            self.cursor = Cursor::Growing(xy, 0.05, color);
        }
    }

    pub fn stop_growing(&mut self) {
        if let Cursor::Growing(xy, size, color) = self.cursor {
            self.squares
                .push(Cursor::Growing(xy, size, color).to_square());
            self.cursor = Cursor::Plain(xy, rand::random());
        }
    }

    pub fn tick(&mut self) {
        if let Cursor::Growing(xy, size, color) = self.cursor {
            self.cursor = Cursor::Growing(xy, size + 0.01, color);
        }
    }
}

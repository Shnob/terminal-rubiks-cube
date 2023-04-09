use colored::Colorize;

pub struct Cube {
    pieces: Vec<Piece>,
}

struct Piece {
    verts: Vec<(f32, f32, f32)>,
    rgb: (u8, u8, u8),
}

impl Cube {
    fn show(&self) {
        for piece in self.pieces.iter() {
        }
    }
}

impl Default for Cube {
    fn default() -> Cube {
        Cube {
            pieces: vec![
                Piece {
                    verts: vec![(-10.0, -10.0, 0.0), (-10.0, 10.0, 0.0), (10.0, 10.0, 0.0), (10.0, -10.0, 0.0)],
                    rgb: (255, 0, 255),
                }
            ],
        }
    }
}

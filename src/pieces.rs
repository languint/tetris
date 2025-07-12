#[derive(Clone)]
pub struct PieceRotation {
    pub(crate) trans_row: i8,
    pub(crate) trans_col: i8,
    pub(crate) rows: Vec<Vec<i8>>,
}

#[derive(Clone)]
pub enum PieceType {
    Straight,
    LLeft,
    LRight,
    Square,
    S,
    Z,
    T,
}

#[derive(Clone)]
pub struct Piece {
    pub(crate) rotations: Vec<PieceRotation>,
}

pub struct PieceState {
    pub(crate) piece: Piece,
    pub(crate) piece_type: PieceType,
    pub(crate) row: i8,
    pub(crate) col: i8,
    pub(crate) rotation: u8,
}

impl Clone for PieceState {
    fn clone(&self) -> Self {
        PieceState {
            piece: Piece::get_piece_data(&self.piece_type),
            piece_type: self.piece_type.clone(),
            row: self.row,
            col: self.col,
            rotation: self.rotation,
        }
    }
}

impl Piece {
    pub fn new(rotations: Vec<PieceRotation>) -> Piece {
        Piece { rotations }
    }

    pub fn get_piece_data(piece_type: &PieceType) -> Piece {
        Piece::new(match piece_type {
            PieceType::Straight => vec![
                PieceRotation {
                    trans_row: 1,
                    trans_col: 0,
                    rows: vec![vec![1, 1, 1, 1]],
                },
                PieceRotation {
                    trans_row: -1,
                    trans_col: 1,
                    rows: vec![vec![1], vec![1], vec![1], vec![1]],
                },
            ],
            PieceType::LLeft => vec![
                PieceRotation {
                    trans_row: 1,
                    trans_col: -1,
                    rows: vec![vec![1, 1, 1], vec![0, 0, 1]],
                },
                PieceRotation {
                    trans_row: -1,
                    trans_col: 0,
                    rows: vec![vec![0, 1], vec![0, 1], vec![1, 1]],
                },
                PieceRotation {
                    trans_row: 0,
                    trans_col: 0,
                    rows: vec![vec![1, 0, 0], vec![1, 1, 1]],
                },
                PieceRotation {
                    trans_row: 0,
                    trans_col: 1,
                    rows: vec![vec![1, 1], vec![1, 0], vec![1, 0]],
                },
            ],
            PieceType::LRight => vec![
                PieceRotation {
                    trans_row: 1,
                    trans_col: -1,
                    rows: vec![vec![1, 1, 1], vec![1, 0, 0]],
                },
                PieceRotation {
                    trans_row: -1,
                    trans_col: 0,
                    rows: vec![vec![1, 1], vec![0, 1], vec![0, 1]],
                },
                PieceRotation {
                    trans_row: 0,
                    trans_col: 0,
                    rows: vec![vec![0, 0, 1], vec![1, 1, 1]],
                },
                PieceRotation {
                    trans_row: 0,
                    trans_col: 1,
                    rows: vec![vec![1, 0], vec![1, 0], vec![1, 1]],
                },
            ],
            PieceType::Square => vec![PieceRotation {
                trans_row: 0,
                trans_col: 0,
                rows: vec![vec![1, 1], vec![1, 1]],
            }],
            PieceType::S => vec![
                PieceRotation {
                    trans_row: 1,
                    trans_col: 0,
                    rows: vec![vec![0, 1, 1], vec![1, 1, 0]],
                },
                PieceRotation {
                    trans_row: -1,
                    trans_col: 0,
                    rows: vec![vec![1, 0], vec![1, 1], vec![0, 1]],
                },
            ],
            PieceType::Z => vec![
                PieceRotation {
                    trans_row: 1,
                    trans_col: 0,
                    rows: vec![vec![1, 1, 0], vec![0, 1, 1]],
                },
                PieceRotation {
                    trans_row: -1,
                    trans_col: 0,
                    rows: vec![vec![0, 1], vec![1, 1], vec![1, 0]],
                },
            ],
            PieceType::T => vec![
                PieceRotation {
                    trans_row: 1,
                    trans_col: -1,
                    rows: vec![vec![1, 1, 1], vec![0, 1, 0]],
                },
                PieceRotation {
                    trans_row: -1,
                    trans_col: 0,
                    rows: vec![vec![0, 1], vec![1, 1], vec![0, 1]],
                },
                PieceRotation {
                    trans_row: 0,
                    trans_col: 0,
                    rows: vec![vec![0, 1, 0], vec![1, 1, 1]],
                },
                PieceRotation {
                    trans_row: 0,
                    trans_col: 1,
                    rows: vec![vec![1, 0], vec![1, 1], vec![1, 0]],
                },
            ],
        })
    }
}

impl PieceState {
    pub fn new(piece_type: PieceType, column: i8) -> PieceState {
        PieceState {
            piece: Piece::get_piece_data(&piece_type),
            piece_type,
            row: 0,
            col: column,
            rotation: 0,
        }
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = (i8, i8)> + '_ {
        let rotation = &self.piece.rotations[self.rotation as usize];
        rotation.rows.iter().enumerate().flat_map(move |(r, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &val)| val != 0)
                .map(move |(c, _)| {
                    (
                        self.row + r as i8 + rotation.trans_row,
                        self.col + c as i8 + rotation.trans_col,
                    )
                })
        })
    }

    pub fn color(&self) -> &str {
        match self.piece_type {
            PieceType::Straight => "light-blue",
            PieceType::LLeft => "blue",
            PieceType::LRight => "orange",
            PieceType::Square => "yellow",
            PieceType::Z => "red",
            PieceType::S => "green",
            PieceType::T => "purple",
        }
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % self.piece.rotations.len() as u8;
    }
}

pub struct Position {
    pub x: int,
    pub y: int
}

impl Position {
    pub fn new(x: int, y: int) -> Position {
        Position { x: x, y: y }
    }
}

pub struct Bound {
    pub min: Position,
    pub max: Position
}

impl Bound {
    pub fn new(min: Position, max: Position) -> Bound {
        Bound { min: min, max: max }
    }
}

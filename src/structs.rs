use std::fmt;

#[derive(Debug)]
pub struct Node {
    g: u32,
    h: u32,
    f: i32,
}

#[derive(Debug)]
pub enum CreationError {
    Negative,
    WrongCharacter,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match *self {
            CreationError::Negative => "number is negative",
            CreationError::WrongCharacter => "wrong character",
        };
        f.write_str(desc)
    }
}

impl Node {
    pub fn new(tuple: (i32, i32, i32)) -> Result<Self, CreationError> {
        if tuple.0 < 0 || tuple.1 < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(Self {
                g: tuple.0 as u32,
                h: tuple.1 as u32,
                f: tuple.2,
            })
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = format!("g: {}, h: {}, f: {}", self.g, self.h, self.f);
        f.write_str(&desc)
    }
}

#[derive(Debug)]
pub struct Tile {
    path: bool,
    node: Node,
}

impl Tile {
    pub fn new(path: bool, node: Node) -> Self {
        Self { path, node }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = format!("({}, {})", self.path, self.node);
        f.write_str(&desc)
    }
}

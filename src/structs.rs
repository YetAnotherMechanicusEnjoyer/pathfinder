use std::fmt;

#[derive(Debug, Clone, Copy)]
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
    pub fn get_info(&self) -> (u32, u32, i32) {
        (self.g, self.h, self.f)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = format!("({}, {}, {})", self.g, self.h, self.f);
        f.write_str(&desc)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    x: usize,
    y: usize,
    path: bool,
    node: Node,
}

impl Tile {
    pub fn new(x: i32, y: i32, path: bool, node: Node) -> Result<Self, CreationError> {
        if x < 0 || y < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(Self {
                x: x as usize,
                y: y as usize,
                path,
                node,
            })
        }
    }
    pub fn get_coord(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    pub fn get_path(&self) -> bool {
        self.path
    }
    pub fn get_node(&self) -> Node {
        self.node
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = format!("({}, {}, {}, {})", self.x, self.y, self.path, self.node);
        f.write_str(&desc)
    }
}

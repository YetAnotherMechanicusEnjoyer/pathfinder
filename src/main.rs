mod structs;
use std::fs::{self};
use std::io::{self, Write};
use structs::{CreationError, Node, Tile};

fn read_map(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("{e}")),
    }
}

fn read_input(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}

fn line_to_vec(
    line: &str,
    y: i32,
    start: (i32, i32),
    end: (i32, i32),
) -> Result<Vec<Tile>, CreationError> {
    let mut ch = line.chars();
    let mut v: Vec<Tile> = Vec::new();
    let mut x: i32 = 0;

    loop {
        let g = (x - start.0).abs() + (y - start.1).abs();
        let h = (x - end.0).abs() + (y - end.1).abs();
        let f = g + h;
        let n = Node::new((g, h, f))?;
        match ch.next() {
            Some('#') => {
                v.push(Tile::new(false, n));
            }
            Some('-') => {
                v.push(Tile::new(true, n));
            }
            None => {
                return Ok(v);
            }
            _ => return Err(CreationError::WrongCharacter),
        }
        x += 1;
    }
}

fn init_map(
    content: &str,
    start: (i32, i32),
    end: (i32, i32),
) -> Result<Vec<Vec<Tile>>, CreationError> {
    let mut map: Vec<Vec<Tile>> = Vec::new();

    let v: Vec<&str> = content.split("\n").collect();
    for (y, line) in v.iter().enumerate() {
        map.push(line_to_vec(line, y as i32, start, end)?);
    }
    Ok(map)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = read_input("Enter map file path: ");
    let start: (i32, i32) = (
        read_input("Enter coordinates of the starting point:\n\tx: ").parse()?,
        read_input("\ty: ").parse()?,
    );
    let end: (i32, i32) = (
        read_input("Enter coordinates of the ending point:\n\tx: ").parse()?,
        read_input("\ty: ").parse()?,
    );
    match read_map(&path) {
        Ok(map) => match init_map(&map, start, end) {
            Ok(m) => {
                let map = m;
                map.iter().for_each(|v| {
                    v.iter().for_each(|tile| println!("{}\n", tile));
                    println!();
                });
            }
            Err(e) => println!("Error {e}"),
        },
        Err(err) => println!("Error: {err}"),
    }
    Ok(())
}

mod structs;
use std::collections::HashSet;
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
                v.push(Tile::new(x, y, false, n)?);
            }
            Some('-') => {
                v.push(Tile::new(x, y, true, n)?);
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

fn get_tile(map: &[Vec<Tile>], coord: (i32, i32)) -> Option<&Tile> {
    let (x, y) = (coord.0 as usize, coord.1 as usize);

    map.get(y)?.get(x)
}

fn next_tile<'a>(
    map: &'a [Vec<Tile>],
    known_map: &mut Vec<&'a Tile>,
    visited: &mut HashSet<(usize, usize)>,
    tile: &'a Tile,
) -> Option<&'a Tile> {
    let (x, y) = tile.get_coord();
    let mut neighbors = Vec::new();

    for (ax, ay) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let (nx, ny) = (x as i32 + ax, y as i32 + ay);

        if visited.contains(&(nx as usize, ny as usize)) {
            continue;
        }
        if let Some(neighbor) = get_tile(map, (nx, ny)) {
            if neighbor.get_path() {
                neighbors.push(neighbor);
                known_map.push(neighbor);
            }
        }
    }
    neighbors.into_iter().min_by_key(|tile| {
        let f = tile.get_node().get_info().2;
        let h = tile.get_node().get_info().1;
        (f, h)
    })
}

fn is_neighbor(tile1: (usize, usize), tile2: (usize, usize)) -> bool {
    (tile1.0 == tile2.0 && (tile1.1 as i32 - tile2.1 as i32).abs() == 1)
        || (tile1.1 == tile2.1 && (tile1.0 as i32 - tile2.0 as i32).abs() == 1)
}

fn search_path(
    map: &[Vec<Tile>],
    start: (i32, i32),
    end: (i32, i32),
) -> Result<Vec<(usize, usize)>, ()> {
    let start = get_tile(map, start).expect("Error: Invalid starting coordinates.");
    let end = get_tile(map, end).expect("Error: Invalid ending coordinates.");
    let mut next = start;
    let mut known_tiles: Vec<&Tile> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut path: Vec<(usize, usize)> = vec![start.get_coord()];

    visited.insert(next.get_coord());
    while next.get_coord() != end.get_coord() {
        next = match next_tile(map, &mut known_tiles, &mut visited, next) {
            Some(tile) => {
                while path.last().is_some() && !is_neighbor(*path.last().unwrap(), tile.get_coord())
                {
                    path.pop();
                }
                visited.insert(tile.get_coord());
                path.push(tile.get_coord());
                tile
            }
            None => {
                match known_tiles
                    .clone()
                    .into_iter()
                    .filter(|tile| !visited.contains(&tile.get_coord()))
                    .min_by_key(|tile| {
                        let f = tile.get_node().get_info().2;
                        let h = tile.get_node().get_info().1;
                        (h, f)
                    }) {
                    Some(t) => {
                        while path.last().is_some()
                            && !is_neighbor(*path.last().unwrap(), t.get_coord())
                        {
                            path.pop();
                        }
                        visited.insert(t.get_coord());
                        path.push(t.get_coord());
                        t
                    }
                    None => {
                        println!("No valid path found.");
                        return Err(());
                    }
                }
            }
        };
    }
    Ok(path)
}

fn print_map(map: &str) {
    let mut v: Vec<&str> = Vec::new();

    for c in map.chars() {
        match c {
            '#' => v.push("\x1b[30;47m   "),
            '-' => v.push("\x1b[30;44m   "),
            _ => v.push("\x1b[0m\n"),
        }
    }
    v.remove(v.len() - 1);
    let line: Vec<&str> = v
        .split_inclusive(|&s| s == "\x1b[0m\n")
        .next()
        .unwrap_or(&[])
        .to_vec();
    print!("   ");
    for x in 0..line.len() - 1 {
        if x < 10 {
            print!(" 0{x}");
        } else if x < 100 {
            print!(" {x}");
        } else {
            print!("{x}");
        }
    }
    println!();
    let mut y: usize = 0;
    print!("00 ");
    for s in v {
        print!("{s}");
        if s == "\x1b[0m\n" {
            y += 1;
            if y < 10 {
                print!("0{y} ");
            } else if y < 100 {
                print!("{y} ");
            } else {
                print!("{y}");
            }
        }
    }
    println!("\x1b[0m");
}

fn print_path(
    map: &[Vec<Tile>],
    path: Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
) {
    print!("   ");
    for x in 0..map[0].len() {
        if x < 10 {
            print!(" 0{x}");
        } else if x < 100 {
            print!(" {x}");
        } else {
            print!("{x}");
        }
    }
    println!();
    for y in 0..map.len() - 1 {
        if y < 10 {
            print!("0{y} ");
        } else if y < 100 {
            print!("{y} ");
        } else {
            print!("{y}");
        }
        for x in 0..map.get(y).unwrap().len() {
            if let Some(tile) = get_tile(map, (x as i32, y as i32)) {
                if start == tile.get_coord() {
                    print!("\x1b[30;43m   ");
                } else if end == tile.get_coord() {
                    print!("\x1b[30;41m   ");
                } else if path.contains(&tile.get_coord()) {
                    print!("\x1b[30;42m   ");
                } else if tile.get_path() {
                    print!("\x1b[30;44m   ");
                } else {
                    print!("\x1b[30;47m   ");
                }
            } else {
                println!("\nError: print_path");
                return;
            }
        }
        println!("\x1b[0m");
    }
}

pub fn pathfinder() -> Result<(), Box<dyn std::error::Error>> {
    let path = read_input("Enter map file path: ");

    match read_map(&path) {
        Ok(map) => {
            print_map(&map);
            let start: (i32, i32) = (
                read_input("Enter coordinates of the starting point:\n\tx: ").parse()?,
                read_input("\ty: ").parse()?,
            );
            let end: (i32, i32) = (
                read_input("Enter coordinates of the ending point:\n\tx: ").parse()?,
                read_input("\ty: ").parse()?,
            );

            match init_map(&map, start, end) {
                Ok(m) => {
                    let tile_map = m;
                    match search_path(&tile_map, start, end) {
                        Ok(path) => print_path(
                            &tile_map,
                            path,
                            (start.0 as usize, start.1 as usize),
                            (end.0 as usize, end.1 as usize),
                        ),
                        Err(()) => return Ok(()),
                    }
                }
                Err(e) => panic!("Error {e}"),
            }
        }
        Err(err) => panic!("Error: {err}"),
    }
    Ok(())
}

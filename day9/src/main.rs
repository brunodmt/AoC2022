use std::fs;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let re = Regex::new(r"^(U|D|R|L) (\d+)$").unwrap();

    let mut visited = HashSet::new();

    let mut head_pos = Position {
        x: 0,
        y: 0,
    };

    let mut tail_pos = Position {
        x: 0,
        y: 0,
    };

    visited.insert(tail_pos);

    for line in contents.lines() {
        if let Some(caps) = re.captures(line) {
            let inst = caps.get(1).unwrap().as_str();
            let size: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let (dx, dy) = match inst {
                "U" => (0, -1),
                "D" => (0, 1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => (0, 0),
            };
            //println!("{} {}", inst, size);
            for i in 0..size {
                
                head_pos.y = head_pos.y + dy;
                head_pos.x = head_pos.x + dx;
                
                tail_pos = calc_tail_position(tail_pos, head_pos);
                visited.insert(tail_pos);
                //println!("  step: {} h:({},{}) t:({},{})", i, head_pos.x, head_pos.y, tail_pos.x, tail_pos.y);
            }
        }
    }

    println!("Result 1 is {}", visited.len());
}

fn problem2() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let re = Regex::new(r"^(U|D|R|L) (\d+)$").unwrap();

    let mut visited = HashSet::new();

    const RSIZE: usize = 10;
    let mut knots = [Position {
        x: 0,
         y: 0,
    };RSIZE];

    visited.insert(knots[RSIZE-1]);

    for line in contents.lines() {
        if let Some(caps) = re.captures(line) {
            let inst = caps.get(1).unwrap().as_str();
            let size: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let (dx, dy) = match inst {
                "U" => (0, -1),
                "D" => (0, 1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => (0, 0),
            };
            for _ in 0..size {
                
                knots[0].y = knots[0].y + dy;
                knots[0].x = knots[0].x + dx;
                
                for i in 1..RSIZE {
                    knots[i] = calc_tail_position(knots[i], knots[i-1]);
                }

                visited.insert(knots[RSIZE-1]);
                //println!("  step: {} h:({},{}) t:({},{})", i, head_pos.x, head_pos.y, tail_pos.x, tail_pos.y);
            }
        }
    }

    println!("Result 1 is {}", visited.len());
}

fn calc_tail_position (current: Position, head: Position) -> Position {
    let dx = head.x - current.x;
    let dy = head.y - current.y;
   
    let result = if dx.abs() < 2 && dy.abs() < 2 {
        current
    } else if dx.abs() == 0 && dy.abs() == 2 {
        Position {
            x: current.x,
            y: current.y + dy/2,
        }
    } else if dx.abs() == 2 && dy.abs() == 0 {
        Position {
            x: current.x + dx/2,
            y: current.y,
        }
    } else if dx.abs() <= 2 && dy.abs() <= 2 {
        Position {
            x: current.x + if dx > 0 { 1 } else { -1 },
            y: current.y + if dy > 0 { 1 } else { -1 },
        }
    } else {
        panic!("Unexpected state");
    };
    //println!("hx:{} hy:{} cx:{} cy:{} dx:{} dy:{} resulting in x:{} y:{}", head.x, head.y, current.x, current.y, dx, dy, result.x, result.y);
    return result;
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

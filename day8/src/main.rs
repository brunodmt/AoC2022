use std::fs;

use std::cell::RefCell;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    
    let grid = load_input(&contents);

    let rows = grid.len();
    let cols = grid[0].len();

    for row in &grid {
        let mut max = i32::MIN;
        for tree in row {
            if tree.height > max {
                max = tree.height;
                let mut visible = tree.visible.borrow_mut();
                *visible = true;
            }
        }
    }

    for row in &grid {
        let mut max = i32::MIN;
        for tree in row.iter().rev() {
            if tree.height > max {
                max = tree.height;
                let mut visible = tree.visible.borrow_mut();
                *visible = true;
            }
        }
    }

    for j in 0..cols {
        let mut max = i32::MIN;
        for i in 0..rows {
            let tree = &grid[i][j];
            if tree.height > max {
                max = tree.height;
                let mut visible = tree.visible.borrow_mut();
                *visible = true;
            }
        }
    }

    for j in 0..cols {
        let mut max = i32::MIN;
        for i in (0..rows).rev() {
            let tree = &grid[i][j];
            if tree.height > max {
                max = tree.height;
                let mut visible = tree.visible.borrow_mut();
                *visible = true;
            }
        }
    }

    let mut visible_count = 0;

    for row in &grid {
        for tree in row {
            if *tree.visible.borrow() {
                visible_count = visible_count + 1;
            }
        }
    }

    println!("Result 1 is {}", visible_count);


    let mut max_score = 0;
    for i in 0..rows {
        for j in 0..cols {
            let tree = &grid[i][j];
            let height = tree.height;
            let mut score = 1;

            if i > 0 {
                for k in (0..i).rev() {
                    if grid[k][j].height >= height || k == 0 {
                        score = score * (i-k);
                        break;
                    }
                }
            }

            for k in (i+1)..rows {
                if grid[k][j].height >= height || k == (rows-1) {
                    score = score * (k-i);
                    break;
                }
            }

            if j > 0 {
                for l in (0..j).rev() {
                    if grid[i][l].height >= height || l == 0 {
                        score = score * (j-l);
                        break;
                    }
                }
            }

            for l in (j+1)..cols {
                if grid[i][l].height >= height || l == (cols-1) {
                    score = score * (l-j);
                    break;
                }
            }

            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Result 2 is {}", max_score);
}

fn load_input(input: &str) -> Vec<Vec<Tree>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let tree = Tree {
                height: i32::try_from(c.to_digit(10).unwrap()).unwrap(),
                visible: RefCell::new(false),
            };
            row.push(tree);
        }
        grid.push(row);
    }
    return grid;
}

#[derive(Debug)]
struct Tree {
    height: i32,
    visible: RefCell<bool>,
}
use std::fs;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let cd_re = Regex::new(r"^\$ cd (.+)$").unwrap();
    let ls_re = Regex::new(r"^\$ ls$").unwrap();
    let dir_re = Regex::new(r"^dir (\w+)$").unwrap();
    let file_re = Regex::new(r"^(\d+) ([\w|\.]+)$").unwrap();

    let root = Rc::new(Dir {
        parent: None,
        name: "/",
        size: RefCell::new(None),
        dirs: Rc::new(RefCell::new(Vec::new())),
        files: Rc::new(RefCell::new(Vec::new())),
    });

    let mut current = Rc::clone(&root);

    for line in contents.lines() {
        if ls_re.is_match(line) {
            // do nothing
        } else if let Some(caps) = cd_re.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            current = match name {
                "/" => Rc::clone(&root),
                ".." => Rc::clone(current.parent.as_ref().expect("Dir has no parent!")),
                _ => {
                    let dirs = current.dirs.borrow();
                    let mut dir_it = dirs.iter();
                    loop {
                        if let Some(dir) = dir_it.next() {
                            if dir.name == name {
                                break Rc::clone(dir);
                            }
                        } else {
                            panic!("Child {} not found in {}!", name, current.name);
                        }
                    }
                }
            };
        } else if let Some(caps) = dir_re.captures(line) {
            let dir = Rc::new(Dir {
                parent: Some(Rc::clone(&current)),
                name: caps.get(1).unwrap().as_str(),
                size: RefCell::new(None),
                dirs: Rc::new(RefCell::new(Vec::new())),
                files: Rc::new(RefCell::new(Vec::new())),
            });
            current.dirs.borrow_mut().push(dir);
        } else if let Some(caps) = file_re.captures(line) {
            let file = Rc::new(File {
                parent: Rc::clone(&current),
                name: caps.get(2).unwrap().as_str(),  
                size: caps.get(1).unwrap().as_str().parse().unwrap(),
            });
            current.files.borrow_mut().push(file);
        } else {
            panic!("Line {} is not recognized", line);
        }
    }

    let (_, result_sum) = process_dir(&root);
    println!("Result 1 is {}", result_sum);

    let total = 70000000;
    let needed = 30000000;
    let used = root.size.borrow().unwrap();
    let free = total - used;
    let missing = needed - free;

    let result_2 = find_smallest(&root, missing);
    println!("Result 2 is {}", result_2);
}

fn process_dir(dir: &Rc<Dir>) -> (u32, u32) {
    let mut this_sum = 0;
    let mut result_sum = 0;
    for file in dir.files.borrow().iter() {
        this_sum = this_sum + file.size;
    }
    for dir in dir.dirs.borrow().iter() {
        let (a, b) = process_dir(dir);
        this_sum = this_sum + a;
        result_sum = result_sum + b;
    }
    if this_sum <= 100000 {
        result_sum = result_sum + this_sum;
    }
    let mut size = dir.size.borrow_mut();
    *size = Some(this_sum);
    return (this_sum, result_sum);
}

fn find_smallest(dir: &Rc<Dir>, needed: u32) -> u32 {
    let mut smallest = dir.size.borrow().unwrap();
    for dir in dir.dirs.borrow().iter() {
        let new = find_smallest(dir, needed);
        if new >= needed && new < smallest {
            smallest = new;
        }
    }
    return smallest;
}

struct File<'a> {
    parent: Rc<Dir<'a>>,
    name: &'a str,
    size: u32,
}

struct Dir<'a> {
    parent: Option<Rc<Dir<'a>>>,
    name: &'a str,
    size: RefCell<Option<u32>>,
    dirs: Rc<RefCell<Vec<Rc<Dir<'a>>>>>,
    files: Rc<RefCell<Vec<Rc<File<'a>>>>>,
}

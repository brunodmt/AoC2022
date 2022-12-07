use regex::Regex;

pub fn find_tops(input: &str) {
    let mut problem = parse_input(input);
    for step in problem.procedure {
        for _ in 0..step.crates {
            if let Some(c) = problem.stacks[step.from-1].pop() {
                problem.stacks[step.to-1].push(c);
            }
        }
    }

    for mut stack in problem.stacks {
        print!("{}", stack.pop().unwrap());
    }
    println!();
}

pub fn find_tops_2(input: &str) {
    let mut problem = parse_input(input);
    for step in problem.procedure {
        let split_at = problem.stacks[step.from-1].len() - step.crates;
        let mut elements = problem.stacks[step.from-1].split_off(split_at);
        problem.stacks[step.to-1].append(&mut elements);
    }

    for mut stack in problem.stacks {
        print!("{}", stack.pop().unwrap());
    }
    println!();
}

fn parse_input(input: &str) -> Problem {
    let steps_re = Regex::new(r"^move (?P<crates>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();

    let mut setup_stack = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if !line.is_empty() {
            setup_stack.push(line);
        } else {
            break;
        }
    }

    let stacks_line = setup_stack.pop().unwrap();
    let stacks = stacks_line.trim().chars().last().unwrap().to_string().parse::<usize>().unwrap();

    let mut problem = Problem {
        stacks: vec![Vec::new(); stacks],
        procedure: Vec::new(),
    };

    setup_stack.reverse();

    for line in setup_stack {
        let mut char_it = line.chars();
        for stack in problem.stacks.iter_mut() {
            char_it.next();
            if let Some(c) = char_it.next() {
                if c != ' ' {
                    stack.push(c);
                }
            }
            char_it.next();
            char_it.next();

        }
    }

    while let Some(line) = lines.next() {
        if steps_re.is_match(line) {
            for cap in steps_re.captures_iter(line) {
                let step = Step {
                    crates: cap[1].trim().parse::<usize>().unwrap(),
                    from: cap[2].trim().parse::<usize>().unwrap(),
                    to: cap[3].trim().parse::<usize>().unwrap(),
                };
                problem.procedure.push(step);
            }
        } else {
            panic!("Unexpected line!");
        }
    }

    return problem;
}

#[derive(Debug)]
struct Step {
    crates: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Problem {
    stacks: Vec<Vec<char>>,
    procedure: Vec<Step>,
}
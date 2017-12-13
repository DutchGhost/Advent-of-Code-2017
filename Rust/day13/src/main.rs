/* TODO:
        Make part 1 and part 2 share the same code for collisions
*/

const PUZZLE: &'static str = include_str!("Input.txt");

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Scanner {
    range: i64,
    current: i64,
    stepper: i64,
}

impl Scanner {
    fn new(range: i64) -> Scanner {
        Scanner {
            range: range,
            current: 0,
            stepper: 1,
        }
    }

    fn update(&mut self) {
        self.current += self.stepper;
        if self.current == 0 || self.current == self.range {
            self.stepper *= -1;
        }
    }
}


fn parse(input: &str) -> Vec<Option<Scanner>> {
    let mut vector: Vec<Option<Scanner>> = vec![None; 99];
    for line in input.lines() {
        let splitted = line.split(": ").map(|item| item.parse::<i64>().expect("Failed to parse")).collect::<Vec<_>>();
        
        if let Some(scanner) = vector.get_mut(splitted[0] as usize) {
            *scanner = Some(Scanner::new(splitted[1] - 1));
        }
    }
    vector
}

//for every Scanner in the firewall, update the scanner.
fn update(firewall: &mut [Option<Scanner>]) {
    for scanner in firewall.iter_mut() {
        if let &mut Some(ref mut s) = scanner {
            s.update();
        }
    }
}

//loop over the depths, get the scanner of the current depth,
//if the scanner's current position is 0...we're caught!
fn severity(firewall: &mut [Option<Scanner>]) -> i64 {
    let mut severity = 0;

    for i in 0..firewall.len() {
        if let Some(opt_scanner) = firewall.get(i) {
            if let &Some(ref scanner) = opt_scanner {
                if scanner.current == 0 {
                    severity += i as i64 * (scanner.range + 1);
                }
            }
        }
        update(firewall)
    }
    severity
}

//loop over the depths, get the scanner of the current depth.
//if the scanner's current position is 0, we're caught. return false.
fn caught(firewall: &mut [Option<Scanner>]) -> bool {
    for i in 0..firewall.len() {
        if let Some(opt_scanner) = firewall.get(i) {
            if let &Some(ref scanner) = opt_scanner {
                if scanner.current == 0 {
                    return true;
                }
            }
        }
        update(firewall)
    }
    false
}

//get the state, clone it into tmp.
//while we're caught in the tmp, update the state, and set tmp to state.
fn part2() -> i64 {
    let mut state = parse(PUZZLE);
    let mut tmp = state.clone();
    let mut waited = 0;
    
    while caught(&mut tmp) {
        update(&mut state);
        tmp = state.clone();
        waited += 1;
    }

    waited
}

fn main() {
    let mut parsed = parse(PUZZLE);
    
    println!("part 1: {}", severity(&mut parsed));
    println!("part 2: {}", part2());
}

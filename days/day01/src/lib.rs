mod part2;

pub fn solution_part1(actions: &[Action], start: usize) -> usize {
    let mut dial = Dial::<99>::new(start);
    let mut zeros = 0;

    for action in actions {
        zeros += dial.apply(action) as usize;
    }

    zeros
}

pub fn solution_part2(actions: &[Action], start: usize) -> usize {
    let mut dial = Dial::<99>::new(start);
    let mut zeros = 0;

    for action in actions {
        zeros += dial.apply_part2(action);
    }

    zeros
}

pub struct Dial<const MAX: usize> {
    inner: usize,
}

impl<const MAX: usize> Dial<MAX> {
    pub fn new(start: usize) -> Self {
        assert!(start <= MAX);
        Self { inner: start }
    }

    /// apply a rotation action, and return true if the inner value is set to 0
    pub fn apply(&mut self, action: &Action) -> bool {
        self.inner = match action.movement {
            Movement::Left => wrapping_sub::<MAX>(self.inner, action.steps),
            Movement::Right => wrapping_add::<MAX>(self.inner, action.steps),
        };

        self.inner == 0
    }
}

fn wrapping_add<const MAX: usize>(a: usize, b: usize) -> usize {
    let max_exclusive = MAX + 1;
    (a + b) % max_exclusive
}

fn wrapping_sub<const MAX: usize>(a: usize, mut b: usize) -> usize {
    let max_exclusive = MAX + 1;

    while b > MAX {
        b -= max_exclusive
    }

    (a + MAX + 1 - b) % max_exclusive
}

pub struct Action {
    pub movement: Movement,
    pub steps: usize,
}

pub enum Movement {
    Left,
    Right,
}

impl Action {
    pub fn parse(input: &str) -> Vec<Action> {
        // picking a vector capacity which is larger than the input file
        // I dont need to do this here, and it is probably horribly inefficient. But I will
        // make that decision if I notice my
        let mut output = Vec::with_capacity(8192);

        for split in input.split("\n") {
            if split.len() < 2 {
                continue;
            }

            let split_bytes = split.as_bytes();
            let movement = Movement::from_byte(split_bytes[0]);

            // SAFETY:
            // - The sequence of bytes passed into this function were originally derived from &str. So
            //   as long as the caller of this function respects the invariance of &str, this function
            //   has no UB
            let num_part = unsafe { str::from_utf8_unchecked(&split_bytes[1..]) };

            let steps = num_part.parse::<usize>().expect("valid number part");

            output.push(Action { movement, steps });
        }

        output
    }
}

impl Movement {
    fn from_byte(c: u8) -> Self {
        match c {
            b'L' => Movement::Left,
            b'R' => Movement::Right,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.movement {
            Movement::Left => write!(f, "L{}", self.steps),
            Movement::Right => write!(f, "R{}", self.steps),
        }
    }
}

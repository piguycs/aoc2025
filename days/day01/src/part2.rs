use super::*;

impl<const MAX: usize> Dial<MAX> {
    pub fn apply_part2(&mut self, action: &Action) -> usize {
        let mut zeroes = 0;

        match action.movement {
            Movement::Left => {
                for _ in 0..action.steps {
                    zeroes += self.backwards() as usize;
                }
            }
            Movement::Right => {
                for _ in 0..action.steps {
                    zeroes += self.forward() as usize;
                }
            }
        };

        zeroes
    }

    // returns true if it made a full rotation around 0
    fn forward(&mut self) -> bool {
        if self.inner == MAX {
            self.inner = 0;
            true
        } else {
            self.inner += 1;
            false
        }
    }

    // returns true if it made a full rotation around 0
    fn backwards(&mut self) -> bool {
        if self.inner == 0 {
            self.inner = MAX;
            false
        } else {
            self.inner -= 1;
            self.inner == 0
        }
    }
}

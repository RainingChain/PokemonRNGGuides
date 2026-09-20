#[derive(PartialEq, Debug)]
pub struct MinMax {
    min: usize,
    max: usize,
    has_val: bool,
}

impl MinMax {
    pub fn contains(&self, val: &usize) -> bool {
        *val >= self.min && *val <= self.max
    }
    pub fn update_bounds(&mut self, new_val: usize) {
        if !self.has_val {
            self.min = new_val;
            self.max = new_val;
            self.has_val = true;
        } else {
            self.min = new_val.min(self.min);
            self.max = new_val.max(self.max);
        }
    }

    pub const fn new(min: usize, max: usize) -> Self {
        Self {
            min,
            max,
            has_val: true,
        }
    }
    pub fn empty() -> Self {
        Self {
            min: 0,
            max: 0,
            has_val: false,
        }
    }
}

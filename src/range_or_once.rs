use std::iter::{once, Once};

pub enum RangeOrOnce {
    OneTime(Once<f64>),
    Range(RangeIter),
}

impl Iterator for RangeOrOnce {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RangeOrOnce::OneTime(it) => it.next(),
            RangeOrOnce::Range(it) => it.next(),
        }
    }
}

#[derive(Clone)]
pub struct RangeIter {
    current: f64,
    stop: f64,
    step: f64,
}

impl Iterator for RangeIter {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let eps = self.step.abs() * 1e-9;
        if self.current + eps < self.stop {
            let val = self.current;
            self.current += self.step;
            Some(val)
        } else {
            None
        }
    }
}

pub enum Input {
    Single(f64),
    Tuple(f64, f64, f64),
}

impl From<f64> for Input {
    fn from(val: f64) -> Self {
        Input::Single(val)
    }
}

impl From<(f64, f64, f64)> for Input {
    fn from((start, stop, step): (f64, f64, f64)) -> Self {
        Input::Tuple(start, stop, step)
    }
}

pub fn make_range_or_once<T: Into<Input>>(input: T) -> RangeOrOnce {
    match input.into() {
        Input::Single(val) => RangeOrOnce::OneTime(once(val)),
        Input::Tuple(start, stop, step) => RangeOrOnce::Range(RangeIter { current: start, stop, step})
    }
}

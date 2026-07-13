use std::{collections::VecDeque, str::FromStr};

pub struct Input {
    tokens: VecDeque<String>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            tokens: VecDeque::new(),
        }
    }

    pub fn next<T: FromStr>(&mut self) -> Result<T, Box<dyn std::error::Error>> {
        while self.tokens.is_empty() {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf)?;
            self.tokens.extend(buf.split_whitespace().map(String::from));
        }
        let token = self.tokens.pop_front().unwrap();
        token
            .parse::<T>()
            .map_err(|_| format!("Failed to parse '{}'", token).into())
    }
}

pub trait InputArray: Sized {
    fn read(rustin: &mut Input) -> Result<Self, Box<dyn std::error::Error>>;
}

impl<T: FromStr, const N: usize> InputArray for [T; N] {
    fn read(rustin: &mut Input) -> Result<Self, Box<dyn std::error::Error>> {
        let mut arr = Vec::with_capacity(N);
        for _ in 0..N {
            arr.push(rustin.next()?);
        }
        arr.try_into()
            .map_err(|_| "Failed to collect into array".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_one() {
        let mut input = Input::new();
        let arr: [i32; 3] = InputArray::read(&mut input).unwrap();
        eprintln!("arr = {:?}", arr);
    }
}

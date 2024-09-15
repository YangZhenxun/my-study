use std::{fmt::Display, ops::{Add, AddAssign, Sub}};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CStrPtr{
    pub string: String,
    pub offset: i64,
}

impl Add<i64> for CStrPtr{
    type Output = Self;
    fn add(mut self, rhs: i64) -> Self::Output {
        if (self.offset+rhs) as usize > self.string.len() {
            for _ in 1..=((self.offset+rhs) as usize-self.string.len()){
                self.string += " ";
            }
        }
        for (u, _c) in self.string.chars().enumerate(){
            if u == (self.offset+rhs).try_into().expect("i64 into usize err") {
                return Self{string: self.string, offset: self.offset+rhs};
            }
        }
        self
    }
}

impl AddAssign<i64> for CStrPtr{
    fn add_assign(&mut self, rhs: i64) {
        *self = self.clone().add(rhs);
    }
}

impl Sub<i64> for CStrPtr{
    type Output = Self;
    fn sub(self, rhs: i64) -> Self::Output {
        self.add(-rhs)
    }
}

impl Sub<String> for CStrPtr{
    type Output = i64;
    fn sub(self, rhs: String) -> Self::Output {
        if self.string == rhs {
            return self.offset;
        }
        0
    }
}

impl Sub<&str> for CStrPtr {
    type Output = i64;
    fn sub(self, rhs: &str) -> Self::Output {
        if self.string == rhs {
            return self.offset;
        }
        0
    }
}

impl CStrPtr{
    pub fn new(string: String, offset: i64) -> Self {
        CStrPtr{string, offset}
    }
    pub fn get_string(&self) -> String{
        let mut result = String::new();
        for (u, c) in self.string.chars().enumerate(){
            if u >= self.offset as usize {
                result.push(c);
            }
        }
        result
    }
    pub fn len(&self) -> usize {
        self.to_string().len()
    }
    pub fn to_i64(&self) -> i64 {
        self.offset
    }
    pub fn point_at(&mut self, who: char) {
        let mut string = String::new();
        for (u, c) in self.string.chars().enumerate(){
            if u != self.offset as usize{
                string.push(c);
            }
            else {
                string.push(who);
            }
        }
        self.string = string;
    }
}

impl Display for CStrPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_string())
    }
}

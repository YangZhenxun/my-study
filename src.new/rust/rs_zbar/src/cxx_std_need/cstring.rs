use super::CStrPtr;

pub fn strchr(astr: &str, c: char) -> Option<(usize, String)>{
    for (u, a) in astr.chars().enumerate(){
        if c == a {
            let mut aft = String::new();
            for (ui, ci) in astr.chars().enumerate(){
                if ui > u {
                    aft += ci.to_string().as_str()
                }
            }
            return Some((u, a.to_string()+aft.as_str()))
        }
    }
    None
}

pub fn strncmp(str1: &str, str2: &str, n: usize) -> i64{
    let mut string1 = String::new();
    let mut string2 = String::new();
    for (u, i) in str1.to_string().chars().enumerate(){
        if u+1 <= n{
            string1.push(i);
        } else {
            break;
        }
    } 
    for (u, i) in str2.to_string().chars().enumerate(){
        if u+1 <= n{
            string2.push(i);
        } else {
            break;
        }
    }
    if string1 < string2 {
        return -1;
    } else if string1 == string2 {
        return 0;
    } else {
        return 1;
    }
}

pub fn strptrchr(astr: CStrPtr, c: char) -> Option<CStrPtr>{
    for (u, a) in astr.get_string().chars().enumerate() {
        if c == a {
            return Some(CStrPtr::new(astr.string, u.try_into().unwrap()))
        }
    }
    None
}

pub fn strtol(string: &str, _end_ptr: Option<&str>, base: u32) -> u32{
    let mut res: u32 = 0;
    for c in string.chars(){
        match c.to_digit(base){
            Some(digit) => res = res*base + digit,
            None => (),
        }
    } 
    res
}
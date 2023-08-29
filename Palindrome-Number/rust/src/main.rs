fn main() {
    if is_palindrome(121) {
        println!("That's a palindrome")
    } else {
        println!("That isn't a palindrome")
    }
}

pub fn is_palindrome(num:i32) -> bool {
    match num {
        0 => { return true },
        i32::MIN..=0 => { false },
        1i32..=i32::MAX => {
            let rev:String = num.to_string().chars().rev().collect();
            rev == num.to_string()
        }
    }
}

fn main() {
    list_numbers(1, 32);
}

fn list_numbers(from: u32, to: u32) { 
    for i in from..to { 
        println!("{}, {:b}, {:x}", i, i, i); 
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 { 
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(2, 1), 1);
    }

    #[test]
    fn test_pow() { 
        let ten: u32 = 10; 
        let three: u32 = 3; 
        let zero: u32 = 0; 
        assert_eq!(ten.pow(three), 1000);
        assert_eq!(ten.pow(zero), 1);
    }

    #[test]
    fn test_bin() { 
        let ten: u32 = 10;
        assert_eq!(format!("{:b}", ten), "1010");

        let ten_bin = format!("{:b}", ten);
        assert_eq!(ten_bin, "1010");
    }
}
/*!
 * https://www.codewars.com/kata/52e88b39ffb6ac53a400022e/train/rust
 */

pub fn int32_to_ip(int: u32) -> String {
    let base = 2_u32.pow(8);
    let mut digits = vec![];
    let mut num = int;
    while digits.len() != 4 {
        digits.push(num % base);
        num /= base;
    }
    digits
        .iter()
        .rev()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}

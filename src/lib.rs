pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_2() {
        let result = add(3, 3);
        assert_eq!(result, 6);
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        let result = add(4, 4);
        assert_eq!(result, 8);
    }
}

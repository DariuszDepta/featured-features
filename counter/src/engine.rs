/// Returns the number of elements in a list.
pub fn count<T>(list: Vec<T>) -> usize {
    list.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_i8() {
        assert_eq!(4, count(vec![12u8, 1u8, 217u8, 34u8]));
    }

    #[test]
    fn count_str() {
        assert_eq!(3, count(vec!["3", "67", "23"]));
    }
}

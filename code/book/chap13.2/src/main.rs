fn main() {
    let v1 = vec![1, 2, 3];
    let v_iter = v1.iter();
    for val in v_iter {
        println!("Got: {:?}", val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstrator() {
        let v1 = vec!["a", "b", "z"];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&"a"));
        assert_eq!(v1_iter.next(), Some(&"b"));
        assert_eq!(v1_iter.next(), Some(&"z"));
        assert_eq!(v1_iter.next(), None);
    }
}

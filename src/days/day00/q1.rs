pub fn q1() {
    println!("fake template for q1");
}

#[cfg(test)]
mod q1_test {
    use super::q1;

    #[test]
    fn test_q1_fucntionality() {
        assert_eq!(q1(), ());
    }
}

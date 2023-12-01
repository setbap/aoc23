pub fn q2() {
    println!("fake template for q2")
}

#[cfg(test)]
mod q2_test {
    use super::q2;

    #[test]
    fn test_q2_fucntionality() {
        assert_eq!(q2(), ());
    }
}

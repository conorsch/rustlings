// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let var1: u8 = 1;
        let var2: u8 = 1;

        assert_eq!(var1, var2);
    }
}

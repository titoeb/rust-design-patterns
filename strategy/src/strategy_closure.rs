struct Adder;

impl Adder {
    pub fn add<F>(x: u8, y: u8, f: F) -> u8
    where
        F: Fn(u8, u8) -> u8,
    {
        f(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arith_adder() {
        assert_eq!(9, Adder::add(4, 5, |x, y| x + y));
    }
    #[test]
    fn bool_adder() {
        assert_eq!(
            0,
            Adder::add(0, 0, |x, y| {
                if x == 1 || y == 1 {
                    1
                } else {
                    0
                }
            })
        )
    }
    #[test]
    fn custom_adder() {
        assert_eq!(8, Adder::add(2, 2, |x, y| { 2 * x * y }))
    }
}

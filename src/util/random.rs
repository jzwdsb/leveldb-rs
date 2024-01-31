pub struct Random {
    seed: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        let seed = seed & 0x7fffffff;
        if seed == 0 || seed == u32::MAX {
            Self { seed: 1 }
        } else {
            Self { seed }
        }
    }

    pub fn next(&mut self) -> u32 {
        let product = self.seed.wrapping_mul(u16::MAX as u32);
        self.seed = ((product >> 31) + (product & u32::MAX)) as u32;
        self.seed %= u32::MAX;
        self.seed
    }

    pub fn uniform(&mut self, n: u32) -> u32 {
        self.next() % n
    }

    pub fn one_in(&mut self, n: u32) -> bool {
        self.next() % n == 0
    }

    pub fn skew(&mut self) -> u32 {
        let mut r = self.next();
        r = r.wrapping_mul(self.next());
        r >> 16
    }
}

mod test {
    use super::*;

    #[test]
    fn test_random() {
        todo!()
    }

    #[test]
    fn test_uniform() {
        todo!()
    }

    #[test]
    fn test_one_in() {
        todo!()
    }

    #[test]
    fn test_skew() {
        todo!()
    }
}

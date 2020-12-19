use santa18::santa_replace;
#[derive(Debug)]
struct SantaNum(usize);

impl std::ops::Add for SantaNum {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl std::ops::Mul for SantaNum {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

pub fn solve() -> usize {
    let results = include!("./part2-input.txt");
    let sum: usize = results.iter().map(|v| v.0).sum();
    sum
}

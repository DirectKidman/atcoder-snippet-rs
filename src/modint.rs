use cargo_snippet::snippet;

#[snippet("Modint")]
use std::mem::swap;
use std::ops::*;

#[snippet("Modint")]
#[derive(Copy, Clone, Debug)]
pub struct Modint<const MOD: usize> {
    val: usize,
}

#[snippet("Modint")]
impl<const MOD: usize> Modint<MOD> {
    pub fn new(val: usize) -> Self {
        Modint { val: val % MOD }
    }

    fn inv(&self) -> Self {
        let mut x: i32 = 1;
        let mut u: i32 = 0;
        let mut s: i32 = self.val as i32;
        let mut t: i32 = MOD as i32;

        while t != 0 {
            let k = s / t;
            s -= k * t;
            swap(&mut s, &mut t);
            x -= k * t;
            swap(&mut x, &mut u);
        }
        x %= MOD as i32;
        if x < 0 {
            x += MOD as i32;
        }

        let x = x as usize;
        Modint { val: x }
    }
}

#[snippet("Modint")]
impl<const MOD: usize> Add for Modint<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            val: (self.val + rhs.val) % MOD,
        }
    }
}

#[snippet("Modint")]
impl<const MOD: usize> AddAssign for Modint<MOD> {
    fn add_assign(&mut self, rhs: Modint<MOD>) {
        *self = *self + rhs;
    }
}

#[snippet("Modint")]
impl<const MOD: usize> Sub for Modint<MOD> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            val: (MOD + self.val - other.val) % MOD,
        }
    }
}

#[snippet("Modint")]
impl<const MOD: usize> SubAssign for Modint<MOD> {
    fn sub_assign(&mut self, rhs: Modint<MOD>) {
        *self = *self - rhs;
    }
}

#[snippet("Modint")]
impl<const MOD: usize> Mul for Modint<MOD> {
    type Output = Modint<MOD>;

    fn mul(self, rhs: Modint<MOD>) -> Self {
        Modint {
            val: (self.val * rhs.val) % MOD,
        }
    }
}

#[snippet("Modint")]
impl<const MOD: usize> MulAssign for Modint<MOD> {
    fn mul_assign(&mut self, rhs: Modint<MOD>) {
        *self = *self * rhs;
    }
}

#[snippet("Modint")]
impl<const MOD: usize> Div for Modint<MOD> {
    type Output = Modint<MOD>;

    fn div(self, rhs: Modint<MOD>) -> Self {
        Modint {
            val: (self.val * rhs.inv().val) % MOD,
        }
    }
}

#[snippet("Modint")]
impl<const MOD: usize> DivAssign for Modint<MOD> {
    fn div_assign(&mut self, rhs: Modint<MOD>) {
        *self = *self / rhs;
    }
}

#[snippet("Modint")]
impl<const MOD: usize> From<i32> for Modint<MOD> {
    fn from(x: i32) -> Self {
        let mut tmp = x;
        while tmp < 0 {
            tmp += MOD as i32;
        }
        let res = Modint::new(tmp as usize);
        res
    }
}

#[snippet("Modint")]
impl<const MOD: usize> From<usize> for Modint<MOD> {
    fn from(x: usize) -> Self {
        let res = Modint::new(x);
        res
    }
}

#[snippet("Modint")]
impl<const MOD: usize> std::fmt::Display for Modint<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[snippet("Modint")]
impl<const MOD: usize> PartialEq for Modint<MOD> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

#[cfg(test)]
mod tests {
    use super::Modint;

    #[test]
    fn modint_test() {
        type Mint = Modint<1_000_000_007>;
        let x: Mint = 1_000_000.into();
        let y: Mint = 2_000_000.into();
        assert_eq!(x * y, 999986007.into());
    }
}

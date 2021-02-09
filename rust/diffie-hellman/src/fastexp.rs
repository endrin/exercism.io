pub fn exp_mod(a: u64, e: u64, m: u64) -> u64 {
    ExpMod::new(a, e, m).last().unwrap_or(1)
}

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    let mut a = a % m;
    let mut b = b % m;
    let mut c: u64 = 0;

    while b > 0 {
        if b % 2 == 1 {
            c = add_mod(c, a, m);
        }
        a = add_mod(a, a, m);
        b /= 2;
    }

    c
}

fn add_mod(a: u64, b: u64, m: u64) -> u64 {
    a.checked_add(b).unwrap_or_else(|| {
        a.wrapping_sub(m).wrapping_add(b)
    }) % m
}

struct ExpMod {
    a: u64,
    e: u64,
    m: u64,
    c: u64,
}

impl ExpMod {
    fn new(a: u64, e: u64, m: u64) -> Self {
        Self { a, e, m, c: 1 }
    }
}

impl Iterator for ExpMod {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.e == 0 {
            return None;
        }

        if self.e % 2 == 0 {
            self.e = self.e / 2;
            self.a = mul_mod(self.a, self.a, self.m)
        } else {
            self.e -= 1;
            self.c = mul_mod(self.c, self.a, self.m)
        }

        Some(self.c)
    }
}

#[cfg(test)]
mod tests {
    use super::{exp_mod, mul_mod};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn expmod_is_same_as_pow_with_mod(
            a in 0..100_u64,
            e in 0..10_u64,
            m in 1u64..
        ) {
            prop_assume!(m > a);

            prop_assert_eq!(
                exp_mod(a, e, m),
                a.pow(e as u32) % m)
        }

        #[test]
        fn mulmod_is_same_as_mul_with_mod(
            a in 0..10000_u64,
            b in 0..10000_u64,
            m in 1u64..
        ) {
            prop_assume!(m > a);

            prop_assert_eq!(
                mul_mod(a, b, m),
                (a * b) % m)
        }
    }
}

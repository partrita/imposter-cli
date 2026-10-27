/// Tiny xorshift64* PRNG: no dependencies, no OS entropy calls, ~16 bytes of state.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x9e37_79b9_7f4a_7c15)
        });
        Self(if seed == 0 {
            0x9e37_79b9_7f4a_7c15
        } else {
            seed
        })
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// Inclusive range.
    #[inline]
    pub fn range(&mut self, lo: u64, hi: u64) -> u64 {
        if hi <= lo {
            lo
        } else {
            lo + self.next_u64() % (hi - lo + 1)
        }
    }

    #[inline]
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_u64() % n as u64) as usize
        }
    }

    #[inline]
    pub fn chance(&mut self, num: u32, den: u32) -> bool {
        den != 0 && (self.next_u64() % den as u64) < num as u64
    }
}

/// A shuffled bag: every item is returned once before anything repeats.
/// When the bag empties it reshuffles, and never returns the same item twice
/// in a row across the seam.
pub struct Deck<T: Copy + PartialEq> {
    items: Vec<T>,
    pos: usize,
    last: Option<T>,
}

impl<T: Copy + PartialEq> Deck<T> {
    pub fn new(items: &[T], rng: &mut Rng) -> Self {
        let mut deck = Self {
            items: items.to_vec(),
            pos: usize::MAX,
            last: None,
        };
        deck.reshuffle(rng);
        deck
    }

    pub fn next(&mut self, rng: &mut Rng) -> T {
        if self.pos >= self.items.len() {
            self.reshuffle(rng);
        }
        let value = self.items[self.pos];
        self.pos += 1;
        self.last = Some(value);
        value
    }

    fn reshuffle(&mut self, rng: &mut Rng) {
        for i in (1..self.items.len()).rev() {
            let j = rng.below(i + 1);
            self.items.swap(i, j);
        }
        if let (Some(last), true) = (self.last, self.items.len() > 1) {
            if self.items[0] == last {
                self.items.swap(0, 1);
            }
        }
        self.pos = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::new(Some(7));
        let mut b = Rng::new(Some(7));
        for _ in 0..32 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn range_is_inclusive_and_bounded() {
        let mut r = Rng::new(Some(1));
        for _ in 0..1000 {
            let v = r.range(3, 9);
            assert!((3..=9).contains(&v));
        }
    }
}

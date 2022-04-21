use std::cmp::max;
use std::cmp::PartialEq;
use std::mem::swap;

// Diff has 2 sequences to calculate difference
pub struct Diff<T> {
    a: Vec<T>,
    b: Vec<T>,
    m: usize,
    n: usize,
}

// DiffResult is result of diff calculation
pub struct DiffResult {
    ed: usize,
    // lcs: Vec<T>,
    // ses: Vec<T>,
}

impl DiffResult {
    // ed returns editdistance between 2 sequences
    pub fn ed(&self) -> usize {
        return self.ed;
    }
}

impl<T: PartialEq> Diff<T> {
    // new is initializer of Diff
    pub fn new(mut a: Vec<T>, mut b: Vec<T>) -> Diff<T> {
        let mut m = a.len();
        let mut n = b.len();
        if a.len() > b.len() {
            swap(&mut a, &mut b);
            swap(&mut m, &mut n);
        }
        return Diff::<T> {
            a: a,
            b: b,
            m: m,
            n: n,
        };
    }

    // build builds difference between 2 sequences
    pub fn build(&self) -> DiffResult {
        let offset: usize = self.m + 1;
        let delta: usize = self.n - self.m;
        let size = self.m + self.n + 3;
        let mut fp: Vec<isize> = vec![-1; size];

        let mut p: usize = 0;
        loop {
            let mut k = offset - p;
            while k <= delta + offset - 1 {
                let y = max(fp[k - 1] + 1, fp[k + 1]);
                fp[k] = self.snake(y - (k as isize) + (offset as isize), y);
                k = k + 1;
            }
            let mut l = delta + p;
            while l >= delta + 1 {
                let y = max(fp[l + offset - 1] + 1, fp[l + 1 + offset]);
                fp[l + offset] = self.snake(y - (l as isize), y);
                l = l - 1;
            }

            let y = max(fp[delta + offset - 1] + 1, fp[delta + 1 + offset]);
            fp[delta + offset] = self.snake(y - (delta as isize), y);

            if fp[delta + offset] >= (self.n as isize) {
                break;
            }
            p = p + 1;
        }
        return DiffResult { ed: delta + 2 * p };
    }

    fn snake(&self, mut x: isize, mut y: isize) -> isize {
        while x >= 0
            && y >= 0
            && x < self.m as isize
            && y < self.n as isize
            && self.a[x as usize] == self.b[y as usize]
        {
            x = x + 1;
            y = y + 1;
        }

        return y;
    }
}

mod test {
    #[test]
    fn test_strdiff() {
        let a = "abc";
        let b = "abd";
        let diff = super::Diff::new(a.chars().collect::<Vec<_>>(), b.chars().collect::<Vec<_>>());
        let res = diff.build();
        assert_eq!(2, res.ed());
    }
    #[test]
    fn test_intdiff() {
        let a = vec![1, 2, 3];
        let b = vec![1, 5, 3];
        let diff = super::Diff::new(a, b);
        let res = diff.build();
        assert_eq!(2, res.ed());
    }
}

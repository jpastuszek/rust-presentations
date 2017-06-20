struct Coll {
    state: Option<u64>
}

fn main() {
    let iter = Coll {
        state: Some(13)
    };

    println!("{:?}", iter.collect::<Vec<_>>());
    // [13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
}

impl Iterator for Coll {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let ret = self.state;
        self.state = match self.state {
            None => None,
            Some(n) if n <= 1 => None,
            Some(n) if n % 2 == 0 => Some(n / 2),
            Some(n) => Some(n * 3 + 1)
        };
        ret
    }
}



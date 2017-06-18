#![feature(conservative_impl_trait)]

fn even(from: u64, to: u64) -> impl Iterator<Item = u64> {
    // type: std::iter::Filter<std::ops::Range<u64>, [closure@<anon>:4:27: 4:41]>
    (from..to + 1).filter(|i| i % 2 == 0)
}

fn main() {
    println!("{:?}", even(1, 10).collect::<Vec<_>>()) // [2, 4, 6, 8, 10]
}



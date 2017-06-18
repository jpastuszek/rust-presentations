let mut numbers = vec![1, 2, 3, 4, 5];
for n in &mut numbers {
    *n *= 2;
    println!("{}", n)
}
println!("{:?}", numbers); // [2, 4, 6, 8, 10]

let mut numbers = vec![1, 2, 3, 4, 5];
{
    let mut i = (&mut numbers).into_iter(); // IntoIterator trait
    loop {
        match i.next() {
            Some(n) => {
                *n *= 2;
                println!("{}", n)
            }
            None => break,
        }
    }
}
println!("{:?}", numbers); // [2, 4, 6, 8, 10]


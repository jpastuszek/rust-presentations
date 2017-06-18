let numbers = vec![1, 2, 3, 4, 5];
for n in &numbers {
    println!("{}", n)
}
println!("{:?}", numbers); // [1, 2, 3, 4, 5]

let numbers = vec![1, 2, 3, 4, 5];
{
    let mut i = (&numbers).into_iter(); // IntoIterator trait
    loop {
        match i.next() {
            Some(n) => println!("{}", n),
            None => break,
        }
    }
}
println!("{:?}", numbers); // [1, 2, 3, 4, 5]



let numbers = vec![1, 2, 3, 4, 5];
for n in numbers {
    println!("{}", n)
}
// println!("{:?}", numbers); -- error[E0382]: use of moved value: `numbers`

let numbers = vec![1, 2, 3, 4, 5];
{
    let mut i = numbers.into_iter(); // IntoIterator trait
    loop {
        match i.next() {
            Some(n) => println!("{}", n),
            None => break,
        }
    }
}
// println!("{:?}", numbers); -- error[E0382]: use of moved value: `numbers`




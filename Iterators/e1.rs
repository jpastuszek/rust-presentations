for word in "The quick brown fox jumps over the lazy dog"
    .split_whitespace()
    .map(|w| w.chars())
    .map(|mut c| {
        c.next().into_iter().flat_map(|c| c.to_uppercase()).chain(
            c.flat_map(|c| c.to_lowercase()),
        )
    })
    .map(|c| c.collect::<String>())
{
    print!("{:?} ", word); // "The" "Quick" "Brown" "Fox" "Jumps" "Over" "The" "Lazy" "Dog"
}



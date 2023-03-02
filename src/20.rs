use std::collections::HashMap;

fn main () {
    // compute d(a) for all a in 1..10000
    // put it into hashmap wit key=a and value=d(a)

    let divisors: HashMap<i32, i32> = (1..10000).map(|number| {
            (number,
            (1..=(number/2)).filter(|div| number % div == 0).sum())})
        .collect();

    // get all pairs where key!=value and hashmap[value]==key
    // sum all those

    let sum = divisors.iter()
        .filter(|(key, value)| { match divisors.get(value) {
            Some(k) => k == *key && key != value,
            _ => false }})
        .fold(0, |acc, (key, value)| {println!("{}, {}", key, value); acc + key});

    println!("sum {}", sum);
}

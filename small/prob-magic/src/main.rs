use num::BigUint as biguint;
use std::cmp::{max, min};
use std::io;

fn main() {
    let deck_size = 60;
    //    println!("Deck size (default 60): ");
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).unwrap();
    //let deck_size = input.trim().parse::<usize>().unwrap_or(60);

    println!("At least one of: ");
    let at_least_one = get_vec_from_input();
    println!("Only one of: ");
    let only_one = get_vec_from_input();
    let on_7 = on_cards(&at_least_one, &only_one, deck_size, 7);
    let on_6 = on_cards(&at_least_one, &only_one, deck_size, 6);
    println!("First hand: {}%", on_7 * 100.);
    println!("First mullingan: {:.2}%", on_6 * 100.);
    println!(
        "Sum of the previous ones: {:.2}%",
        (on_7 + (1. - on_7) * on_6) * 100.
    );

    let mut explicit = biguint::from(0usize);
    for i in 1..4 {
        let mut sum = biguint::from(0usize);
        for j in 1..=min(6 - i, 5) {
            sum += biguint::from(binomiale(8, j)) * biguint::from(binomiale(44 - i - j, 6 - i - j));
        }
        explicit += sum * binomiale(4, i);
    }
    let tmp = binomiale(60, 7);
    println!("Explicit: {}", tmp);
}

fn get_vec_from_input() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

#[allow(dead_code)]
fn on_cards(
    at_least_one: &[usize],
    only_one: &[usize],
    deck_size: usize,
    nums_cards: usize,
) -> f64 {
    let from = at_least_one.len();
    let to = nums_cards - only_one.len();
    let cards = (from..to)
        .map(|i| {
            binomiale(
                deck_size - only_one.iter().sum::<usize>() - i,
                nums_cards - only_one.len() - i,
            )
        })
        .collect::<Vec<usize>>();

    let costanti: f64 =
        only_one.iter().product::<usize>() as f64 / binomiale(deck_size, nums_cards) as f64;
    costanti * compute(&at_least_one, &cards, 0)
}

fn binomiale(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    (max(k, n - k) + 1..=n).product::<usize>() / (1..=min(k, n - k)).product::<usize>()
}

fn compute(bins: &[usize], cards: &[usize], i: usize) -> f64 {
    if bins.len() == 0 {
        return cards[i] as f64;
    }
    let mut res = 0.;
    for j in 0..min(cards.len() - i, bins[0] + 1) {
        res += binomiale(bins[0], j + 1) as f64 * compute(&bins[1..], cards, i + j);
    }
    res
}

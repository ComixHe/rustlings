// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter_mut();

    assert_eq!(my_iterable_fav_fruits.next(), Some(&mut "banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&mut "custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&mut "avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&mut "peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&mut "raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);
}

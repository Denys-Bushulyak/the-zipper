use criterion::{Criterion, black_box, criterion_group, criterion_main};
use the_zipper::*;
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("go up", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("c"),
            ])));

            location.go_up()
        })
    });
    c.bench_function("go down", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("c"),
            ])));

            location.go_down()
        })
    });
    c.bench_function("go left", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("c"),
            ])));

            location.go_left()
        })
    });
    c.bench_function("go right", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("c"),
            ])));

            location.go_right()
        })
    });
    c.bench_function("go down and up", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("c"),
            ])));

            location.go_down().map(Location::go_up)
        })
    });
    c.bench_function("go left and right", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("c"),
            ])));

            location.go_left().map(Location::go_right)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

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
    c.bench_function("get_nth", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("b"),
                Tree::Item("*"),
                Tree::Item("c"),
            ])));

            location.get_nth(2)
        })
    });
    c.bench_function("memo_get_nth", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("b"),
                Tree::Item("*"),
                Tree::Item("c"),
            ])));

            let memo_location = location.with_memo();
            memo_location.get_nth(2)
        })
    });
    c.bench_function("repeated_get_nth", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("b"),
                Tree::Item("*"),
                Tree::Item("c"),
            ])));

            for _ in 0..5 {
                let _ = location.clone().get_nth(2);
            }
        })
    });
    c.bench_function("repeated_memo_get_nth", |b| {
        b.iter(|| {
            let location = black_box(Location::new(Tree::Section(vec![
                Tree::Item("a"),
                Tree::Item("+"),
                Tree::Item("b"),
                Tree::Item("*"),
                Tree::Item("c"),
            ])));

            let memo_location = location.with_memo();
            for _ in 0..5 {
                let _ = memo_location.clone().get_nth(2);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

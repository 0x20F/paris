use criterion::{criterion_group, criterion_main, Criterion};
use paris::Formatter;


pub fn colorize_with_tags_benchmark(c: &mut Criterion) {
    c.bench_function(
        "logger formatting with tags",
        |b| b.iter(|| {
            Formatter::colorise_string("<cyan><wrong>normal \
            text</> <on black> <green> <html> <more text as tag> \
            </what> this should be a very very long string so everything can burn\
            and test whatever, <heres a wrong tag> <on blue> heres something\
            on a blue background</>");
        })
    );
}



pub fn colorize_without_tags_benchmark(c: &mut Criterion) {
    c.bench_function(
        "logger formatting no tags",
        |b| b.iter(|| {
            Formatter::colorise_string("cyan wrong normal \
            text / on black green html more text as tag \
            /what this should be a very very long string so everything can burn\
            and test whatever, heres a wrong tag on blue heres something\
            on a blue background/");
        })
    );
}


criterion_group!(
    benches,
    colorize_with_tags_benchmark,
    colorize_without_tags_benchmark
);
criterion_main!(benches);
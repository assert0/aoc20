#[macro_use]
extern crate bencher;

use bencher::Bencher;
use aoc20::days;

static EXAMPLE: [i32; 6] = [1721, 979, 366, 299, 675, 1456];


fn nested_find(bench: &mut Bencher) {
    bench.iter(|| {
        days::day1::find2(&EXAMPLE.to_vec(), 2020)
    })
}

fn itertool_find(bench: &mut Bencher) {
    bench.iter(|| {
        days::day1::find(&EXAMPLE.to_vec(), 2020, 3)
    })
}

benchmark_group!(benches, nested_find, itertool_find);
benchmark_main!(benches);
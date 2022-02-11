# Vec vs. HashMap for Lookups

I had some code that provided a `Vec` of ids, which was a mapping between an old id (the value in the `Vec`)
and its new id (the position in the `Vec`). It was originally only used to map new ids to old ids, which is
just accessing the item at the `new id`.

The need to go the other way then came about: map old id to new id. My hypothesis was that up to a certain
size just searching the `Vec` would be faster than using a `HashMap`. This repo aims to test that theory.

I ended up comparing `Vec`, `HashMap`, and `FxHashMap` in 5 different sizes. The results on my machine
showed that `FxHashMap` was pretty much always the best option.

## Results

Test machine: AMD Ryzen 5700G (8c/16t)

    test bench_fxmap_10   ... bench:         262 ns/iter (+/- 15)
    test bench_fxmap_50   ... bench:         262 ns/iter (+/- 5)
    test bench_fxmap_250  ... bench:         263 ns/iter (+/- 10)
    test bench_fxmap_500  ... bench:         263 ns/iter (+/- 6)
    test bench_fxmap_1000 ... bench:         264 ns/iter (+/- 23)
    test bench_map_10     ... bench:         691 ns/iter (+/- 32)
    test bench_map_50     ... bench:         700 ns/iter (+/- 9)
    test bench_map_250    ... bench:         692 ns/iter (+/- 16)
    test bench_map_500    ... bench:         711 ns/iter (+/- 25)
    test bench_map_1000   ... bench:         695 ns/iter (+/- 17)
    test bench_vec_10     ... bench:         341 ns/iter (+/- 81)
    test bench_vec_50     ... bench:       1,302 ns/iter (+/- 38)
    test bench_vec_250    ... bench:       4,249 ns/iter (+/- 85)
    test bench_vec_500    ... bench:       7,841 ns/iter (+/- 220)
    test bench_vec_1000   ... bench:      13,970 ns/iter (+/- 215)

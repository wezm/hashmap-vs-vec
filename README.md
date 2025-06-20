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

Test machine: AMD Ryzen AI 7 350 (8c/16t); adds 5 item tests

    test bench_fxmap_5    ... bench:         169.82 ns/iter (+/- 38.20)
    test bench_fxmap_10   ... bench:         168.99 ns/iter (+/- 0.59)
    test bench_fxmap_50   ... bench:         170.44 ns/iter (+/- 2.67)
    test bench_fxmap_250  ... bench:         169.25 ns/iter (+/- 37.40)
    test bench_fxmap_500  ... bench:         170.98 ns/iter (+/- 1.06)
    test bench_fxmap_1000 ... bench:         169.33 ns/iter (+/- 0.52)
    test bench_map_5      ... bench:         443.05 ns/iter (+/- 2.66)
    test bench_map_10     ... bench:         440.92 ns/iter (+/- 2.07)
    test bench_map_50     ... bench:         453.84 ns/iter (+/- 2.56)
    test bench_map_250    ... bench:         441.79 ns/iter (+/- 2.34)
    test bench_map_500    ... bench:         455.29 ns/iter (+/- 1.02)
    test bench_map_1000   ... bench:         454.23 ns/iter (+/- 3.98)
    test bench_vec_5      ... bench:         162.72 ns/iter (+/- 7.73)
    test bench_vec_10     ... bench:         207.77 ns/iter (+/- 5.56)
    test bench_vec_50     ... bench:         810.52 ns/iter (+/- 4.05)
    test bench_vec_250    ... bench:       4,804.87 ns/iter (+/- 44.98)
    test bench_vec_500    ... bench:       7,959.04 ns/iter (+/- 53.86)
    test bench_vec_1000   ... bench:      14,958.11 ns/iter (+/- 61.95)


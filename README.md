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

    test bench_fxmap_10   ... bench:          31 ns/iter (+/- 13)
    test bench_fxmap_50   ... bench:         167 ns/iter (+/- 3)
    test bench_fxmap_250  ... bench:         168 ns/iter (+/- 4)
    test bench_fxmap_500  ... bench:         167 ns/iter (+/- 2)
    test bench_fxmap_1000 ... bench:         167 ns/iter (+/- 4)
    test bench_map_10     ... bench:          76 ns/iter (+/- 2)
    test bench_map_50     ... bench:         392 ns/iter (+/- 10)
    test bench_map_250    ... bench:         389 ns/iter (+/- 16)
    test bench_map_500    ... bench:         389 ns/iter (+/- 10)
    test bench_map_1000   ... bench:         388 ns/iter (+/- 10)
    test bench_vec_10     ... bench:          34 ns/iter (+/- 0)
    test bench_vec_50     ... bench:         661 ns/iter (+/- 21)
    test bench_vec_250    ... bench:       2,692 ns/iter (+/- 69)
    test bench_vec_500    ... bench:       4,381 ns/iter (+/- 96)
    test bench_vec_1000   ... bench:       7,330 ns/iter (+/- 280)

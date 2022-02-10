#![feature(test)]

extern crate test;

use rustc_hash::FxHashMap;
use std::collections::HashMap;
use test::Bencher;

macro_rules! bench_vec {
    ($b:ident, $n:literal) => {
        let collection = COLLECTION[..$n].to_vec();
        let lookup_limit = $n.min(50);
        $b.iter(|| {
            LOOKUP[..lookup_limit].iter().for_each(|i| {
                test::black_box(hashmap_vs_vec::vec(i % $n, &collection));
            })
        });
    };
}

macro_rules! bench_map {
    ($b:ident, $n:literal, $map:ty, $f:path) => {
        let collection: $map = COLLECTION[..$n]
            .iter()
            .enumerate()
            .map(|(i, j)| (i as u16, *j))
            .collect();
        let lookup_limit = $n.min(50);
        $b.iter(|| {
            LOOKUP[..lookup_limit].iter().for_each(|i| {
                test::black_box($f(i % $n, &collection));
            })
        });
    };
}

#[bench]
fn bench_vec_10(b: &mut Bencher) {
    bench_vec!(b, 10);
}

#[bench]
fn bench_vec_50(b: &mut Bencher) {
    bench_vec!(b, 50);
}

#[bench]
fn bench_vec_250(b: &mut Bencher) {
    bench_vec!(b, 250);
}

#[bench]
fn bench_vec_500(b: &mut Bencher) {
    bench_vec!(b, 500);
}

#[bench]
fn bench_vec_1000(b: &mut Bencher) {
    bench_vec!(b, 1000);
}

#[bench]
fn bench_fxmap_10(b: &mut Bencher) {
    bench_map!(b, 10, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_fxmap_50(b: &mut Bencher) {
    bench_map!(b, 50, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_fxmap_250(b: &mut Bencher) {
    bench_map!(b, 250, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_fxmap_500(b: &mut Bencher) {
    bench_map!(b, 500, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_fxmap_1000(b: &mut Bencher) {
    bench_map!(b, 1000, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_map_10(b: &mut Bencher) {
    bench_map!(b, 10, HashMap<u16, u16>, hashmap_vs_vec::map);
}

#[bench]
fn bench_map_50(b: &mut Bencher) {
    bench_map!(b, 50, HashMap<u16, u16>, hashmap_vs_vec::map);
}

#[bench]
fn bench_map_250(b: &mut Bencher) {
    bench_map!(b, 250, HashMap<u16, u16>, hashmap_vs_vec::map);
}

#[bench]
fn bench_map_500(b: &mut Bencher) {
    bench_map!(b, 500, HashMap<u16, u16>, hashmap_vs_vec::map);
}

#[bench]
fn bench_map_1000(b: &mut Bencher) {
    bench_map!(b, 1000, HashMap<u16, u16>, hashmap_vs_vec::map);
}

const LOOKUP: &[u16] = &[
    380, 669, 814, 519, 432, 888, 199, 538, 623, 713, 220, 188, 570, 765, 394, 725, 359, 154, 707,
    915, 65, 463, 118, 791, 898, 715, 761, 288, 725, 785, 620, 959, 339, 580, 394, 26, 91, 739,
    633, 343, 307, 579, 189, 285, 973, 18, 938, 334, 745, 2,
];
const COLLECTION: &[u16] = &[
    991, 389, 601, 888, 494, 646, 550, 411, 135, 481, 204, 396, 704, 780, 950, 519, 586, 234, 646,
    505, 544, 28, 292, 496, 422, 210, 736, 694, 547, 986, 486, 889, 564, 437, 579, 395, 168, 136,
    368, 769, 970, 962, 856, 902, 472, 749, 540, 645, 928, 674, 18, 970, 545, 280, 937, 556, 317,
    951, 56, 435, 292, 230, 700, 336, 65, 117, 239, 142, 133, 404, 586, 381, 822, 319, 422, 601,
    125, 291, 223, 887, 240, 168, 251, 759, 550, 499, 86, 153, 456, 429, 947, 24, 254, 438, 804,
    858, 157, 861, 405, 965, 881, 426, 782, 897, 533, 558, 39, 313, 967, 607, 972, 916, 559, 391,
    690, 81, 641, 259, 952, 746, 991, 887, 905, 313, 169, 645, 243, 708, 674, 864, 929, 379, 783,
    206, 937, 58, 435, 729, 326, 346, 188, 766, 564, 352, 639, 611, 492, 199, 811, 608, 649, 239,
    455, 390, 964, 278, 456, 203, 118, 193, 280, 827, 822, 281, 753, 3, 350, 598, 27, 977, 294,
    483, 472, 728, 731, 12, 810, 9, 415, 478, 132, 510, 147, 56, 287, 660, 234, 282, 62, 627, 708,
    235, 425, 504, 800, 842, 362, 60, 703, 665, 213, 640, 426, 160, 414, 807, 657, 522, 616, 887,
    765, 377, 999, 464, 660, 501, 183, 46, 646, 120, 302, 735, 328, 390, 891, 784, 759, 803, 334,
    686, 492, 610, 898, 449, 744, 759, 597, 297, 114, 806, 450, 559, 435, 823, 269, 33, 928, 390,
    867, 978, 176, 844, 144, 823, 348, 654, 161, 105, 769, 872, 360, 291, 682, 193, 907, 605, 957,
    835, 281, 496, 822, 262, 504, 648, 605, 899, 354, 559, 690, 452, 25, 483, 641, 437, 315, 732,
    856, 407, 942, 31, 494, 288, 581, 386, 394, 401, 184, 213, 246, 497, 749, 731, 12, 809, 881,
    480, 411, 475, 118, 450, 779, 233, 806, 624, 949, 457, 525, 656, 479, 370, 378, 977, 41, 563,
    58, 817, 983, 81, 617, 251, 863, 984, 863, 763, 847, 363, 739, 174, 315, 217, 84, 671, 643,
    584, 79, 784, 41, 544, 617, 822, 769, 63, 633, 78, 991, 358, 420, 101, 359, 372, 374, 371, 265,
    868, 632, 965, 163, 843, 67, 741, 416, 132, 445, 95, 566, 466, 24, 351, 524, 605, 421, 101,
    670, 650, 784, 240, 887, 194, 871, 654, 894, 395, 745, 314, 510, 297, 517, 624, 778, 274, 218,
    337, 919, 27, 241, 77, 276, 201, 859, 186, 173, 196, 242, 424, 948, 329, 692, 173, 934, 731,
    966, 166, 65, 302, 415, 657, 973, 876, 182, 545, 207, 574, 530, 300, 444, 608, 157, 73, 657,
    223, 51, 776, 262, 211, 503, 893, 564, 72, 486, 881, 730, 634, 423, 67, 491, 698, 965, 37, 461,
    542, 208, 175, 991, 883, 576, 426, 574, 39, 426, 854, 899, 900, 268, 837, 79, 225, 811, 122,
    765, 559, 20, 163, 826, 844, 497, 150, 615, 876, 359, 33, 218, 241, 845, 621, 701, 617, 944,
    934, 253, 14, 156, 819, 699, 321, 510, 182, 950, 477, 179, 548, 229, 32, 888, 224, 41, 850,
    385, 347, 209, 932, 817, 419, 649, 962, 426, 295, 570, 138, 25, 600, 609, 838, 463, 520, 161,
    315, 699, 858, 565, 965, 282, 284, 178, 552, 448, 325, 637, 422, 498, 547, 904, 293, 124, 668,
    10, 780, 787, 978, 128, 383, 51, 489, 504, 454, 977, 617, 934, 657, 485, 704, 588, 153, 668,
    361, 973, 43, 656, 768, 116, 37, 837, 610, 229, 209, 688, 917, 560, 875, 608, 83, 496, 461,
    606, 345, 636, 683, 969, 333, 879, 603, 67, 105, 190, 820, 403, 794, 699, 686, 175, 371, 149,
    490, 459, 448, 770, 878, 511, 906, 286, 404, 165, 772, 932, 218, 573, 449, 930, 209, 634, 64,
    657, 209, 894, 799, 397, 904, 507, 509, 338, 680, 378, 871, 243, 117, 403, 219, 388, 863, 869,
    523, 563, 562, 340, 280, 718, 186, 101, 971, 330, 256, 297, 232, 452, 593, 421, 486, 959, 446,
    253, 468, 742, 297, 175, 758, 549, 304, 996, 221, 375, 377, 512, 663, 430, 349, 585, 724, 444,
    567, 20, 119, 698, 319, 956, 969, 246, 558, 536, 963, 154, 792, 590, 560, 394, 372, 266, 110,
    17, 660, 322, 135, 900, 785, 374, 537, 663, 986, 945, 161, 450, 815, 536, 801, 171, 834, 268,
    834, 634, 519, 796, 438, 457, 593, 332, 885, 91, 80, 865, 779, 872, 266, 301, 157, 382, 279,
    39, 279, 891, 919, 20, 815, 317, 258, 752, 137, 183, 394, 646, 575, 139, 31, 683, 840, 860,
    555, 450, 707, 240, 524, 406, 24, 471, 364, 677, 769, 465, 540, 906, 64, 48, 414, 250, 519,
    251, 623, 872, 197, 960, 397, 176, 779, 428, 314, 740, 636, 529, 152, 174, 597, 260, 942, 327,
    7, 16, 44, 981, 214, 106, 607, 460, 861, 260, 69, 378, 421, 525, 713, 428, 81, 378, 422, 15,
    759, 267, 986, 619, 224, 334, 583, 437, 513, 323, 498, 572, 173, 994, 946, 564, 258, 572, 401,
    436, 666, 861, 452, 202, 663, 398, 546, 117, 500, 69, 156, 77, 218, 764, 800, 426, 327, 970,
    877, 722, 838, 953, 277, 719, 687, 164, 538, 25, 108, 859, 146, 320, 587, 953, 45, 589, 200,
    961, 427, 711, 153, 768, 58, 810, 197, 467, 587, 217, 436, 43, 736, 971, 58, 443, 594, 794,
    968, 999, 589, 522, 591, 114, 496, 227, 144, 160, 202, 287, 657, 888, 431, 199, 927, 66, 776,
    216, 519, 410, 210, 334, 11, 537, 521, 900, 524, 658, 891, 804, 395, 426, 672, 861, 329, 860,
    562, 550, 791, 468, 796, 268, 310, 240, 750, 108, 808, 782, 355, 14, 302, 241, 508, 830, 383,
    178, 571, 704, 5, 322, 235, 1000, 329, 35, 485, 616, 827, 77, 249, 335, 137, 992, 758, 21, 689,
    778, 737, 663, 527, 710, 189, 938, 112, 698, 339, 49, 559, 152, 376, 162, 717, 321, 697, 116,
    565, 590, 328, 800, 257, 492, 487,
];

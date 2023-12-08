use std::{fs, ops::Range};

enum AlmanacCategory {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

struct RangeMap {
    from: Range<usize>,
    from_category: AlmanacCategory,
    to: Range<usize>,
    to_category: AlmanacCategory,
}

fn construct_range_map() -> RangeMap {
    RangeMap {
        from: 0..1,
        from_category: AlmanacCategory::Seed,
        to: 0..1,
        to_category: AlmanacCategory::Soil,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File should be readable");

    let test = RangeMap {
        from: 1..4,
        from_category: AlmanacCategory::Seed,
        to: 393..397,
        to_category: AlmanacCategory::Soil,
    };
}

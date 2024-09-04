use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn part_one(_input: &str) -> Option<u64> {
    let mut b = 0x10000;
    let mut f = 10678677;
    f += b % 256;
    f &= 0xFFFFFF;
    f *= 65899;
    f &= 0xFFFFFF;
    while b >= 256 {
        b /= 256;
        f += b % 256;
        f &= 0xFFFFFF;
        f *= 65899;
        f &= 0xFFFFFF;
    }
    Some(f)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut seen_vals = HashMap::new();
    let mut b: u64;
    let mut f: u64 = 0;
    for i in 0.. {
        b = f | 0x10000;
        f = 10678677;
        f += b % 256;
        f &= 0xFFFFFF;
        f *= 65899;
        f &= 0xFFFFFF;
        while b >= 256 {
            b /= 256;
            f += b % 256;
            f &= 0xFFFFFF;
            f *= 65899;
            f &= 0xFFFFFF;
        }
        if i == 10915 {
            return Some(f);
        }
        if let Some(old_i) = seen_vals.insert(f, i) {
            println!("Last saw val {f} at {old_i}, index now is {i}");
        }
    }
    None
}

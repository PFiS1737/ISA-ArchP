//! This module is used to test the algorithmic correctness of the high-part multiplication operations 'mulhu' and 'mulhsu'.
//!
//! For the specific hardware implementation, please refer to the [schematics](../../assets/schematics).

use rand::Rng;

#[test]
fn mulh() {
    let cases = [0, 1, -1, i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX];

    for &i in &cases {
        for &j in &cases {
            let a = i as u32;
            let b = j as u32;

            let e = mulhu32(a, b);
            let r = sim_mulhu32(a, b);
            assert_eq!(e, r, "mulhu mismatch: {} * {}", i, j);

            let e = mulhsu32(i, b);
            let r = sim_mulhsu32(i, b);
            assert_eq!(e, r, "mulhsu mismatch: {} * {}", i, j);
        }
    }

    let mut rng = rand::rng();

    for _ in 0..1_000_000 {
        let a = rng.random::<u32>();
        let b = rng.random::<u32>();

        let e = mulhu32(a, b);
        let r = sim_mulhu32(a, b);
        assert_eq!(e, r, "rand mulhu mismatch: {} * {}", a, b);

        let e = mulhsu32(a as i32, b);
        let r = sim_mulhsu32(a as i32, b);
        assert_eq!(e, r, "rand mulhsu mismatch: {} * {}", a, b);
    }
}

fn mul64(a: i64, b: i64) -> i64 {
    (a as i128 * b as i128) as i64
}

fn sim_mulhu32(a: u32, b: u32) -> u32 {
    let a64 = a as u64; // zero-extend
    let b64 = b as u64; // zero-extend

    let lo64 = mul64(a64 as i64, b64 as i64);

    (lo64 >> 32) as u32
}
fn mulhu32(a: u32, b: u32) -> u32 {
    ((a as u64 * b as u64) >> 32) as u32
}

fn sim_mulhsu32(a: i32, b: u32) -> u32 {
    let a64 = a as i64; // sign-extend
    let b64 = b as u64; // zero-extend

    let lo64 = mul64(a64, b64 as i64);

    (lo64 >> 32) as u32
}
fn mulhsu32(a: i32, b: u32) -> u32 {
    ((a as i64 * b as u64 as i64) >> 32) as u32
}

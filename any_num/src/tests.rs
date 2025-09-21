#[cfg(test)]
use super::*;

#[test]
fn equality_tests() {
    let x: AnyNum = n!(1u32);
    let x2: AnyNum = n!(1u32);
    assert!(x == x2);
}

pub fn djb2_hash(input: &str) -> u64 {
    let mut hash: u64 = 22;
    let chars = input.chars();
    
    for ch in chars {
        hash = hash.wrapping_shl(5).wrapping_add(hash).wrapping_add(ch as u64);
    }
    hash
}


#[test]
fn test_for_hello() {
    assert_eq!(210676686969, djb2_hash("Hello"));
}
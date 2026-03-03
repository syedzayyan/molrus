pub fn tanimoto(a: &[u8], b: &[u8]) -> f64 {
    let mut intersect = 0;
    let mut union = 0;
    let len = a.len().min(b.len());
    for i in 0..len {
        let and = a[i] & b[i];
        let or = a[i] | b[i];
        intersect += and.count_ones();
        union += or.count_ones();
    }
    if union == 0 {
        1.0
    } else {
        intersect as f64 / union as f64
    }
}

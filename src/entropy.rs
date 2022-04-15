pub fn process(arg: &str) -> f32 {
    let byt: &[u8] = arg.as_bytes();
    let mut histogram = [0u64; 256];

    for &b in byt {
        histogram[b as usize] += 1;
    }

    histogram
        .iter()
        .cloned()
        .filter(|&h| h != 0)
        .map(|h| h as f32 / byt.len() as f32)
        .map(|ratio| -ratio * ratio.log2())
        .sum()
}
/// Read the first 4 bits as a u32 big endian
///
///  # Panics
///
/// Panics if slice is smaller than 4 elements
pub fn read_u32_be(slice: &[u8]) -> u32 {
    as_u32_be(&pop4(&slice).unwrap())
}

// Source: https://stackoverflow.com/a/36676814/2096729
fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

fn pop4(four: &[u8]) -> Result<[u8; 4], &'static str> {
    if four.len() < 4 {
        Err("pop4: slice length less than 4")
    } else {
        Ok([four[0], four[1], four[2], four[3]])
    }
}

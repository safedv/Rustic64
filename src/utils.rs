/// Computes the DJB2 hash for the given buffer
pub fn dbj2_hash(buffer: &[u8]) -> u32 {
    let mut hsh: u32 = 5381; // DJB2 hash starts with a magic number
    let mut iter: usize = 0;
    let mut cur: u8;

    while iter < buffer.len() {
        cur = buffer[iter];

        if cur == 0 {
            iter += 1;
            continue;
        }

        if cur >= ('a' as u8) {
            cur -= 0x20;
        }

        hsh = ((hsh << 5).wrapping_add(hsh)) + cur as u32;
        iter += 1;
    }
    hsh
}

/// Calculates the length of a C-style null-terminated string.
///
/// This function counts the number of characters in the string until it encounters a null byte.
pub fn get_cstr_len(pointer: *const char) -> usize {
    let mut tmp: u64 = pointer as u64;

    // Iterate over the string until a null byte (0) is found
    unsafe {
        while *(tmp as *const u8) != 0 {
            tmp += 1;
        }
    }

    // Return the length of the string (difference between the end and start)
    (tmp - pointer as u64) as _
}

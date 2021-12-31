use boring::hash::{Hasher, MessageDigest};
use std::slice;

#[no_mangle]
pub extern "C" fn compute_sha_256(
    input_bytes: *const u8,
    input_len: usize,
    output_bytes: *mut u8,
    output_len: *mut usize,
) -> bool {
    if input_bytes.is_null() {
        return false;
    }
    let input = unsafe { slice::from_raw_parts(input_bytes, input_len) };

    if output_len.is_null() {
        return false;
    }

    let md = MessageDigest::sha256();
    let size = md.size();
    unsafe {
        if output_bytes.is_null() || *output_len < size {
            *output_len = size;
            return false;
        }
    }
    let output = unsafe { slice::from_raw_parts_mut(output_bytes, size) };

    let mut hasher = match Hasher::new(md) {
        Ok(hasher) => hasher,
        Err(_) => return false,
    };
    if hasher.update(input).is_err() {
        return false;
    }
    let result = match hasher.finish() {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    if result.len() != output.len() {
        return false;
    }

    output.copy_from_slice(&result);
    unsafe {
        *output_len = size;
    }
    true
}

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut data = data;
    fuzzing::decode_variants(&mut data);
});

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = webpki::trust_anchor_util::cert_der_as_trust_anchor(data);
});

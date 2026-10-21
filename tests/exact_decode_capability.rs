use truehd_atmos::*;
use truehd_atmos::types::*;

#[test]
fn exact_decode_fails_closed_when_not_compiled() {
    let handle = truehd_atmos_decoder_create(-1);
    assert!(!handle.is_null());
    let status = unsafe { truehd_atmos_decoder_set_ignore_interpolation(handle, false) };
    let expected = if cfg!(feature = "matrix-interpolation") { OK } else { ERR_INVALID_ARGUMENT };
    assert_eq!(status, expected);
    unsafe { truehd_atmos_decoder_destroy(handle) };
}

#[test]
fn approximate_decode_remains_an_explicit_option() {
    let handle = truehd_atmos_decoder_create(-1);
    assert!(!handle.is_null());
    assert_eq!(unsafe { truehd_atmos_decoder_set_ignore_interpolation(handle, true) }, OK);
    unsafe { truehd_atmos_decoder_destroy(handle) };
}

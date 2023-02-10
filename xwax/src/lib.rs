use std::ffi::CString;
use std::mem::MaybeUninit;


#[cfg(test)]
mod tests {
    use core::time;

    use super::*;

    #[test]
    fn test_init() -> Result<(), Box<dyn std::error::Error>> {
        let mut timecoder = MaybeUninit::<xwax_sys::timecoder>::uninit();
        let timecoder_name = CString::new("serato_2a")?;
        let timecoder_def = unsafe { xwax_sys::timecoder_find_definition(timecoder_name.as_ptr()) };
        unsafe {
            xwax_sys::timecoder_init(timecoder.as_mut_ptr(), timecoder_def, 1.0, 96000, true);
        }
        let timecoder = unsafe { timecoder.assume_init() };
        assert_eq!(timecoder.def, timecoder_def);
        Ok(())
    }
}
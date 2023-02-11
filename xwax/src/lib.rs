use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(thiserror::Error, Debug)]
pub enum XwaxError {}

pub struct Timecoder {
    tc: xwax_sys::timecoder,
}

unsafe impl Send for Timecoder {}

impl Timecoder {
    pub fn init() -> Result<Self, XwaxError> {
        let mut tc = MaybeUninit::<xwax_sys::timecoder>::uninit();
        let timecoder_name = CString::new("serato_cd").unwrap();
        let timecoder_def = unsafe { xwax_sys::timecoder_find_definition(timecoder_name.as_ptr()) };
        unsafe {
            xwax_sys::timecoder_init(tc.as_mut_ptr(), timecoder_def, 1.0, 44100, false);
        }
        Ok(Timecoder {
            tc: unsafe { tc.assume_init() },
        })
    }

    pub fn submit(&mut self, samples: &[i16]) {
        unsafe {
            xwax_sys::timecoder_submit(&mut self.tc, samples.as_ptr() as *mut i16, samples.len());
        }
    }

    pub fn get_pitch(&self) -> f64 {
        unsafe {
            xwax_sys::_timecoder_get_pitch(
                &self.tc as *const xwax_sys::timecoder as *mut xwax_sys::timecoder,
            )
        }
    }

    pub fn get_position(&self) -> Option<i32> {
        match unsafe {
            xwax_sys::timecoder_get_position(
                &self.tc as *const xwax_sys::timecoder as *mut xwax_sys::timecoder,
                std::ptr::null_mut(),
            )
        } {
            -1 => None,
            position => Some(position),
        }
    }
}

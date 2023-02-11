use std::mem::MaybeUninit;
use std::time::Duration;

fn lfsr(code: u32, taps: u32) -> u32 {
    let mut taken = code & taps;
    let mut xrs = 0;

    while taken != 0 {
        xrs += taken & 1;
        taken >>= 1;
    }

    xrs & 1
}

fn fwd(current: u32, bits: i32, taps: u32) -> u32 {
    let l = lfsr(current, taps | 1);
    (current >> 1) | (l << (bits - 1))
}

fn rev(current: u32, bits: i32, taps: u32) -> u32 {
    let mask = (1 << bits) - 1;
    let l = lfsr(current, (taps >> 1) | (1 << (bits - 1)));
    ((current << 1) & mask) | l
}

#[derive(thiserror::Error, Debug)]
pub enum XwaxError {
    #[error("Failed to initialized timecode LUT.")]
    LUTInitError,
    #[error("LUT wrapped during building.")]
    LUTWrapError,
    #[error("LUT was not symmetric.")]
    LUTSymmetryError,
}

#[derive(Debug)]
pub struct Postition {
    pub position: i32,
    pub elapsed: Duration,
}

#[repr(transparent)]
pub struct TimecoderDef {
    def: xwax_sys::timecode_def,
}

unsafe impl Send for TimecoderDef {}

impl TimecoderDef {
    pub fn new(
        bits: i32,
        resolution: i32,
        seed: u32,
        taps: u32,
        length: u32,
        safe: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lut = MaybeUninit::<xwax_sys::lut>::uninit();
        if unsafe { xwax_sys::lut_init(lut.as_mut_ptr(), length as i32) } == -1 {
            return Err(XwaxError::LUTInitError.into());
        }
        let mut lut = unsafe { lut.assume_init() };

        let mut current = seed;

        for _ in 0..length {
            if unsafe { xwax_sys::lut_lookup(&mut lut, current) } != u32::MAX {
                return Err(XwaxError::LUTWrapError.into());
            }
            unsafe {
                xwax_sys::lut_push(&mut lut, current);
            }
            let next = fwd(current, bits, taps);
            if rev(next, bits, taps) != current {
                return Err(XwaxError::LUTSymmetryError.into());
            }
            current = next;
        }

        Ok(TimecoderDef {
            def: xwax_sys::timecode_def {
                name: std::ptr::null(),
                desc: std::ptr::null(),
                bits,
                resolution,
                flags: 0,
                seed,
                taps,
                length,
                safe,
                lookup: true,
                lut,
            },
        })
    }

    pub fn switch_phase(self) -> Self {
        TimecoderDef {
            def: xwax_sys::timecode_def {
                flags: self.def.flags | 0b001,
                ..self.def
            },
        }
    }

    pub fn switch_primary(self) -> Self {
        TimecoderDef {
            def: xwax_sys::timecode_def {
                flags: self.def.flags | 0b010,
                ..self.def
            },
        }
    }

    pub fn switch_polarity(self) -> Self {
        TimecoderDef {
            def: xwax_sys::timecode_def {
                flags: self.def.flags | 0b100,
                ..self.def
            },
        }
    }
}

pub struct Timecoder {
    tc: xwax_sys::timecoder,
}

unsafe impl Send for Timecoder {}

impl Timecoder {
    pub fn init(mut def: TimecoderDef, speed: f64, sample_rate: u32, phono: bool) -> Self {
        let mut tc = MaybeUninit::<xwax_sys::timecoder>::uninit();

        unsafe {
            xwax_sys::timecoder_init(
                tc.as_mut_ptr(),
                &mut def as *mut TimecoderDef as *mut xwax_sys::timecode_def,
                speed,
                sample_rate,
                phono,
            );
        }
        Timecoder {
            tc: unsafe { tc.assume_init() },
        }
    }

    pub fn submit(&mut self, samples: &[i16]) {
        unsafe {
            xwax_sys::timecoder_submit(
                &mut self.tc,
                samples.as_ptr() as *mut i16,
                samples.len() / 2,
            );
        }
    }

    pub fn get_pitch(&self) -> f64 {
        unsafe {
            xwax_sys::_timecoder_get_pitch(
                &self.tc as *const xwax_sys::timecoder as *mut xwax_sys::timecoder,
            )
        }
    }

    pub fn get_position(&self) -> Option<Postition> {
        let mut elapsed = 0.0;
        match unsafe {
            xwax_sys::timecoder_get_position(
                &self.tc as *const xwax_sys::timecoder as *mut xwax_sys::timecoder,
                &mut elapsed,
            )
        } {
            -1 => None,
            position => Some(Postition {
                position,
                elapsed: Duration::from_secs_f64(elapsed),
            }),
        }
    }
}

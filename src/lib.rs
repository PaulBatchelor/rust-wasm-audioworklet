use std::f32::consts::PI;

#[repr(C)]
pub struct PhaseDistortionOsc {
   phs: f32,
   freq: f32,
   sr: u32,

   // magic circle LFO
   // https://pbat.ch/sndkit/chorus/
   // https://ccrma.stanford.edu/~jos/pasp/Digital_Sinusoid_Generators.html
   mc_x1: f32,
   mc_x2: f32,
   mc_eps: f32,
}

fn mtof(nn: f32) -> f32 {
    let freq = (2.0_f32).powf((nn - 69.0) / 12.0) * 440.0;
    freq
}

impl PhaseDistortionOsc {
    pub fn new(sr: u32) -> PhaseDistortionOsc {
        PhaseDistortionOsc {
            phs: 0.0,
            freq: mtof(48.0),
            sr: sr,
            mc_x1: 1.0,
            mc_x2: 0.0,
            mc_eps: 2.0 * (PI * (0.4 / sr as f32)).sin(),
        }
    }

    pub fn phasewarp(&self, phs: f32, warp: f32) -> f32 {
        let wmp = (warp + 1.0) * 0.5;

        if phs < wmp {
            if wmp != 0.0 {
                return (0.5 / wmp) * phs;
            }
        } else {
            if wmp != 1.0 {
                return (0.5 / (1.0 - wmp)) * (phs - wmp) + 0.5;
            }
        }

        return 0.0;
    }

    pub fn tick(&mut self) -> f32 {
        // compute LFO
        self.mc_x1 = self.mc_x1 + self.mc_eps*self.mc_x2;
        self.mc_x2 = -self.mc_eps*self.mc_x1 + self.mc_x2;

        let lfo = (self.mc_x2 + 1.0)*0.5;
        let lfo = 0.01 + 0.5*lfo;

        let out = (self.phasewarp(self.phs, lfo) * 2.0 * PI).sin();
        self.phs += self.freq / self.sr as f32;
        if self.phs > 1.0 {
            self.phs -= 1.0;
        }

        if self.phs < 0.0 {
            self.phs += 1.0;
        }
        out * 0.5
    }

    pub fn process(&mut self, outbuf: *mut f32, sz: usize) {

        let outbuf: &mut [f32] = unsafe {
            std::slice::from_raw_parts_mut(outbuf, sz)
        };

        for n in 0..sz {
            outbuf[n] = self.tick();
        }

    }

}

#[no_mangle]
pub extern "C" fn pdosc_new(sr: u32) -> Box<PhaseDistortionOsc> {
    Box::new(PhaseDistortionOsc::new(sr))
}

#[no_mangle]
pub extern "C" fn pdosc_tick(pdo: &mut PhaseDistortionOsc) -> f32 {
    pdo.tick()
}

#[no_mangle]
pub extern "C" fn pdosc_process(pdo: &mut PhaseDistortionOsc, outbuf: *mut f32, sz: usize) {
    pdo.process(outbuf, sz);
}

#[no_mangle]
pub extern "C" fn pdosc_del(_: Option<Box<PhaseDistortionOsc>>) {

}

// adapted from Glicol
// https://github.com/chaosprint/glicol/blob/7ece81d6fadfc5a8873df2a3ac04f8f915fa1998/rs/wasm/src/lib.rs#L9-L15
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

use boingboingboing as boing;

#[derive(Clone, Copy)]
pub struct Voice {
    blsaw: boing::blep::BLEP,
    lpf: boing::butterworth::ButterworthLowPass,
    lfo: boing::magic_circle::MagicCircleSine,
    phs: f32,
    lfoval: f32,
}

#[repr(C)]
pub struct IsoRhythms {
    voices: [Voice; 6],
    bigverb: boing::bigverb::BigVerb,
}

impl Voice {
    pub fn new(sr: usize) -> Self {
        Voice {
            blsaw: boing::blep(sr),
            lpf: boing::butlp(sr),
            lfo: boing::mcsine(sr),
            phs: 0.0,
            lfoval: 0.0,
        }
    }

    pub fn pitch(&mut self, nn: f32) {
        self.blsaw.set_freq(mtof(nn));
    }

    pub fn rate(&mut self, freq: f32) {
        self.lfo.set_freq(freq);
    }

    pub fn phase(&mut self, phs: f32) {
        self.phs = phs;
    }

    pub fn init(&mut self) {
        self.pitch(60.0);
        self.rate(0.3);
    }

    pub fn tick(&mut self) -> f32 {
        let smp = self.blsaw.saw();
        let s = (1.0 + self.lfo.tick()) * 0.5;
        self.lfoval = s;
        self.lpf.set_freq(100.0 + 400.0 * s);
        let smp = self.lpf.tick(smp);
        return smp * 0.3 * s;
    }
}

impl IsoRhythms {
    pub fn new(sr: usize) -> Self {
        IsoRhythms {
            voices: [Voice::new(sr); 6],
            bigverb: boing::bigverb(sr),
        }
    }

    pub fn setup(&mut self) {
        let base = 60.0 - 4.0;
        let voices = &mut self.voices;

        voices[0].init();
        voices[0].pitch(base + 12.0);
        voices[0].rate(1.0 / 10.0);
        voices[0].phase(0.0);

        voices[1].init();
        voices[1].pitch(base + 11.0);
        voices[1].rate(1.0 / 9.0);
        voices[1].phase(0.1);

        voices[2].init();
        voices[2].pitch(base + 7.0);
        voices[2].rate(1.0 / 7.0);
        voices[2].phase(0.2);

        voices[3].init();
        voices[3].pitch(base);
        voices[3].rate(1.0 / 6.0);
        voices[3].phase(0.3);

        voices[4].init();
        voices[4].pitch(base + 2.0);
        voices[4].rate(1.0 / 5.0);

        voices[5].init();
        voices[5].pitch(base + 4.0);
        voices[5].rate(1.0 / 4.0);

        self.bigverb.init();
    }

    pub fn tick(&mut self) -> f32 {
        let mut out = 0.0;
        for v in 0..6 {
            out += self.voices[v].tick();
        }
        let (rvb, _) = self.bigverb.tick(out, out);
        out + rvb * 0.2
    }

    pub fn process(&mut self, outbuf: *mut f32, sz: usize) {
        let outbuf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(outbuf, sz) };

        for n in 0..sz {
            outbuf[n] = self.tick();
        }
    }

    pub fn cvparams(&mut self, outbuf: *mut f32, sz: usize) {
        let outbuf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(outbuf, sz) };

        for n in 0..6 {
            outbuf[n] = self.voices[n].lfoval;
        }
    }
}

fn mtof(nn: f32) -> f32 {
    let freq = (2.0_f32).powf((nn - 69.0) / 12.0) * 440.0;
    freq
}

#[no_mangle]
pub extern "C" fn isorhythms_new(sr: usize) -> Box<IsoRhythms> {
    Box::new(IsoRhythms::new(sr))
}

#[no_mangle]
pub extern "C" fn isorhythms_tick(ir: &mut IsoRhythms) -> f32 {
    ir.tick()
}

#[no_mangle]
pub extern "C" fn isorhythms_setup(ir: &mut IsoRhythms) {
    ir.setup()
}

#[no_mangle]
pub extern "C" fn isorhythms_process(ir: &mut IsoRhythms, outbuf: *mut f32, sz: usize) {
    ir.process(outbuf, sz);
}

#[no_mangle]
pub extern "C" fn isorhythms_cvparams(ir: &mut IsoRhythms, outbuf: *mut f32, sz: usize) {
    ir.cvparams(outbuf, sz);
}

#[no_mangle]
pub extern "C" fn isorhythms_del(_: Option<Box<IsoRhythms>>) {}

// adapted from Glicol
// https://github.com/chaosprint/glicol/blob/7ece81d6fadfc5a8873df2a3ac04f8f915fa1998/rs/wasm/src/lib.rs#L9-L15
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

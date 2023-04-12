class PhaseDistortionOscillator extends AudioWorkletProcessor {
    constructor(options) {
        super(options)
        const wasmBytes = options.processorOptions.wasmBytes;
        const mod = new WebAssembly.Module(wasmBytes);
        this.wasm = new WebAssembly.Instance(mod, {});
        //this.dsp = this.wasm.exports.pdosc_new(sampleRate);
        this.dsp = this.wasm.exports.isorhythms_new(sampleRate);
        this.wasm.exports.isorhythms_setup(this.dsp);
        this.outptr = this.wasm.exports.alloc(128);
        this.outbuf = new Float32Array(this.wasm.exports.memory.buffer,
                this.outptr,
                128);
    }

    process(inputs, outputs, parameters) {
        const output = outputs[0];
        //this.wasm.exports.pdosc_process(this.dsp, this.outptr, 128);
        this.wasm.exports.isorhythms_process(this.dsp, this.outptr, 128);
        for (let channel = 0; channel < output.length; ++channel) {
            const outputChannel = output[channel];
            for (let i = 0; i < outputChannel.length; ++i) {
                outputChannel[i] = this.outbuf[i];
            }
        }

        return true;
    }
}

registerProcessor('pdosc', PhaseDistortionOscillator);

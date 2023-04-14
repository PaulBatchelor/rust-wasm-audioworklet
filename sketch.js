const audioContext = new AudioContext();
let started = false;
let width = 300;
let height = 400;
let IsoNode = null;

function setup() {
    createCanvas(width, height).center();
}

function get_lfo(x, y) {
    if (IsoNode != null && IsoNode.cv != null) {
        return IsoNode.cv[y * 2 + x];
    }

    return 0.0;
}

function draw() {
    background(0, 128, 128);
    let gridx = width / 2.0;
    let gridy = height / 3.0;
    let radius = gridx * 0.7;
    fill(255, 255, 255);
    noStroke();
    if (IsoNode != null) {
        IsoNode.poll_cvparams();
    }
    for (let y = 0; y < 3; y++) {
        for (let x = 0; x < 2; x++) {
            let csize = 0.1 * radius + 0.9*radius * get_lfo(x, y);
            circle(gridx*0.5 + x*gridx,
                gridy*0.5 + y*gridy, csize);
        }
    }
}

function mouseClicked() {
    if (started === false) {
        audioContext.resume();
        started = true;
    }
}

class IsoRhythmsNode extends AudioWorkletNode {
    constructor(context, name, options) {
        super(context, name, options);
        this.test = 42;
        this.cv = null;
        this.port.onmessage = (event) => this.onmessage(event.data);
    }

    onmessage(event) {
        if (event.type === "cv-response") {
            this.cv = event.cv;
        }
    }

    poll_cvparams() {
        this.port.postMessage({type: "cv-get"});
    }
}

window.addEventListener('load', async () => {
    let context = audioContext;
    try {
        await context.audioWorklet.addModule('isorhythms.js');
    } catch(e) {
        throw new Error(`noise generator error: ${e.message}`);
    }

    const wasmFile = await fetch('dsp.wasm');
    const wasmBuffer = await wasmFile.arrayBuffer();

    const options = {
        wasmBytes: wasmBuffer
    };
    const irnode = new
        IsoRhythmsNode(context, 'isorhythms', {
        processorOptions: options
    });

    irnode.connect(context.destination);
    IsoNode = irnode;
})

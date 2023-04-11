const audioContext = new AudioContext();

const startAudio = async (context) => {
  try {
      await context.audioWorklet.addModule('pdosc.js');
  } catch(e) {
    throw new Error(`noise generator error: ${e.message}`);
  }

  const wasmFile = await fetch('dsp.wasm');
  const wasmBuffer = await wasmFile.arrayBuffer();

  const options = {
        wasmBytes: wasmBuffer
  };
  const PDOsc = new
        AudioWorkletNode(context, 'pdosc', {
            processorOptions: options
        });

  PDOsc.connect(context.destination);

};

window.addEventListener('load', async () => {
  const buttonStart = document.getElementById('button-start');
  const buttonStop = document.getElementById('button-stop');
  const buttonBegin = document.getElementById('button-begin');
  console.log(audioContext.state);
  buttonBegin.disabled = false;
  buttonStart.disabled = true;
  buttonBegin.addEventListener('click', async () => {
      await startAudio(audioContext);
      audioContext.resume();
      buttonBegin.disabled = true;
      buttonBegin.textContent = 'Playing...';
      buttonStart.disabled = true;
      buttonStop.disabled = false;
  }, false);
  buttonStop.addEventListener('click', async () => {
      audioContext.suspend();
      buttonStart.disabled = false;
      buttonStop.disabled = true;
  }, false);

  buttonStart.addEventListener('click', async () => {
      audioContext.resume();
      buttonStart.disabled = true;
      buttonStop.disabled = false;
  }, false);
});


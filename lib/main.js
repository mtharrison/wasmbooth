import FpsIndicator from './fps';
import * as Utils from './utils';

const WIDTH = 400;
const HEIGHT = 400;

const elements = {
    canvas: {
        video: document.getElementById('video'),
        hidden: document.createElement('canvas'),
        visible: document.getElementById('canvas'),
        capture: document.getElementById('capture'),
        template: document.getElementById('template'),
        ctx: {}
    },
    audio: {
        shutter: document.getElementById('shutter')
    },
    downloadBtn: document.getElementById('btn-download'),
    loading: document.getElementById('loading'),
    fps: document.getElementById('fps'),
    options: [
        document.getElementById('mirror_x_on'),
        document.getElementById('mirror_y_on'),
        document.getElementById('grayscale_on'),
        document.getElementById('outline_on'),
        document.getElementById('sharpen_on'),
        document.getElementById('invert_on'),
        document.getElementById('blur_on'),
        document.getElementById('emboss_on'),
    ]
};

// Hidden canvas for rendering video directly onto

elements.canvas.hidden.width = WIDTH;
elements.canvas.hidden.height = HEIGHT;
elements.canvas.ctx.hidden = elements.canvas.hidden.getContext('2d');

// Target canvas for rendering effects to

elements.canvas.ctx.visible = elements.canvas.visible.getContext('2d');

// Setup target canvas for capture

elements.canvas.ctx.capture = elements.canvas.capture.getContext('2d');
elements.canvas.ctx.capture.drawImage(elements.canvas.template, 0, 0);

// Hook up download button

elements.downloadBtn.addEventListener('click', function (e) {

    let countdown = 3;
    elements.loading.innerText = '';

    const tick = () => {

        if (countdown === 0) {
            sounds.shutter();
            setImmediate(() => {

                elements.downloadBtn.innerText = 'CAPTURE';
                elements.canvas.visible.classList.add('flash');
                elements.canvas.ctx.capture.drawImage(elements.canvas.visible, 25, 25);
                var dataURL = elements.canvas.capture.toDataURL('image/png');
                const a = document.createElement('a');
                a.href = dataURL;
                a.download = "wasmbooth.png";
                setTimeout(() => elements.canvas.visible.classList.remove('flash'), 1000);

                var event = new MouseEvent('click');
                a.dispatchEvent(event);
            });

            return;
        }

        sounds.beep();
        elements.downloadBtn.innerText = '... ' + countdown.toString() + ' ...';

        setTimeout(() => {

            countdown--;
            tick();

        }, 1000);
    };

    tick();

    e.preventDefault();
});

const getOptions = () => {

    // Create a bitset of options, an efficient way to pass options to wasm
    // invert:   0b00000001
    // mirror_y: 0b00000100
    //
    // options:  0b00000101 === 5

    let options = 0;

    for (let [i, el] of elements.options.entries()) {
        options |= el.checked ? (1 << i) : 0;
    }

    return options;
};

// Start to capture webcam

navigator.mediaDevices.getUserMedia({ video: true, audio: false })
    .then(function (stream) {

        elements.canvas.video.srcObject = stream;
    })
    .catch(function (err) {

        throw err;
    });

// Create a beep sound for countdown

window.sounds = (() => {
    var AudioContext = window.AudioContext // Default
        || window.webkitAudioContext       // Safari and old versions of Chrome
        || false;

    if (AudioContext) {
        var context = new AudioContext();

        // Beep sound

        const beep = (() => {

            const freq = 830.6;
            const type = 'sine';
            const osc = context.createOscillator();
            const gain = context.createGain();
            gain.gain.value = 0;
            osc.type = type;
            osc.connect(gain);
            osc.frequency.value = freq;
            gain.connect(context.destination);
            osc.start(0);

            return () => {
                gain.gain.value = 0.1;
                setTimeout(() => gain.gain.value = 0, 150);
            }
        })();

        const shutter = () => {

            elements.audio.shutter.play();
        };

        return { beep, shutter };
    }

    return { beep: () => { }, shutter: () => { } };
})();


fetch('wasmbooth.wasm').then((response) =>
    response.arrayBuffer()
).then((bytes) =>
    WebAssembly.instantiate(bytes)      // support safari
).then(({ instance }) => {

    // Allocate enough space for pixel data in the wasm linear memory
    // and get a pointer to the start of the data

    const offset = instance.exports.alloc_pixels(WIDTH * HEIGHT);

    // Setup FPS indicator

    const fps = new FpsIndicator(250, elements.fps);

    const render = () => {

        fps.tick();

        // Draw video frame to hidden canvas

        elements.canvas.ctx.hidden.drawImage(elements.canvas.video, (elements.canvas.video.videoWidth - WIDTH) / 2, (elements.canvas.video.videoHeight - HEIGHT) / 2, WIDTH, HEIGHT, 0, 0, WIDTH, HEIGHT);

        // Get the image data from the hidden canvas

        const imageData = elements.canvas.ctx.hidden.getImageData(0, 0, WIDTH, HEIGHT);

        // Copy image data into wasm linear memory

        Utils.memcopy(imageData.data.buffer, instance.exports.memory.buffer, offset);

        // Call into wasm to apply filters to image data

        instance.exports.apply_filters(offset, getOptions(), WIDTH, HEIGHT);

        // Get view onto wasm memory

        const data = new Uint8ClampedArray(instance.exports.memory.buffer, offset, WIDTH * HEIGHT * 4);

        // Create new image data object

        const imageDataUpdated = new ImageData(data, WIDTH, HEIGHT);

        // Write image data back to canvas

        elements.canvas.ctx.visible.putImageData(imageDataUpdated, 0, 0);

        // Queue another frame

        requestAnimationFrame(render);
    }

    render();
});
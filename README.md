![WASMBOOTH](https://raw.githubusercontent.com/mtharrison/wasmbooth/master/public/logo.png)

## Video effect booth written in Rust and WebAssembly

Play with it here: https://mtharrison.github.io/wasmbooth/

### Aim

I wrote this purely to teach myself more about both Rust and WebAssembly and how to use the two together. The aim of this is definitely _not_ to show off the performance of wasm. I haven't benchmarked or compared this to a pure JS implementation but I wouldn't be surprised if it were slower because it copies all the ImageData from canvas into the wasm linear memory on every frame. Additionally it uses convolutional image processing for a few of the effects, which aren't the most efficient algorithms but are elegant and easy to write/understand.

### How it works

The front end is usual HTML, CSS, JS. It streams your webcam into an offscreen video element, which is then written to a hidden canvas. On each frame we grab the image data from the canvas and write it into WebAssembly's linear memory at a pre-determined offset. We then call a WebAssembly function that will process those pixels with our chosen filters. Finally, we construct a new ImageData object and put it on a visible canvas.

To capture a still, we write the visible canvas data into a premade template.

The wasm module exposes 2 functions to JavaScript. One tells the module to allocate enough space to hold all our pixel data and returns a pointer, which is a simple integer offset in the wasm linear memory. The other function takes that pointer and the dimensions of the image, along with our chosen filters.

- `lib` - Contains the frontend JS which will be bundled into public/bundle.js by webpack
- `public` - Everything that will be served up to the browser including compiled wasm module
- `src` - The Rust source code which will be compiled to wasm

### Usage

To simply use the app, run the following:

- `npm install --production` to install hapi (to serve the site)
- `npm start` to start a server

Then browse to `http://localhost:4000`

If you want to change JS inside lib, you should run:

- `npm install` to webpack
- `npm run build-js` after to bundle the JS again

If you want to change Rust, you should run:

- `npm run build-wasm` to recompile the .wasm module. You will need nighty Rust and the wasm target installed for this. There's a [good explanation here](https://rust-lang-nursery.github.io/rust-wasm/setup.html)

There are some Rust tests, to run them run:

- `npm test` or `cargo test`

### Contributing

PRs welcome to improve the code or approach or to add more effects, this is all about learning! I'm a newbie to both Rust and wasm so please open an issue if you think there's something I missed or could have done better.

/******/ (function (modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if (installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
      /******/
    }
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
      /******/
    };
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
    /******/
  }
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function (exports, name, getter) {
/******/ 		if (!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, {
/******/ 				configurable: false,
/******/ 				enumerable: true,
/******/ 				get: getter
        /******/
      });
      /******/
    }
    /******/
  };
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function (exports) {
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
    /******/
  };
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function (module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
    /******/
  };
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function (object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./lib/main.js");
  /******/
})
/************************************************************************/
/******/({

/***/ "./lib/fps.js":
/*!********************!*\
  !*** ./lib/fps.js ***!
  \********************/
/*! exports provided: default */
/***/ (function (module, __webpack_exports__, __webpack_require__) {

        "use strict";
        eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"default\", function() { return Averager; });\nclass Averager {\n    constructor(interval, element) {\n        this.lastTick = performance.now();\n        this.lastNotify = this.lastTick;\n        this.interval = interval;\n        this.element = element;\n        this.runningSum = 0;\n        this.runningSamples = 0;\n    }\n\n    tick() {\n        const now = performance.now();\n        this.runningSum += (now - this.lastTick);\n        this.runningSamples++;\n        this.lastTick = now;\n\n        if ((now - this.lastNotify) > this.interval) {\n            this.notify(now);\n        }\n    }\n\n    notify(now) {\n        const avgFrame = this.runningSum / this.runningSamples;\n        const fps = 1000 / avgFrame;\n        this.element.innerText = `${fps.toFixed(2)}fps`;\n\n        this.lastNotify = now;\n        this.runningSamples = 0;\n        this.runningSum = 0;\n    }\n}\n\n//# sourceURL=webpack:///./lib/fps.js?");

        /***/
      }),

  /***/ "./lib/main.js":
  /*!*********************!*\
    !*** ./lib/main.js ***!
    \*********************/
  /*! no exports provided */
  /***/ (function (module, __webpack_exports__, __webpack_require__) {

        "use strict";
        eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _fps__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./fps */ \"./lib/fps.js\");\n/* harmony import */ var _utils__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./utils */ \"./lib/utils.js\");\n\n\n\nconst WIDTH = 400;\nconst HEIGHT = 400;\n\nconst elements = {\n    canvas: {\n        video: document.getElementById('video'),\n        hidden: document.createElement('canvas'),\n        visible: document.getElementById('canvas'),\n        capture: document.getElementById('capture'),\n        template: document.getElementById('template'),\n        ctx: {}\n    },\n    downloadBtn: document.getElementById('btn-download'),\n    fps: document.getElementById('fps'),\n    options: [\n        document.getElementById('mirror_x_on'),\n        document.getElementById('mirror_y_on'),\n        document.getElementById('grayscale_on'),\n        document.getElementById('outline_on'),\n        document.getElementById('sharpen_on'),\n        document.getElementById('invert_on'),\n        document.getElementById('blur_on'),\n        document.getElementById('emboss_on'),\n    ]\n};\n\n// Hidden canvas for rendering video directly onto\n\nelements.canvas.hidden.width = WIDTH;\nelements.canvas.hidden.height = HEIGHT;\nelements.canvas.ctx.hidden = elements.canvas.hidden.getContext('2d');\n\n// Target canvas for rendering effects to\n\nelements.canvas.ctx.visible = elements.canvas.visible.getContext('2d');\n\n// Setup target canvas for capture\nelements.canvas.ctx.capture = elements.canvas.capture.getContext('2d');\nelements.canvas.ctx.capture.drawImage(elements.canvas.template, 0, 0);\n\n// Hook up download button\n\nelements.downloadBtn.addEventListener('click', function (e) {\n\n    let countdown = 3;\n\n    const tick = () => {\n\n        if (countdown === 0) {\n            elements.downloadBtn.innerText = 'CAPTURE';\n            elements.canvas.visible.classList.add('flash');\n            elements.canvas.ctx.capture.drawImage(elements.canvas.visible, 25, 25);\n            var dataURL = elements.canvas.capture.toDataURL('image/png');\n            const a = document.createElement('a');\n            a.href = dataURL;\n            a.download = \"wasmbooth.png\";\n            setTimeout(() => elements.canvas.visible.classList.remove('flash'));\n\n            var event = new MouseEvent('click');\n            a.dispatchEvent(event);\n\n            return;\n        }\n\n        elements.downloadBtn.innerText = countdown.toString();\n\n        setTimeout(() => {\n\n            countdown--;\n            tick();\n\n        }, 1000);\n    };\n\n    tick();\n\n    e.preventDefault();\n});\n\nconst getOptions = () => {\n\n    // Create a bitset of options, an efficient way to pass options to wasm\n    // invert:   0b00000001\n    // mirror_y: 0b00000100\n    //\n    // options:  0b00000101 === 5\n\n    let options = 0;\n\n    for (let [i, el] of elements.options.entries()) {\n        options |= el.checked ? (1 << i) : 0;\n    }\n\n    return options;\n};\n\n// Start to capture webcam\n\nnavigator.mediaDevices.getUserMedia({ video: true, audio: false })\n    .then(function (stream) {\n\n        elements.canvas.video.srcObject = stream;\n    })\n    .catch(function (err) {\n\n        throw err;\n    });\n\n\nfetch('wasm_video_filters.wasm').then((response) =>\n    response.arrayBuffer()\n).then((bytes) =>\n    WebAssembly.instantiate(bytes)\n).then(({ instance }) => {\n\n    // Allocate enough space for pixel data in the wasm linear memory\n    // and get a pointer to the start of the data\n\n    const offset = instance.exports.alloc_pixels(WIDTH * HEIGHT);\n\n    // Setup FPS indicator\n\n    const fps = new _fps__WEBPACK_IMPORTED_MODULE_0__[\"default\"](250, elements.fps);\n\n    const render = () => {\n\n        fps.tick();\n\n        // Draw video frame to hidden canvas\n\n        elements.canvas.ctx.hidden.drawImage(elements.canvas.video, -((elements.canvas.video.videoWidth - 400) / 2), -((elements.canvas.video.videoHeight - 400) / 2));\n\n        // Get the image data from the hidden canvas\n\n        const imageData = elements.canvas.ctx.hidden.getImageData(0, 0, WIDTH, HEIGHT);\n\n        // Copy image data into wasm linear memory\n\n        _utils__WEBPACK_IMPORTED_MODULE_1__[\"memcopy\"](imageData.data.buffer, instance.exports.memory.buffer, offset);\n\n        // Call into wasm to apply filters to image data\n\n        instance.exports.apply_filters(offset, getOptions(), WIDTH, HEIGHT);\n\n        // Get view onto wasm memory\n\n        const data = new Uint8ClampedArray(instance.exports.memory.buffer, offset, WIDTH * HEIGHT * 4);\n\n        // Create new image data object\n\n        const imageDataUpdated = new ImageData(data, WIDTH, HEIGHT);\n\n        // Write image data back to canvas\n\n        elements.canvas.ctx.visible.putImageData(imageDataUpdated, 0, 0);\n\n        // Queue another frame\n\n        requestAnimationFrame(render);\n    }\n\n    render();\n});\n\n//# sourceURL=webpack:///./lib/main.js?");

        /***/
      }),

  /***/ "./lib/utils.js":
  /*!**********************!*\
    !*** ./lib/utils.js ***!
    \**********************/
  /*! exports provided: memcopy */
  /***/ (function (module, __webpack_exports__, __webpack_require__) {

        "use strict";
        eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"memcopy\", function() { return memcopy; });\nfunction memcopy(b1, b2, offset) {\n    new Uint8Array(b2, offset, b1.byteLength).set(new Uint8Array(b1));\n};\n\n//# sourceURL=webpack:///./lib/utils.js?");

        /***/
      })

    /******/
  });
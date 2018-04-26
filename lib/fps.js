export default class Fps {
    constructor(interval, element) {
        this.lastTick = performance.now();
        this.lastNotify = this.lastTick;
        this.interval = interval;
        this.element = element;
        this.runningSum = 0;
        this.runningSamples = 0;
    }

    tick() {
        const now = performance.now();
        this.runningSum += (now - this.lastTick);
        this.runningSamples++;
        this.lastTick = now;

        if ((now - this.lastNotify) > this.interval) {
            this.notify(now);
        }
    }

    notify(now) {
        const avgFrame = this.runningSum / this.runningSamples;
        const fps = 1000 / avgFrame;
        this.element.innerText = `${fps.toFixed(2)}fps`;
        this.lastNotify = now;
        this.runningSamples = 0;
        this.runningSum = 0;
    }
}
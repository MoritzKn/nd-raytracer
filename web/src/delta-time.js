const TARGET_FPS = 20;

export class DeltaTime {
  constructor() {
    this.frameStart = null;
    this.dt = 1000 / TARGET_FPS;

    this.fps = TARGET_FPS;
    this.frameCounter = 0;
    this.secondStart = null;
  }

  start() {
    const now = performance.now();
    this.frameStart = now;

    if (this.secondStart === null) {
      this.secondStart = now;
    }

    if (now - this.secondStart > 1000) {
      this.fps = this.frameCounter;
      this.frameCounter = 0;
      this.secondStart = now;
    }
  }

  end() {
    const now = performance.now();
    this.dt = now - this.frameStart;

    this.frameCounter += 1;
  }

  // how much faster/slower do we need to be to hit the target fps
  differenceTarget() {
    return this.fps / TARGET_FPS;
  }

  onTarget() {
    const diff = Math.abs(1 - this.differenceTarget());
    return diff < 0.3;
  }

  reset() {
    this.fps = TARGET_FPS;
  }

  // factor for something that should happen after x seconds
  // posX += 10 * dtSec(2) means move 10 units in 2 sec
  dtSec(sec) {
    return this.dt / 1000 / sec;
  }

  toString() {
    return `${this.dt.toFixed(2)}ms (${this.fps} fps)`;
  }
}

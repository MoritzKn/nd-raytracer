const TARGET_FPS = 30;

export class DeltaTime {
  constructor() {
    this.targetDt = 1000 / TARGET_FPS;
    this.dt = this.targetDt;
    this.avgDt = null;
    this.frameStart = null;
    this.lastReset = 0;
  }

  start() {
    this.frameStart = performance.now();
  }

  end() {
    this.dt = performance.now() - this.frameStart;
    if (this.avgDt !== null) {
      this.avgDt = (this.avgDt * 30 + this.dt) / 31;
    } else if (this.lastReset > 0) {
      // HACK: Don't count the first frame after reset
      this.avgDt = this.dt;
    }
    this.lastReset += 1;
  }

  // how much faster/slower do we need to be to hit the target dt
  differenceTarget() {
    if (this.avgDt !== null) {
      return this.targetDt / this.avgDt;
    } else {
      return 1;
    }
  }

  onTarget() {
    let maxDiff;
    if (this.lastReset < 30) {
      maxDiff = 0.5;
    } else if (this.lastReset < 160) {
      maxDiff = 0.2;
    } else {
      maxDiff = 0.1;
    }
    const diff = Math.abs(1 - this.differenceTarget());
    return diff < maxDiff;
  }

  resetAvg() {
    this.avgDt = null;
    this.lastReset = 0;
  }

  // factor for something that should happen after x seconds
  // posX += 10 * dtSec(2) means move 10 units in 2 sec
  dtSec(sec) {
    return this.dt / 1000 / sec;
  }

  toString() {
    const avg = this.avgDt || this.dt;
    return `${this.dt.toFixed(2)}ms (avg: ${avg.toFixed(2)}ms)`;
  }
}

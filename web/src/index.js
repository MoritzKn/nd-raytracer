import { WorkerPool } from "./worker-pool";
import { DeltaTime } from "./delta-time";

const TAU = Math.PI * 2;

// Never devide the buffer in steps smaller than this...
// Thats what we use a step in the rendering
const STEP = 27;

function getRotation(angle, scale) {
  return [Math.cos(angle) * scale, Math.sin(angle) * scale];
}

const bounces = [];

function getBounce(id, current, range, speed, offset = 0) {
  current -= offset;

  const dir = bounces[id] || (Math.random() < 0.5 ? 1 : -1);
  current += dir * range * speed;

  const upperLimit = range * 0.5;
  if (current > upperLimit) {
    current = upperLimit - (current - upperLimit);
    bounces[id] = -dir;
  }

  const lowerLimit = range * -0.5;
  if (current < lowerLimit) {
    current = lowerLimit - (current - lowerLimit);
    bounces[id] = -dir;
  }

  current += offset;

  return current;
}

function sleep(ms) {
  return new Promise(resolve => {
    setTimeout(resolve, ms);
  });
}

function nextFrame(ms) {
  return new Promise(resolve => {
    requestAnimationFrame(resolve);
  });
}

class RenderController {
  constructor() {
    this.canvas = document.getElementById("canvas");
    this.ctx = canvas.getContext("2d");

    this.workerPool = new WorkerPool("worker.js");
    this.dt = new DeltaTime();

    this.camPos = [8, 8, 6, 8, -3, -2, -1, 0, 1, 2, 3, 4];
    this.dimension = 4;

    // Scale the canvas to hit the frame rate (targetDt)
    this.scale = 1;

    this.currentFrame = null;
    this.paused = true;

    let timeout = null;
    let resizeAction = null;
    window.addEventListener("resize", async () => {
      await resizeAction;
      clearTimeout(timeout);
      timeout = setTimeout(() => {
        resizeAction = this.stop().then(() => {
          this.resize();
          this.resume();
        });
      }, 50);
    });

    this.resize();
  }

  async start(dimension) {
    this.paused = false;
    if (dimension) {
      this.dimension = dimension;
    }

    await this.workerPool.broadcast("start", { dimension: this.dimension });

    this.dt.resetAvg();

    this.currentFrame = this.draw();
  }

  async stop() {
    this.paused = true;
    await this.currentFrame;
  }

  resume() {
    this.paused = false;
    this.currentFrame = this.draw();
  }

  async draw() {
    if (!this.dt.onTarget()) {
      // sqrt because update is O(scale^2)
      let diff = Math.sqrt(this.dt.differenceTarget());
      const newScale = Math.min(Math.max(this.scale * diff, 0.1), 2);
      await this.resize(newScale);
    }

    this.dt.start();

    /* **** Update Scene **** */

    let angle = Math.atan2(this.camPos[1], this.camPos[0]);
    this.camPos = [
      ...getRotation(angle + TAU * this.dt.dtSec(18), 8),
      getBounce(0, this.camPos[2], 8, this.dt.dtSec(6)),
      getBounce(1, this.camPos[3], 8, this.dt.dtSec(9)),
      getBounce(2, this.camPos[4], 8, this.dt.dtSec(6.5)),
      getBounce(3, this.camPos[5], 8, this.dt.dtSec(5.5)),
      getBounce(4, this.camPos[6], 8, this.dt.dtSec(5.25)),
      getBounce(5, this.camPos[7], 8, this.dt.dtSec(5)),
      getBounce(6, this.camPos[8], 8, this.dt.dtSec(4.75))
    ];

    /* **** Compute Image **** */

    let jobCount = this.workerPool.size() * 2;
    let jobs = [];
    let chunkSize = Math.ceil(this.canvas.height / jobCount / STEP) * STEP;

    for (var i = 0; i < jobCount; i++) {
      let start = chunkSize * i;
      let end = Math.min(start + chunkSize, this.canvas.height);
      let imageData = this.ctx.getImageData(0, start, this.canvas.width, end);

      jobs.push(
        this.workerPool.schedule("update", {
          data: imageData.data,
          camPos: this.camPos,
          start,
          end,
          width: this.canvas.width,
          height: this.canvas.height,
          dimension: this.dimension
        })
      );
    }

    const chunks = await Promise.all(jobs);
    this.dt.end();

    /* **** Draw to Canvas **** */

    // TODO: We know our frame rate, waiting for RAF slows us down
    // On the other hand, otherwise this runs without focus :/
    await nextFrame();

    chunks.forEach((data, i) => {
      const imageData = new ImageData(data, this.canvas.width);
      this.ctx.putImageData(imageData, 0, chunkSize * i);
    });

    console.log(`update dt: ${this.dt}, scale: ${this.scale.toFixed(2)}`);

    if (!this.paused) {
      this.currentFrame = this.draw();
    }
  }

  resize(newScale) {
    if (newScale) {
      if (newScale === this.scale) {
        return;
      }
      this.scale = newScale;
    }

    const wrapper = document.getElementById("wrapper");

    const width = Math.ceil((wrapper.clientWidth * this.scale) / STEP) * STEP;
    const height = Math.ceil((wrapper.clientHeight * this.scale) / STEP) * STEP;

    let canvasScale = Math.max(
      wrapper.clientWidth / width,
      wrapper.clientHeight / height
    );
    this.canvas.style.transform = `scale(${Math.max(canvasScale + 0.01, 1)})`;
    this.canvas.style.transformOrigin = `center`;
    this.canvas.width = width;
    this.canvas.height = height;

    this.dt.resetAvg();

    console.log(
      `rescale: ${this.scale.toFixed(2)}, canvas: ${width} ${height}`
    );
  }
}

const render = new RenderController();
render.start(4);

document.getElementById("dimension").addEventListener("change", async event => {
  const dimension = Math.max(Math.min(parseInt(event.target.value, 10), 9), 2);
  document.getElementById("dimension").value = dimension;

  await render.stop();
  await render.start(dimension);
});

import { WorkerPool } from "./worker-pool";
import { DeltaTime } from "./delta-time";

const dimensionsEl = document.getElementById("dimension");
const sceneEl = document.getElementById("scene");
const statsEl = document.getElementById("stats");

const TAU = Math.PI * 2;

// Never devide the buffer in steps smaller than this...
// Thats what we use a step in the rendering
const STEP = 9;

const bounces = [];

function getBounce(id, current, range, speed, offset = 0) {
  current -= offset;

  const dir = bounces[id] || (Math.random() < 0.5 ? 1 : -1);
  bounces[id] = dir;
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
  return new Promise(requestAnimationFrame);
}

class RenderController {
  constructor() {
    this.canvas = document.getElementById("canvas");
    this.ctx = canvas.getContext("2d");

    this.workerPool = new WorkerPool("worker.js");
    this.dt = new DeltaTime();

    this.camPos = [8, 8, 6, 8, -3, -2, -1, 0, 1, 2, 3, 4];
    this.dimension = 4;
    this.scene = "inital";

    // Scale the canvas to hit the frame rate (targetDt)
    this.scale = 1;
    this.imageData = [];

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

    setInterval(() => {
      statsEl.innerHTML = `
      fps: ${this.dt.fps()}<br>
      scale: ${this.scale.toFixed(2)}`;
    }, 500);

    this.resize();
  }

  async start({ dimension, scene }) {
    this.paused = false;
    if (dimension) {
      this.dimension = dimension;
    }
    if (scene) {
      this.scene = scene;
    }

    await this.workerPool.broadcast("start", {
      dimension: this.dimension,
      scene: this.scene
    });

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
    let rot = TAU * this.dt.dtSec(18);
    this.camPos[0] = Math.cos(angle + rot) * 8;
    this.camPos[1] = Math.sin(angle + rot) * 8;
    this.camPos[2] = getBounce(3, this.camPos[2], 8, this.dt.dtSec(8));
    this.camPos[3] = getBounce(4, this.camPos[3], 8, this.dt.dtSec(8));
    this.camPos[4] = getBounce(5, this.camPos[4], 8, this.dt.dtSec(8));
    this.camPos[5] = getBounce(6, this.camPos[5], 8, this.dt.dtSec(8));
    this.camPos[6] = getBounce(7, this.camPos[6], 8, this.dt.dtSec(8));
    this.camPos[7] = getBounce(8, this.camPos[7], 8, this.dt.dtSec(8));
    this.camPos[8] = getBounce(9, this.camPos[8], 8, this.dt.dtSec(8));

    /* **** Compute Image **** */

    let targetJobCount = this.workerPool.size() * 6;
    let chunkSize = Math.max(
      Math.ceil(this.canvas.height / targetJobCount / STEP) * STEP,
      STEP * 3 // we want to be able to do some adaptive rendering
    );
    let jobCount = Math.ceil(this.canvas.height / chunkSize);
    let jobs = [];

    for (var i = 0; i < jobCount; i++) {
      let start = chunkSize * i;
      let end = Math.min(start + chunkSize, this.canvas.height);
      let imageData =
        this.imageData[i] ||
        this.ctx.getImageData(0, start, this.canvas.width, end - start);

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

    await nextFrame();

    /* **** Draw to Canvas **** */

    chunks.forEach((data, i) => {
      const imageData = new ImageData(data, this.canvas.width);
      this.ctx.putImageData(imageData, 0, chunkSize * i);
      this.imageData[i] = imageData;
    });

    this.dt.end();

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

    const canvasScale = Math.max(
      wrapper.clientWidth / width,
      wrapper.clientHeight / height
    );
    this.canvas.style.transform = `scale(${canvasScale})`;
    this.canvas.style.transformOrigin = `top left`;
    this.canvas.width = width;
    this.canvas.height = height;

    this.dt.resetAvg();
    this.imageData = [];

    console.log(
      `rescale: ${this.scale.toFixed(2)}, canvas: ${width} ${height}`
    );
  }
}

const render = new RenderController();

async function updateScene() {
  const scene = sceneEl.value;
  const dimension = Math.max(Math.min(parseInt(dimensionsEl.value, 10), 9), 2);

  await render.stop();
  await render.start({ dimension, scene });
}

dimensionsEl.addEventListener("change", updateScene);
sceneEl.addEventListener("change", updateScene);

updateScene();

import { WorkerPool } from "./worker-pool";
import { DeltaTime } from "./delta-time";

const TAU = Math.PI * 2;

const canvas = document.getElementById("canvas");
const wrapper = document.getElementById("wrapper");
const ctx = canvas.getContext("2d");
const workerPool = new WorkerPool("worker.js");
const dt = new DeltaTime();

// Never device the buffer in steps smaller than this...
// Thats what we use a step in the rendering
const largestStep = 27;

let stop = true;
let dimension = 3;
let frameId = null;

// Scale the canvas to hit the frame rate (targetDt)
let scale = 1;

let camPos = [-6, 0, -2, 0];

function getRotation(angle, scale) {
  return [Math.cos(angle) * scale, Math.sin(angle) * scale];
}

async function draw() {
  if (stop) {
    return;
  }

  if (!dt.onTarget()) {
    // sqrt because update is O(scale^2)
    let diff = Math.sqrt(dt.differenceTarget());
    const newScale = Math.min(Math.max(scale * diff, 0.1), 2);

    if (newScale != scale) {
      scale = newScale;
      resize();
      dt.resetAvg();
    }
  }

  const imageDataWidth = canvas.width;

  let angle1 = Math.atan2(camPos[1], camPos[0]);
  let angle2 = Math.atan2(camPos[3], camPos[2]);
  camPos = [
    ...getRotation(angle1 + TAU * dt.dtSec(12), 8),
    ...getRotation(angle2 + TAU * dt.dtSec(6), 2)
  ];

  dt.start();

  let jobs = workerPool.size() * 2;
  let chunkSize = Math.ceil(canvas.height / jobs / largestStep) * largestStep;
  let chunks = [];
  for (var i = 0; i < jobs; i++) {
    let start = chunkSize * i;
    let end = Math.min(start + chunkSize, canvas.height);

    let imageData = ctx.getImageData(0, start, imageDataWidth, end);
    chunks.push(
      workerPool.schedule("update", {
        data: imageData.data,
        camPos,
        start,
        end,
        width: canvas.width,
        height: canvas.height,
        dimension
      })
    );
  }

  chunks = await Promise.all(chunks);

  if (stop) {
    return;
  }

  chunks.forEach((data, i) => {
    const imageData = new ImageData(data, imageDataWidth);
    ctx.putImageData(imageData, 0, chunkSize * i);
  });

  dt.end();

  console.log(`update dt: ${dt}, scale: ${scale.toFixed(2)}`);

  frameId = requestAnimationFrame(draw);
}

function resize() {
  canvas.width =
    Math.ceil((wrapper.clientWidth * scale) / largestStep) * largestStep;
  canvas.height =
    Math.ceil((wrapper.clientHeight * scale) / largestStep) * largestStep;

  let canvasScale = Math.max(
    wrapper.clientWidth / canvas.width,
    wrapper.clientHeight / canvas.height
  );
  canvas.style.transform = `scale(${Math.max(canvasScale + 0.01, 1)})`;
  canvas.style.transformOrigin = `center`;

  console.log(
    `rescale: ${scale.toFixed(2)}, canvas: ${canvas.width} ${canvas.height}`
  );
}

async function start() {
  stop = false;
  document.getElementById("dimension").value = dimension;
  await workerPool.broadcast("start", { dimension });

  frameId = requestAnimationFrame(draw);
}

window.addEventListener("resize", resize);
resize();

document.getElementById("dimension").addEventListener("change", event => {
  stop = true;
  cancelAnimationFrame(frameId);

  setTimeout(function() {
    dimension = Math.max(Math.min(parseInt(event.target.value, 10), 9), 2);
    start().catch(err => console.error(err));
  }, dt * 2);
});

start().catch(err => console.error(err));

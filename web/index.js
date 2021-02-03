const libPromise = import("./pkg");
const canvas = document.getElementById("canvas");
const wrapper = document.getElementById("wrapper");
const ctx = canvas.getContext("2d");

let dimension = 4;
let minCanvasDim;
let world;
let imageData;
let frameId = null;

let scale = 0.75;
let dt = 18;

let camPos = [-4, 0];

function getRotation(angle, scale) {
  return [Math.cos(angle) * scale, Math.sin(angle) * scale];
}

function draw() {
  if (dt > 24) {
    scale = Math.min(scale / (dt / 22), 2);
    resize(scale);
  }
  if (dt < 16) {
    scale = Math.max(scale / (dt / 18), 0.1);
    resize(scale);
  }

  // const camSpeed = 1 / 1000;
  // camPos[0] += camSpeed * dt;
  // if (camPos[0] > camDist * 2) camPos[0] -= camDist * 2;
  // camPos[1] += camSpeed * dt;
  // if (camPos[1] > camDist * 2) camPos[1] -= camDist * 2;

  let angle = Math.atan2(camPos[1], camPos[0]) + ((Math.PI / 8) * dt) / 1000;
  camPos = getRotation(angle, 6);

  let start = performance.now();
  const res = lib.update(
    imageData.data,
    world,
    camPos,
    imageData.width,
    imageData.height,
    minCanvasDim,
    dimension
  );

  imageData = new ImageData(res, imageData.width);
  ctx.putImageData(imageData, 0, 0);
  dt = performance.now() - start;
  console.log(`update dt: ${dt.toFixed(0)}ms, scale: ${scale.toFixed(2)}`);

  frameId = requestAnimationFrame(draw);
}

function resize() {
  const step = 27;

  canvas.width = Math.floor(wrapper.clientWidth * scale);
  canvas.height = Math.floor(wrapper.clientHeight * scale);
  minCanvasDim = Math.min(canvas.width, canvas.height);

  let bufferWidth = Math.ceil(canvas.width / step) * step;
  let bufferHeight = Math.ceil(canvas.height / step) * step;
  imageData = ctx.getImageData(0, 0, bufferWidth, bufferHeight);

  canvas.style.transform = `
  scale(${1 / scale})
  translate(
    ${(bufferWidth - canvas.width) / -2}px,
    ${(bufferHeight - canvas.height) / -2}px
    )`;
  canvas.style.transformOrigin = `left top`;

  console.log(
    `resize: scale: ${scale.toFixed(2)}, buffer: ${bufferWidth} ${bufferHeight}`
  );
}

function stackSpheres(world, dimension) {
  const count = 2 ** dimension;
  const outerR = 1;

  for (let i = 0; i < count; i++) {
    // this is so dumm but it works
    const pos = i
      .toString(2)
      .padStart(dimension, "0")
      .split("")
      .map(Number)
      .map(n => n * 2 - 1);
    world.add_sphere(
      pos,
      new lib.Sphere(outerR, lib.Color.rgba(0.15, 0.35, 1, 0.78))
    );
  }
  const innerR = Math.sqrt(dimension) - outerR;
  world.add_sphere(
    [],
    new lib.Sphere(innerR, lib.Color.rgba(1, 0.4, 0.2, 0.63))
  );
}

async function init() {
  window.lib = await libPromise;

  window.addEventListener("resize", resize);
  resize();

  world = new lib.World();

  stackSpheres(world, dimension);

  frameId = requestAnimationFrame(draw);
}

document.getElementById("dimension").addEventListener("change", event => {
  cancelAnimationFrame(frameId);
  dimension = Math.max(Math.min(parseInt(event.target.value, 10), 9), 2);
  event.target.value = dimension;
  init();
});

init().catch(err => console.error(err));

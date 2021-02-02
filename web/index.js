const libPromise = import("./pkg");
const canvas = document.getElementById("canvas");
const wrapper = document.getElementById("wrapper");
const ctx = canvas.getContext("2d");

let minCanvasDim;
let world;
let imageData;

function draw() {
  console.time("update");
  const res = lib.update(
    world,
    canvas.width,
    canvas.height,
    minCanvasDim,
    imageData.data
  );

  imageData = new ImageData(res, canvas.width);
  ctx.putImageData(imageData, 0, 0);
  console.timeEnd("update");

  requestAnimationFrame(draw);
}

function resize() {
  canvas.width = wrapper.clientWidth;
  canvas.height = wrapper.clientHeight;
  // canvas.width = 800;
  // canvas.height = 700;
  minCanvasDim = Math.min(canvas.width, canvas.height);
  imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
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

  stackSpheres(world, 4);

  requestAnimationFrame(draw);
}

init().catch(err => console.error(err));

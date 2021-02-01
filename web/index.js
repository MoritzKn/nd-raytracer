const libPromise = import("./pkg");
const canvas = document.getElementById("canvas");
const wrapper = document.getElementById("wrapper");
const ctx = canvas.getContext("2d");

let minCanvasDim;
let world;
let imageData;

function draw() {
  const res = lib.update(world, canvas.width, canvas.height, minCanvasDim, imageData.data);
  imageData = new ImageData(res, canvas.width);

  ctx.putImageData(imageData, 0, 0);

  requestAnimationFrame(draw);
}

function resize() {
  canvas.width = wrapper.clientWidth;
  canvas.height = wrapper.clientHeight;
  // canvas.width = 100;
  // canvas.height = 100;
  minCanvasDim = Math.min(canvas.height, canvas.width);
  imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
}

async function init() {
  window.lib = await libPromise;
  console.log(lib);

  window.addEventListener("resize", resize);
  resize();

  world = new lib.World();
  world.add_sphere(new lib.Sphere(1, lib.Color.rgb(1, 0, 1)))

  requestAnimationFrame(draw);
}

init().catch(err => console.error(err));

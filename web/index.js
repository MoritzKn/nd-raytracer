const libPromise = import("./pkg");
const canvas = document.getElementById("canvas");
const wrapper = document.getElementById("wrapper");
const ctx = canvas.getContext("2d");

let dimension = 3;
let minCanvasDim;
let world;
let imageData;
let frameId = null;

let scale = 1;
// delta time (ms)
let dt = 18;
const targetDt = 32;

let camPos = [-6, 0, -2, 0];

const TAU = Math.PI * 2;

function getRotation(angle, scale) {
  return [Math.cos(angle) * scale, Math.sin(angle) * scale];
}

// factor for something that should happen after x seconds
// posX += 10 * dtSec(2) means move 10 units in 2 sec
function dtSec(sec) {
  return dt / 1000 / sec;
}

function draw() {
  if (dt < targetDt * 0.8 || dt > targetDt * 1.2) {
    // how much faster/slower do we need to be to hit the target dt
    let div = targetDt / dt;
    // sqrt because update is O(scale^2)
    scale *= Math.sqrt(div);
    scale = Math.min(Math.max(scale, 0.1), 2);
    resize(scale);
  }

  let angle1 = Math.atan2(camPos[1], camPos[0]);
  let angle2 = Math.atan2(camPos[3], camPos[2]);
  camPos = [
    ...getRotation(angle1 + TAU * dtSec(12), 8),
    ...getRotation(angle2 + TAU * dtSec(6), 2)
  ];

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
  console.log(`update dt: ${dt.toFixed(2)}ms, scale: ${scale.toFixed(2)}`);

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
      new lib.Sphere(
        outerR,
        lib.Color.rgba(i / count, (count - i) / count, 1, 0.8),
        0.8
      )
    );
  }
  const innerR = Math.sqrt(dimension) - outerR;
  world.add_sphere(
    [],
    new lib.Sphere(innerR, lib.Color.rgba(1, 0.4, 0.2, 1), 0.8)
  );
}

function cube(world, dim, pos, i) {
  let count = 4;
  let scale = 2;
  let radius = 0.4;
  let outline = true;

  if (dim === 0) {
    const axies = pos.filter(c => Math.abs(c) === scale / 2).length;

    if (outline && axies < dimension - 1) {
      return;
    }

    world.add_sphere(
      pos,
      new lib.Sphere(
        radius,
        lib.Color.rgba(
          axies / (dimension - 1),
          (dimension - axies) / (dimension - 1),
          (dimension - axies) / (dimension - 1),
          0.8
        )
      )
    );

    return;
  }

  for (var i = 0; i < count; i++) {
    // recursively go through dimensions
    // component is fitst x, than y, than z, etc
    const component = (i / (count - 1)) * scale - scale / 2;
    cube(world, dim - 1, [component, ...pos], i);
  }
}

async function init() {
  document.getElementById("dimension").value = dimension;
  window.lib = await libPromise;

  window.addEventListener("resize", resize);
  resize();

  world = new lib.World();

  stackSpheres(world, dimension);
  // cube(world, dimension, [], true, 0);

  world.add_light(
    [-6.0, -6.0, 12.0, 6.0],
    new lib.Light(lib.Color.rgba(1, 1, 1, 0.6))
  );

  // world.add_light(
  //   [-6.0, 6.0, 6.0, 4.0],
  //   new lib.Light(lib.Color.rgba(1, 1, 1, 0.2))
  // );
  // world.add_light(
  //   [6.0, 6.0, 6.0, 3.0],
  //   new lib.Light(lib.Color.rgba(1, 1, 1, 0.2))
  // );

  // world.add_sphere(
  //   [3, 0, 0],
  //   new lib.Sphere(2, lib.Color.rgba(0.9, 0.9, 0.9, 1), 0.9)
  // );

  // world.add_sphere([-2, 1], new lib.Sphere(0.4, lib.Color.rgba(0, 1, 0, 0.5)));
  // world.add_sphere(
  //   [-2, 0],
  //   new lib.Sphere(0.4, lib.Color.rgba(0, 0.5, 0, 0.9), 0.9)
  // );
  // world.add_sphere(
  //   [-2, -1],
  //   new lib.Sphere(0.4, lib.Color.rgba(0.5, 1, 0.5, 0.9), 0.1)
  // );

  // world.add_sphere([], new lib.Sphere(3, lib.Color.rgba(1, 0.5, 0, 0.1)));
  // world.add_sphere([1.2], new lib.Sphere(2, lib.Color.rgba(0, 1, 0.5, 0.5)));
  // world.add_sphere([-2], new lib.Sphere(1.5, lib.Color.rgba(0, 0.5, 1, 1)));

  for (var i = -3; i <= 3; i++) {
    for (var j = -3; j <= 3; j++) {
      world.add_sphere(
        [i * 1.2, j * 1.2, -4],
        new lib.Sphere(0.8, lib.Color.rgba(0.9, 0.9, 0.9, 1))
      );
    }
  }

  world.add_sphere(
    [0, 4.5],
    new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.6, 0.1, 1), 0.4)
  );
  world.add_sphere(
    [4.5, 0],
    new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.6, 0.1, 1), 0.4)
  );
  world.add_sphere(
    [0, -4.5],
    new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.6, 0.1, 1), 0.4)
  );
  world.add_sphere(
    [-4.5, 0],
    new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.6, 0.1, 1), 0.4)
  );

  frameId = requestAnimationFrame(draw);
}

document.getElementById("dimension").addEventListener("change", event => {
  cancelAnimationFrame(frameId);
  dimension = Math.max(Math.min(parseInt(event.target.value, 10), 9), 2);
  init();
});

init().catch(err => console.error(err));

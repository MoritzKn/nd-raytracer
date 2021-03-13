window.canvas = document.getElementById("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
window.ctx = canvas.getContext("2d");

let size = 25;
let DPR = window.devicePixelRatio;
let SCALE =
  Math.floor(Math.min(window.innerWidth, window.innerHeight) / DPR / size) *
  DPR;

function line(x1, y1, x2, y2, color = "black") {
  ctx.lineWidth = 1 * DPR;
  ctx.strokeStyle = color;

  ctx.beginPath();
  ctx.moveTo(x1 * SCALE, y1 * SCALE);
  ctx.lineTo(x2 * SCALE, y2 * SCALE);
  ctx.stroke();
}

function ray(x, y, xdir, ydir) {
  line(x, y, x + xdir * 100, y + ydir * 100);
}

function box(x, y, width, height, color = "black") {
  ctx.lineWidth = 1 * DPR;
  ctx.strokeStyle = color;

  ctx.rect(x * SCALE, y * SCALE, width * SCALE, height * SCALE);
  ctx.stroke();
}

function point(x, y, label = "", color = "black") {
  line(x - 0.2, y - 0.2, x + 0.2, y + 0.2, color);
  line(x - 0.2, y + 0.2, x + 0.2, y - 0.2, color);
  ctx.font = "16px serif";
  ctx.fillStyle = color;
  ctx.fillText(`${x}, ${y} ${label}`, (x + 0.1) * SCALE, (y + 0.5) * SCALE);
}

function grid() {
  for (let x = 0; x < size * 4; x++) {
    line(x, 0, x, 100, "#ddd");
  }
  for (let y = 0; y < size * 4; y++) {
    line(0, y, 100, y, "#ddd");
  }
}

const mul = (a, b) => a * b;
const div = (a, b) => a / b;
const add = (a, b) => a + b;
const sub = (a, b) => a - b;

function vec(...components) {
  components.x = components[0];
  components.y = components[1];
  components.z = components[2];
  components.w = components[3];

  return components;
}

function vecOp(op, a, b) {
  return a.map((ac, i) => op(ac, b[i]));
}

function vecOpScalar(op, a, b) {
  return a.map(ac => op(ac, b));
}

function normalize(...components) {
  const len = Math.hypot(...components);
  return vecOpScalar(div, components, len);
}

function aabbIntersection(x, y, width, height, rayX, rayY, rayNX, rayNY) {
  ray(rayX, rayY, rayNX, rayNY);
  point(rayX, rayY);

  let aabbMinX = x;
  let aabbMaxX = x + width;
  let aabbMinY = y;
  let aabbMaxY = y + height;

  let tmin = -Infinity;
  let tmax = Infinity;

  if (rayNX != 0.0) {
    let tx1 = (aabbMinX - rayX) / rayNX;
    let tx2 = (aabbMaxX - rayX) / rayNX;

    tmin = Math.max(tmin, Math.min(tx1, tx2));
    tmax = Math.min(tmax, Math.max(tx1, tx2));
  }

  // line(rayX, rayY, rayX + rayNX * tmin, rayY + rayNY * tmin, "green");
  // point(rayX + rayNX * tmin, rayY + rayNY * tmin, "tmin x", "green");

  // line(rayX, rayY, rayX + rayNX * tmax, rayY + rayNY * tmax, "blue");
  // point(rayX + rayNX * tmax, rayY + rayNY * tmax, "tmax x", "blue");

  if (rayNY != 0.0) {
    let ty1 = (aabbMinY - rayY) / rayNY;
    let ty2 = (aabbMaxY - rayY) / rayNY;

    tmin = Math.max(tmin, Math.min(ty1, ty2));
    tmax = Math.min(tmax, Math.max(ty1, ty2));
  }

  // line(rayX, rayY, rayX + rayNX * tmin, rayY + rayNY * tmin, "red");
  // point(rayX + rayNX * tmin, rayY + rayNY * tmin, "tmin y", "red");

  // line(rayX, rayY, rayX + rayNX * tmax, rayY + rayNY * tmax, "yellow");
  // point(rayX + rayNX * tmax, rayY + rayNY * tmax, "tmax y", "yellow");

  let hit = tmax >= tmin && (tmin > 0 || tmax > 0);

  point(
    rayX + rayNX * tmin,
    rayY + rayNY * tmin,
    tmin > 0 ? "min (hit)" : "min (no hit)",
    "blue"
  );
  point(
    rayX + rayNX * tmax,
    rayY + rayNY * tmax,
    tmax > 0 ? "max (hit)" : "max (no hit)",
    "blue"
  );

  return hit;
}

function draw() {
  DPR = window.devicePixelRatio;
  SCALE =
    Math.floor(Math.min(window.innerWidth, window.innerHeight) / DPR / size) *
    DPR;
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  grid();

  line(0, 14, 100, 14, "green");
  line(0, 10, 100, 10, "green");
  line(4, 0, 4, 100, "green");
  line(8, 0, 8, 100, "green");
  point(4, 10);
  point(8, 10);
  point(4, 14);
  point(8, 14);
  box(4, 10, 4, 4);

  const dir = normalize(2, -1);
  aabbIntersection(4, 10, 4, 4, 5, 13, ...dir);
}

draw();
window.addEventListener("resize", draw);

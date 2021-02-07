const libPromise = import("../pkg");
let lib;
let world;

function packSpheres2(world) {
  let colorN = 0;
  const r = 1;
  for (var x = -1; x <= 1; x++) {
    for (var y = -1; y <= 1; y++) {
      colorN++;
      const color = (colorN % 3)
        .toString(2)
        .padStart(3, "0")
        .split("")
        .map(Number)
        .map(n => n * 0.6 + 0.2);
      world.add_sphere(
        [y % 2 === 0 ? x * 2 + 1 : x * 2, y * Math.sqrt(2)],
        new lib.Sphere(r, lib.Color.rgba(...color, 0.5))
      );
    }
  }
}

function packSpheres3(world) {
  let colorN = 0;
  const r = Math.hypot(1, 0.5, 0.5) / 2;
  for (var x = -1; x <= 1; x++) {
    for (var y = -1; y <= 1; y++) {
      for (var z = -1; z <= 1; z++) {
        colorN++;
        const color = (colorN % 3)
          .toString(2)
          .padStart(3, "0")
          .split("")
          .map(Number)
          .map(n => n * 0.6 + 0.2);
        world.add_sphere(
          [x, y + (x % 2) / 2, z + (y % 2) / 2, 0.3],
          new lib.Sphere(r, lib.Color.rgba(...color, 0.5))
        );
      }
    }
  }
}

function packSpheres4(world) {
  let colorN = 0;
  const r = Math.hypot(0.5, 0.5, 0.5, 0.5) / 2;
  for (var x = -1; x <= 1; x++) {
    for (var y = -1; y <= 1; y++) {
      for (var z = -1; z <= 1; z++) {
        for (var w = -1; w <= 1; w++) {
          colorN++;
          const color = (colorN % 3)
            .toString(2)
            .padStart(2, "0")
            .split("")
            .map(Number)
            .map(n => n * 0.6 + 0.2);
          world.add_sphere(
            [x, y, z, w],
            new lib.Sphere(r, lib.Color.rgba(...color, 0, 0.5))
          );
          world.add_sphere(
            [x + 1 / 2, y + 1 / 2, z + 1 / 2, w + 1 / 2],
            new lib.Sphere(r, lib.Color.rgba(...color, 1, 0.5))
          );
        }
      }
    }
  }
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
        0.6
      )
    );
  }
  const innerR = Math.sqrt(dimension) - outerR;
  world.add_sphere(
    [],
    new lib.Sphere(innerR, lib.Color.rgba(1, 0.4, 0.2, 1), 0.6)
  );
}

function cube(world, dimension, pos, orgDimension = dimension) {
  let count = 4;
  let scale = 2;
  let radius = 0.4;
  let outline = true;

  if (dimension === 0) {
    const axies = pos.filter(c => Math.abs(c) === scale / 2).length;

    if (outline && axies < 2) {
      return;
    }

    world.add_sphere(
      pos,
      new lib.Sphere(
        radius,
        lib.Color.rgba(
          axies / (orgDimension - 2),
          (orgDimension - axies) / (orgDimension - 1),
          (orgDimension - axies) / (orgDimension - 1),
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
    cube(world, dimension - 1, [component, ...pos], orgDimension);
  }
}

function update({ data, camPos, start, end, width, height, dimension }) {
  return lib.update(
    data,
    world,
    camPos,
    start,
    end,
    width,
    height,
    Math.min(width, height),
    dimension
  );
}

async function start({ dimension }) {
  lib = await libPromise;

  world = new lib.World();

  // stackSpheres(world, dimension);
  // cube(world, dimension, [], dimension);
  // packSpheres2(world);
  // packSpheres3(world);
  packSpheres4(world);

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

  if (dimension > 2) {
    for (var i = -1.5; i <= 1.5; i++) {
      for (var j = -1.5; j <= 1.5; j++) {
        world.add_sphere(
          [i * 1.2, j * 1.2, -3],
          new lib.Sphere(0.6, lib.Color.rgba(0.9, 0.9, 0.9, 1), 0.7)
        );
      }
    }
  }

  // world.add_sphere(
  //   [0, 3.5, 0],
  //   new lib.Sphere(0.5, lib.Color.rgba(0, 1, 0.1, 1), 0.2)
  // );
  // world.add_sphere(
  //   [3.5, 0, 0],
  //   new lib.Sphere(0.5, lib.Color.rgba(0, 1, 0.1, 1), 0.2)
  // );
  // world.add_sphere(
  //   [0, -3.5, 0],
  //   new lib.Sphere(0.5, lib.Color.rgba(1, 0.1, 0, 1), 0.2)
  // );
  // world.add_sphere(
  //   [-3.5, 0, 0],
  //   new lib.Sphere(0.5, lib.Color.rgba(1, 0.1, 0, 1), 0.2)
  // );
}

function handleEvent(type, data) {
  switch (type) {
    case "start":
      return start(data);
    case "update":
      return update(data);
  }
}

self.addEventListener("message", async function(event) {
  const { type, id, data } = event.data;
  try {
    const res = await handleEvent(type, data);
    let transfer = [];
    if (res && res.data && res.data.buffer) {
      transfer.push(res.data.buffer);
    }
    self.postMessage({ type, id, data: res }, transfer);
  } catch (error) {
    const res = await handleEvent(type, data);
    self.postMessage({ type, id, error });
  }
});

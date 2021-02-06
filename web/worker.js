const libPromise = import("./pkg");
let lib;
let world;

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

    if (outline && axies < orgDimension - 1) {
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

  stackSpheres(world, dimension);
  // cube(world, dimension, [], dimension);

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

  for (var i = -2; i <= 2; i++) {
    for (var j = -2; j <= 2; j++) {
      world.add_sphere(
        [i * 1.2, j * 1.2, -3],
        new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.9, 0.9, 1))
      );
    }
  }

  world.add_sphere(
    [0, 4.5],
    new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.6, 0.1, 1))
  );
  world.add_sphere(
    [4.5, 0],
    new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.6, 0.1, 1))
  );
  world.add_sphere(
    [0, -4.5],
    new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.6, 0.1, 1))
  );
  world.add_sphere(
    [-4.5, 0],
    new lib.Sphere(0.5, lib.Color.rgba(0.9, 0.6, 0.1, 1))
  );
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

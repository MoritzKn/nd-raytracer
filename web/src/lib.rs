extern crate wasm_bindgen;

mod color;
mod fixedvector;
mod tracer;
mod vector;
mod world;

use wasm_bindgen::prelude::*;

use color::{Color, ColorInt};
use fixedvector::FixedVector;
use tracer::{sample, DimensionalWorld};
use vector::Vector;
use world::World;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused)]
macro_rules! log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub type Float = f32;

fn set_px(
    data: &mut wasm_bindgen::Clamped<Vec<u8>>,
    start: isize,
    width: isize,
    x: isize,
    y: isize,
    color: ColorInt,
) {
    let index = ((x + (y - start) * width) * 4) as usize;

    data[index + 0] = color[0];
    data[index + 1] = color[1];
    data[index + 2] = color[2];
    data[index + 3] = 255; // color[3];
}

fn get_px(
    data: &wasm_bindgen::Clamped<Vec<u8>>,
    start: isize,
    width: isize,
    x: isize,
    y: isize,
) -> ColorInt {
    let index = ((x + (y - start) * width) * 4) as usize;

    [
        data[index + 0],
        data[index + 1],
        data[index + 2],
        data[index + 3],
    ]
}

fn get_px_checked(
    data: &wasm_bindgen::Clamped<Vec<u8>>,
    start: isize,
    width: isize,
    x: isize,
    y: isize,
) -> Option<ColorInt> {
    let index = ((x + (y - start) * width) * 4) as usize;

    // Overflows so we only need to check upper limit
    if index + 3 >= data.len() {
        None
    } else {
        Some([
            data[index + 0],
            data[index + 1],
            data[index + 2],
            data[index + 3],
        ])
    }
}

fn test_deviation(
    center: Color,
    top: Option<Color>,
    right: Option<Color>,
    bottom: Option<Color>,
    left: Option<Color>,
    threshold: Float,
) -> bool {
    if let Some(top) = top {
        if center != top && center.div(&top) > threshold {
            return true;
        };
    }
    if let Some(right) = right {
        if center != right && center.div(&right) > threshold {
            return true;
        }
    }
    if let Some(bottom) = bottom {
        if center != bottom && center.div(&bottom) > threshold {
            return true;
        }
    }
    if let Some(left) = left {
        if center != left && center.div(&left) > threshold {
            return true;
        }
    }

    false
}

fn init_sample_grid<V: Vector>(
    data: &mut wasm_bindgen::Clamped<Vec<u8>>,
    world: &DimensionalWorld<V>,
    start: isize,
    end: isize,
    width: isize,
    height: isize,
    min_dim: Float,
    step: isize,
) {
    let offset_x = (min_dim - width as Float) / 2.0;
    let offset_y = (min_dim - height as Float) / 2.0;

    let step_offset = step / 2;
    for step_y in (start + step_offset..end - step_offset).step_by(step as usize) {
        let rel_y = 1.0 - (step_y as Float + offset_y) / min_dim;

        for step_x in (step_offset..width - step_offset).step_by(step as usize) {
            let rel_x = (step_x as Float + offset_x) / min_dim;

            let color = sample::<V>(&world, rel_x, rel_y).to_int();
            set_px(data, start, width, step_x, step_y, color);
        }
    }
}

fn fill_sample_grid<V: Vector>(
    data: &mut wasm_bindgen::Clamped<Vec<u8>>,
    world: &DimensionalWorld<V>,
    start: isize,
    end: isize,
    width: isize,
    height: isize,
    min_dim: Float,
    step: isize,
    substep: isize,
    deviation_threshold: Float,
) {
    let offset_x = (min_dim - width as Float) / 2.0;
    let offset_y = (min_dim - height as Float) / 2.0;

    // NOTE: offset is floored!
    let step_offset = step / 2;
    let substep_offset = substep / 2;

    assert!((end - start) % step == 0);
    assert!(width % step == 0);
    assert!(height % step == 0);

    for step_y in (start + step_offset..end - step_offset).step_by(step as usize) {
        let substep_range_y = ((step_y - step_offset + substep_offset)..(step_y + step_offset + 1))
            .step_by(substep as usize);

        for step_x in (step_offset..width - step_offset).step_by(step as usize) {
            let substep_range_x = ((step_x - step_offset + substep_offset)
                ..(step_x + step_offset + 1))
                .step_by(substep as usize);

            let center_int = get_px(&data, start, width, step_x, step_y);
            let center = Color::from_int(&center_int);
            let top = get_px_checked(&data, start, width, step_x, step_y - step)
                .as_ref()
                .map(Color::from_int);
            let right = get_px_checked(&data, start, width, step_x + step, step_y)
                .as_ref()
                .map(Color::from_int);
            let bottom = get_px_checked(&data, start, width, step_x, step_y + step)
                .as_ref()
                .map(Color::from_int);
            let left = get_px_checked(&data, start, width, step_x - step, step_y)
                .as_ref()
                .map(Color::from_int);

            let resample = top.is_none()
                || bottom.is_none()
                || test_deviation(center, top, bottom, left, right, deviation_threshold);

            for substep_y in substep_range_y.clone() {
                let rel_y = 1.0 - (substep_y as Float + offset_y) / min_dim;

                for substep_x in substep_range_x.clone() {
                    if substep_x == step_x && substep_y == step_y {
                        // center is already sampled
                        continue;
                    }

                    let rel_x = (substep_x as Float + offset_x) / min_dim;

                    let color = if resample {
                        sample::<V>(&world, rel_x, rel_y).to_int()
                    } else {
                        center_int
                    };

                    set_px(data, start, width, substep_x, substep_y, color);
                }
            }
        }
    }
}

fn update_n<V: Vector>(
    mut data: wasm_bindgen::Clamped<Vec<u8>>,
    world: &World,
    cam_pos: Vec<Float>,
    start: isize,
    end: isize,
    width: isize,
    height: isize,
    min_dim: Float,
) -> wasm_bindgen::Clamped<Vec<u8>> {
    let cam_pos = V::pad(&cam_pos, -8.0);
    let world = DimensionalWorld::from_world(world, cam_pos);

    init_sample_grid::<V>(&mut data, &world, start, end, width, height, min_dim, 9);
    // fill_sample_grid::<V>(
    //     &mut data, &world, start, end, width, height, min_dim, 27, 9, 0.05,
    // );
    fill_sample_grid::<V>(
        &mut data, &world, start, end, width, height, min_dim, 9, 3, 0.05,
    );
    fill_sample_grid::<V>(
        &mut data, &world, start, end, width, height, min_dim, 3, 1, 0.1,
    );

    data
}

#[wasm_bindgen]
pub fn update(
    data: wasm_bindgen::Clamped<Vec<u8>>,
    world: &World,
    cam_pos: Vec<Float>,
    start: isize,
    end: isize,
    width: isize,
    height: isize,
    min_dim: Float,
    dimension: usize,
) -> wasm_bindgen::Clamped<Vec<u8>> {
    match dimension {
        2 => update_n::<FixedVector<2>>(data, world, cam_pos, start, end, width, height, min_dim),
        3 => update_n::<FixedVector<3>>(data, world, cam_pos, start, end, width, height, min_dim),
        4 => update_n::<FixedVector<4>>(data, world, cam_pos, start, end, width, height, min_dim),
        5 => update_n::<FixedVector<5>>(data, world, cam_pos, start, end, width, height, min_dim),
        6 => update_n::<FixedVector<6>>(data, world, cam_pos, start, end, width, height, min_dim),
        7 => update_n::<FixedVector<7>>(data, world, cam_pos, start, end, width, height, min_dim),
        8 => update_n::<FixedVector<8>>(data, world, cam_pos, start, end, width, height, min_dim),
        9 => update_n::<FixedVector<9>>(data, world, cam_pos, start, end, width, height, min_dim),
        _ => data,
    }
}

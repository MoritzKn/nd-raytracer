extern crate wasm_bindgen;

use wasm_bindgen::__rt::core::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

type Float = f32;

trait Vector:
    Sized
    + Copy
    + Debug
    + std::ops::Mul<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Mul<Float, Output = Self>
    + std::ops::Add<Float, Output = Self>
    + std::ops::Sub<Float, Output = Self>
    + std::ops::Div<Float, Output = Self>
{
    fn new() -> Self;

    fn from_iter(iter: impl Iterator<Item = Float>) -> Self;

    fn pad(base: &[Float], default: Float) -> Self;

    fn components(&self) -> &[Float];

    fn length(&self) -> Float {
        let sum_of_squares = self
            .components()
            .into_iter()
            .map(|c| c * c)
            .fold(0.0, |a, b| a + b);

        Float::sqrt(sum_of_squares)
    }

    fn normalize(&self) -> Self {
        let len = self.length();

        self.div_scalar(len)
    }

    fn dot(&self, other: &Self) -> Float {
        self.components()
            .into_iter()
            .zip(other.components().into_iter())
            .map(|(a, b)| a * b)
            .fold(0.0, |a, b| a + b)
    }

    fn add(&self, other: &Self) -> Self {
        Self::from_iter(
            self.components()
                .into_iter()
                .zip(other.components().into_iter())
                .map(|(a, b)| a + b),
        )
    }

    fn sub(&self, other: &Self) -> Self {
        Self::from_iter(
            self.components()
                .into_iter()
                .zip(other.components().into_iter())
                .map(|(a, b)| a - b),
        )
    }

    fn mul(&self, other: &Self) -> Self {
        Self::from_iter(
            self.components()
                .into_iter()
                .zip(other.components().into_iter())
                .map(|(a, b)| a * b),
        )
    }

    fn div(&self, other: &Self) -> Self {
        Self::from_iter(
            self.components()
                .into_iter()
                .zip(other.components().into_iter())
                .map(|(a, b)| a / b),
        )
    }

    fn add_scalar(&self, other: Float) -> Self {
        Self::from_iter(self.components().into_iter().map(|a| a + other))
    }

    fn sub_scalar(&self, other: Float) -> Self {
        Self::from_iter(self.components().into_iter().map(|a| a - other))
    }

    fn mul_scalar(&self, other: Float) -> Self {
        Self::from_iter(self.components().into_iter().map(|a| a * other))
    }

    fn div_scalar(&self, other: Float) -> Self {
        Self::from_iter(self.components().into_iter().map(|a| a / other))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NdVec<const L: usize> {
    components: [Float; L],
}

impl<const L: usize> Vector for NdVec<L> {
    fn new() -> Self {
        Self {
            components: [0.0; L],
        }
    }

    #[inline]
    fn from_iter(iter: impl Iterator<Item = Float>) -> Self {
        let mut components = [0.0; L];

        for (comp, value) in components.iter_mut().zip(iter) {
            *comp = value;
        }

        Self { components }
    }

    fn pad(base: &[Float], default: Float) -> Self {
        let mut components = [default; L];

        for (comp, value) in components.iter_mut().zip(base.iter()) {
            *comp = *value;
        }

        Self { components }
    }

    #[inline]
    fn components(&self) -> &[Float] {
        &self.components
    }
}

impl<const L: usize> std::ops::Add<NdVec<L>> for NdVec<L> {
    type Output = NdVec<L>;

    fn add(self, other: NdVec<L>) -> NdVec<L> {
        Vector::add(&self, &other)
    }
}

impl<const L: usize> std::ops::Sub<NdVec<L>> for NdVec<L> {
    type Output = NdVec<L>;

    fn sub(self, other: NdVec<L>) -> NdVec<L> {
        Vector::sub(&self, &other)
    }
}

impl<const L: usize> std::ops::Mul<NdVec<L>> for NdVec<L> {
    type Output = NdVec<L>;

    fn mul(self, other: NdVec<L>) -> NdVec<L> {
        Vector::mul(&self, &other)
    }
}

impl<const L: usize> std::ops::Div<NdVec<L>> for NdVec<L> {
    type Output = NdVec<L>;

    fn div(self, other: NdVec<L>) -> NdVec<L> {
        Vector::div(&self, &other)
    }
}

impl<const L: usize> std::ops::Add<Float> for NdVec<L> {
    type Output = NdVec<L>;

    fn add(self, other: Float) -> NdVec<L> {
        Vector::add_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Sub<Float> for NdVec<L> {
    type Output = NdVec<L>;

    fn sub(self, other: Float) -> NdVec<L> {
        Vector::sub_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Mul<Float> for NdVec<L> {
    type Output = NdVec<L>;

    fn mul(self, other: Float) -> NdVec<L> {
        Vector::mul_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Div<Float> for NdVec<L> {
    type Output = NdVec<L>;

    fn div(self, other: Float) -> NdVec<L> {
        Vector::div_scalar(&self, other)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    array: [Float; 4],
}

type ColorInt = [u8; 4];

#[wasm_bindgen]
impl Color {
    pub fn rgba(r: Float, g: Float, b: Float, a: Float) -> Self {
        Self {
            array: [r, g, b, a],
        }
    }

    pub fn rgb(r: Float, g: Float, b: Float) -> Self {
        Self {
            array: [r, g, b, 1.0],
        }
    }

    pub fn red(&self) -> Float {
        self.array[0]
    }
    pub fn green(&self) -> Float {
        self.array[1]
    }
    pub fn blue(&self) -> Float {
        self.array[2]
    }
    pub fn alpha(&self) -> Float {
        self.array[3]
    }

    fn from_int(slice: &ColorInt) -> Self {
        Self {
            array: [
                (slice[0] as Float / 255.0),
                (slice[1] as Float / 255.0),
                (slice[2] as Float / 255.0),
                (slice[3] as Float / 255.0),
            ],
        }
    }

    fn to_int(&self) -> ColorInt {
        [
            (self.array[0] * 255.0) as u8,
            (self.array[1] * 255.0) as u8,
            (self.array[2] * 255.0) as u8,
            (self.array[3] * 255.0) as u8,
        ]
    }

    fn apply(&mut self, top: &Self) {
        let alpha = top.alpha();
        let invert = 1.0 - alpha;

        self.array[0] = self.red() * invert + top.red() * alpha;
        self.array[1] = self.green() * invert + top.green() * alpha;
        self.array[2] = self.blue() * invert + top.blue() * alpha;
    }

    fn mix(&self, other: &Color) -> Color {
        Self::rgba(
            (self.red() + other.red()) / 2.0,
            (self.green() + other.green()) / 2.0,
            (self.blue() + other.blue()) / 2.0,
            (self.alpha() + other.alpha()) / 2.0,
        )
    }

    fn adjust_brightness(&mut self, brightness: Float) {
        self.array[0] = self.red() * brightness;
        self.array[1] = self.green() * brightness;
        self.array[2] = self.blue() * brightness;
    }

    fn div(&self, other: &Self) -> Float {
        if self == other {
            return 0.0;
        }

        let rd = self.red() - other.red();
        let gd = self.green() - other.green();
        let bd = self.blue() - other.blue();

        Float::sqrt(rd * rd + gd * gd + bd * bd)
    }
}

static BG_COLOR: Color = Color {
    array: [1.0, 1.0, 1.0, 1.0],
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Surface {
    color: Color,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Sphere {
    radius: Float,
    surface: Surface,
}

#[wasm_bindgen]
impl Sphere {
    #[wasm_bindgen(constructor)]
    pub fn new(radius: Float, color: Color) -> Self {
        Self {
            radius,
            surface: Surface { color },
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct World {
    spheres: Vec<(Vec<Float>, Sphere)>,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { spheres: vec![] }
    }

    #[wasm_bindgen]
    pub fn add_sphere(&mut self, pos: Vec<Float>, sphere: Sphere) {
        self.spheres.push((pos, sphere));
    }
}

#[derive(Debug)]
struct DimensionalWorld<V: Vector> {
    center: V,
    cam_pos: V,
    light_pos: V,
    spheres: Vec<(V, Sphere)>,
}

impl<V: Vector> DimensionalWorld<V> {
    fn from_world(world: &World, cam_pos: V) -> Self {
        Self {
            center: V::new(),
            cam_pos,
            light_pos: V::pad(&[-3.0, -3.0, 3.0], 3.0),
            spheres: world
                .spheres
                .iter()
                .map(|(position, s)| {
                    let position = V::pad(&position, 0.0);
                    (position, s.to_owned())
                })
                .collect(),
        }
    }
}

struct Intersection<V: Vector> {
    position: V,
    normal: V,
    distance: Float,
    surface: Surface,
}

fn test_sphere_intersection<V: Vector>(
    origin: &V,
    ray: &V,
    center: &V,
    sphere: &Sphere,
) -> Option<Intersection<V>> {
    let origin_to_sphere = *center - *origin;

    // len of ray to the point where it's closest to the sphere center
    let tc = ray.dot(&origin_to_sphere);

    if tc > 0.0 {
        let origin_to_sphere_len = origin_to_sphere.length();

        // center of sphere to ray
        let d = Float::sqrt(origin_to_sphere_len * origin_to_sphere_len - tc * tc);

        // if we hit the sphere
        if d < sphere.radius {
            // length from intersection to the point where d hits the ray (i.e. end of tc)
            let t1c = Float::sqrt(sphere.radius * sphere.radius - d * d);

            // length to first intersection
            let tc1 = tc - t1c;

            // point of first intersection on the ray
            let first_intersection = *ray * tc1;
            let hit = *origin + first_intersection;

            return Some(Intersection {
                position: hit,
                normal: (hit - *center).normalize(),
                distance: (hit - *origin).length(),
                surface: sphere.surface.clone(),
            });
        }
    }

    None
}

fn get_all_intersections<V: Vector>(
    world: &DimensionalWorld<V>,
    origin: &V,
    ray: &V,
) -> Vec<Intersection<V>> {
    let mut all = vec![];
    for (position, sphere) in &world.spheres {
        if let Some(intersection) = test_sphere_intersection(origin, ray, &position, sphere) {
            all.push(intersection)
        }
    }

    all.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

    all
}

fn trace<V: Vector>(world: &DimensionalWorld<V>, cam_pos: &V, ray: &V, light_pos: &V) -> Color {
    let all = get_all_intersections(world, &cam_pos, &ray);

    let mut color = BG_COLOR;
    for hit in all {
        let hit_to_light = (*light_pos - hit.position).normalize();
        let angle = hit.normal.dot(&hit_to_light);
        let mut hit_color = hit.surface.color;

        hit_color.adjust_brightness(angle);

        let shadow_casters = get_all_intersections(world, &hit.position, &hit_to_light);
        if shadow_casters.len() > 1 {
            hit_color.adjust_brightness(0.5);
        }

        // color = hit_color;
        color.apply(&hit_color);
    }

    color
}

fn sample<V: Vector>(world: &DimensionalWorld<V>, rel_x: Float, rel_y: Float) -> Color {
    let zoom = 1.4;
    let cam_dir = (world.center - world.cam_pos).normalize();
    let cam_dir_ort = V::pad(&[-cam_dir.components()[1], cam_dir.components()[0]], 0.0);

    let centered_x = rel_x * 2.0 - 1.0;
    let centered_y = rel_y * 2.0 - 1.0;
    let pos_on_sensor_x = cam_dir_ort * centered_x;
    let pos_on_sensor_y = V::pad(&[0.0, 0.0, 1.0], 0.0) * centered_y;
    let pos_on_sensor = pos_on_sensor_x + pos_on_sensor_y;

    let ray = (cam_dir * zoom + pos_on_sensor).normalize();

    trace(world, &world.cam_pos, &ray, &world.light_pos)
}

fn set_px(
    data: &mut wasm_bindgen::Clamped<Vec<u8>>,
    width: isize,
    x: isize,
    y: isize,
    color: ColorInt,
) {
    let index = ((x + y * width) * 4) as usize;

    data[index + 0] = color[0];
    data[index + 1] = color[1];
    data[index + 2] = color[2];
    data[index + 3] = 255; // color[3];
}

fn get_px(data: &wasm_bindgen::Clamped<Vec<u8>>, width: isize, x: isize, y: isize) -> ColorInt {
    let index = ((x + y * width) * 4) as usize;

    [
        data[index + 0],
        data[index + 1],
        data[index + 2],
        data[index + 3],
    ]
}

fn get_px_checked(
    data: &wasm_bindgen::Clamped<Vec<u8>>,
    width: isize,
    x: isize,
    y: isize,
) -> Option<ColorInt> {
    let index = ((x + y * width) * 4) as usize;

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

fn find_max_deviation(
    center: Color,
    top: Option<Color>,
    right: Option<Color>,
    bottom: Option<Color>,
    left: Option<Color>,
) -> Float {
    let mut div = 0.0;

    if let Some(top) = top {
        div = Float::max(div, center.div(&top));
    }
    if let Some(right) = right {
        div = Float::max(div, center.div(&right));
    }
    if let Some(bottom) = bottom {
        div = Float::max(div, center.div(&bottom));
    }
    if let Some(left) = left {
        div = Float::max(div, center.div(&left));
    }

    div
}

fn init_sample_grid<V: Vector>(
    data: &mut wasm_bindgen::Clamped<Vec<u8>>,
    world: &DimensionalWorld<V>,
    width: isize,
    height: isize,
    min_canvas_dim: Float,
    step: isize,
) {
    let offset_x = (min_canvas_dim - width as Float) / 2.0;
    let offset_y = (min_canvas_dim - height as Float) / 2.0;

    let step_offset = step / 2;
    for step_y in (step_offset..height).step_by(step as usize) {
        let rel_y = 1.0 - (step_y as Float + offset_y) / min_canvas_dim;

        for step_x in (step_offset..width).step_by(step as usize) {
            let rel_x = (step_x as Float + offset_x) / min_canvas_dim;

            let color = sample::<V>(&world, rel_x, rel_y).to_int();
            set_px(data, width, step_x, step_y, color);
        }
    }
}

fn fill_sample_grid<V: Vector>(
    data: &mut wasm_bindgen::Clamped<Vec<u8>>,
    world: &DimensionalWorld<V>,
    width: isize,
    height: isize,
    min_canvas_dim: Float,
    step: isize,
    substep: isize,
    deviation_threshold: Float,
) {
    let offset_x = (min_canvas_dim - width as Float) / 2.0;
    let offset_y = (min_canvas_dim - height as Float) / 2.0;

    // NOTE: offset is floored!
    let step_offset = step / 2;
    let substep_offset = substep / 2;

    assert!(width % step == 0);
    assert!(height % step == 0);

    for step_y in (step_offset..height).step_by(step as usize) {
        let substep_range_y = ((step_y - step_offset + substep_offset)..(step_y + step_offset + 1))
            .step_by(substep as usize);

        for step_x in (step_offset..width).step_by(step as usize) {
            let substep_range_x = ((step_x - step_offset + substep_offset)
                ..(step_x + step_offset + 1))
                .step_by(substep as usize);

            let center_int = get_px(&data, width, step_x, step_y);
            let center = Color::from_int(&center_int);
            let top = get_px_checked(&data, width, step_x, step_y - step)
                .as_ref()
                .map(Color::from_int);
            let right = get_px_checked(&data, width, step_x + step, step_y)
                .as_ref()
                .map(Color::from_int);
            let bottom = get_px_checked(&data, width, step_x, step_y + step)
                .as_ref()
                .map(Color::from_int);
            let left = get_px_checked(&data, width, step_x - step, step_y)
                .as_ref()
                .map(Color::from_int);

            let max_div = find_max_deviation(center, top, bottom, left, right);
            let resample = max_div > deviation_threshold;

            for substep_y in substep_range_y.clone() {
                let rel_y = 1.0 - (substep_y as Float + offset_y) / min_canvas_dim;

                for substep_x in substep_range_x.clone() {
                    if substep_x == step_x && substep_y == step_y {
                        // center is already sampled
                        continue;
                    }

                    let rel_x = (substep_x as Float + offset_x) / min_canvas_dim;

                    let color = if resample {
                        sample::<V>(&world, rel_x, rel_y).to_int()
                    } else {
                        center_int
                    };

                    set_px(data, width, substep_x, substep_y, color);
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn update(
    mut data: wasm_bindgen::Clamped<Vec<u8>>,
    world: &World,
    cam_pos: Vec<Float>,
    width: isize,
    height: isize,
    min_canvas_dim: Float,
) -> wasm_bindgen::Clamped<Vec<u8>> {
    type V = NdVec<3>;

    // let cam_pos = V::pad(&[-4.0, -5.0], 1.0);
    let cam_pos = V::pad(&cam_pos, 2.0);
    let world = DimensionalWorld::from_world(world, cam_pos);

    // init_sample_grid::<V>(&mut data, &world, width, height, min_canvas_dim, 1);
    // fill_sample_grid::<V>(&mut data, &world, width, height, min_canvas_dim, 27, 1, 0.1);

    init_sample_grid::<V>(&mut data, &world, width, height, min_canvas_dim, 27);
    fill_sample_grid::<V>(&mut data, &world, width, height, min_canvas_dim, 27, 9, 0.1);
    fill_sample_grid::<V>(&mut data, &world, width, height, min_canvas_dim, 9, 3, 0.1);
    fill_sample_grid::<V>(&mut data, &world, width, height, min_canvas_dim, 3, 1, 0.1);

    data
}

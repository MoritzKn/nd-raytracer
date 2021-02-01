extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

type Context = web_sys::CanvasRenderingContext2d;

type Float = f32;

trait Vector:
    Sized
    + Copy
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

#[derive(Clone, Copy)]
pub struct NdVec<const L: usize> {
    components: [Float; L],
}

impl<const L: usize> NdVec<L> {
    fn from_slice(components: [Float; L]) -> Self {
        Self { components }
    }
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
        println!("hi");
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
#[derive(Clone, Copy)]
pub struct Color {
    array: [u8; 4],
}

#[wasm_bindgen]
impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            array: [r, g, b, a],
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            array: [r, g, b, 255],
        }
    }
}

// impl Vector for Color {
//     fn new() -> Self {
//         Self(NdVec::new())
//     }

//     fn from_iter(iter: impl Iterator<Item = Float>) -> Self {
//         Self(NdVec::from_iter(iter))
//     }

//     fn pad(base: &[Float], d: Float) -> Self {
//         Self(NdVec::pad(base, d))
//     }

//     fn components(&self) -> &[Float] {
//         self.0.components()
//     }
// }

#[wasm_bindgen]
pub struct Surface {
    color: Color,
}

#[wasm_bindgen]
pub struct Sphere {
    radius: f64,
    surface: Surface,
}

#[wasm_bindgen]
impl Sphere {
    #[wasm_bindgen(constructor)]
    pub fn new(radius: f64, color: Color) -> Self {
        Self {
            radius,
            surface: Surface { color },
        }
    }
}

#[wasm_bindgen]
pub struct World {
    spheres: Vec<Sphere>,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { spheres: vec![] }
    }

    #[wasm_bindgen]
    pub fn add_sphere(&mut self, s: Sphere) {
        self.spheres.push(s);
    }
}

fn trace<V: Vector>(wold: &World, cam_pos: V, ray: V, light_pos: V) -> Color {
    Color::rgba(20, 10, 10, 255)
}

fn sample<V: Vector>(wold: &World, x: Float, y: Float) -> Color {
    let cam_pos = V::pad(&[-5.0], 0.0);
    let light_pos = V::pad(&[-2.0, -2.0], 0.0);

    let center = V::new();

    let x_centered = x * 2.0 - 1.0;
    let y_centered = y * 2.0 - 1.0;

    let cam_dir = (center - cam_pos).normalize();
    let cam_dir_ort = V::pad(&[-cam_dir.components()[1], cam_dir.components()[0]], 0.0);
    let pos_on_sensor_x = cam_dir_ort * x_centered;
    let pos_on_sensor_y = V::pad(&[0.0, 0.0, 1.0], 0.0) * y_centered;
    let pos_on_sensor = pos_on_sensor_x * pos_on_sensor_y;

    let ray = (cam_dir + pos_on_sensor).normalize();

    trace(wold, cam_pos, ray, light_pos)
}

#[wasm_bindgen]
pub fn sample_3(wold: &World, x: Float, y: Float) -> Color {
    sample::<NdVec<3>>(wold, x, y)
}

#[wasm_bindgen]
pub fn update(
    world: &World,
    width: usize,
    height: usize,
    min_canvas_dim: usize,
    mut data: wasm_bindgen::Clamped<Vec<u8>>,
) -> wasm_bindgen::Clamped<Vec<u8>> {
    let offset_y = (min_canvas_dim - height) / 2;
    let offset_x = (min_canvas_dim - width) / 2;

    for y in 0..height {
        let rel_y = 1.0 - (y + offset_y) as Float / min_canvas_dim as Float;
        for x in 0..width {
            let rel_x = (x + offset_x) as Float / min_canvas_dim as Float;

            let color = sample_3(world, rel_x, rel_y).array;
            let index = (y * width + x) * 4;
            data[index + 0] = color[0];
            data[index + 1] = color[1];
            data[index + 2] = color[2];
            data[index + 3] = color[3];
        }
    }

    data
}

#[wasm_bindgen]
pub fn draw(canvas: web_sys::HtmlCanvasElement, ctx: Context) {
    let height = canvas.height();
    let width = canvas.width();
    let image_data = ctx
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap();

    let mut data = image_data.data();
    let px_count = data.len() / 4;
    for i in 0..px_count {
        let i = i * 4;
        data[i + 0] = 255;
        data[i + 1] = 0;
        data[i + 2] = 0;
        data[i + 3] = 255;
    }

    let out: wasm_bindgen::Clamped<&[u8]> = wasm_bindgen::Clamped(&data);
    let out = web_sys::ImageData::new_with_u8_clamped_array(out, width).unwrap();

    ctx.put_image_data(&out, 0.0, 0.0).unwrap();

    // // Draw the outer circle.
    // ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();
    //
    // // Draw the mouth.
    // ctx.move_to(110.0, 75.0);
    // ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();
    //
    // // Draw the left eye.
    // ctx.move_to(65.0, 65.0);
    // ctx.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();
    //
    // // Draw the right eye.
    // ctx.move_to(95.0, 65.0);
    // ctx.arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();
    //
    // ctx.stroke();
}

use crate::color::Color;
use crate::vector::Vector;
use crate::world::{Light, Sphere, Surface, World};
use crate::Float;

pub static BG_COLOR: Color = Color {
    array: [1.0, 1.0, 1.0, 1.0],
};

#[derive(Debug)]
pub struct DimensionalWorld<V: Vector> {
    center: V,
    cam_pos: V,
    lights: Vec<(V, Light)>,
    spheres: Vec<(V, Sphere)>,
}

impl<V: Vector> DimensionalWorld<V> {
    pub fn from_world(world: &World, cam_pos: V) -> Self {
        Self {
            center: V::new(),
            cam_pos,
            lights: world
                .lights
                .iter()
                .map(|(position, s)| {
                    let position = V::pad(&position, 0.0);
                    (position, s.to_owned())
                })
                .collect(),
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
    let mut all = Vec::with_capacity(world.spheres.len());
    for (position, sphere) in &world.spheres {
        if let Some(intersection) = test_sphere_intersection(origin, ray, &position, sphere) {
            all.push(intersection)
        }
    }

    all.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

    all
}

fn get_light_color<V: Vector>(
    mut light_color: Color,
    shadow_casters: Vec<Intersection<V>>,
) -> Color {
    for sc in shadow_casters {
        light_color.apply(&sc.surface.color.normalize());
        light_color.adjust_brightness(1.0 - sc.surface.color.alpha())
    }

    light_color
}

fn trace<V: Vector>(
    world: &DimensionalWorld<V>,
    cam_pos: &V,
    ray: &V,
    reflection_bounces: usize,
) -> Color {
    let all = get_all_intersections(world, &cam_pos, &ray);

    let mut color = BG_COLOR;
    for (index, hit) in all.iter().enumerate() {
        if index != all.len() - 1 {
            let next_alpha = all[index + 1].surface.color.alpha();
            if next_alpha == 1.0 {
                // Not show anyways
                continue;
            }
        }

        let mut hit_color = hit.surface.color;

        let mut lights_color = Color::rgba(0.3, 0.3, 0.3, 1.0);
        for (light_pos, light) in &world.lights {
            let hit_to_light = (*light_pos - hit.position).normalize();

            let mut color = get_light_color(
                light.color,
                get_all_intersections(world, &hit.position, &hit_to_light),
            );

            let angle = hit.normal.dot(&hit_to_light);
            // The more the brightness of the light is influenced by the angle the softer curves will look
            let brightness = Float::max(angle * 0.8 + 0.2, 0.0);
            color.adjust_brightness(brightness);

            lights_color.combine(&color);
        }

        hit_color.apply(&lights_color);

        if reflection_bounces > 0 && hit.surface.reflection > 0.0 {
            let ray_reflection = *ray - (hit.normal * 2.0 * ray.dot(&hit.normal));
            let mut color = trace(
                world,
                &hit.position,
                &ray_reflection,
                reflection_bounces - 1,
            );
            color.set_alpha(hit.surface.reflection);
            hit_color.mix(&color);
        }

        color.mix(&hit_color);
    }

    color
}

pub fn sample<V: Vector>(world: &DimensionalWorld<V>, rel_x: Float, rel_y: Float) -> Color {
    let zoom = 1.4;
    let cam_dir = (world.center - world.cam_pos).normalize();
    let cam_dir_ort = V::pad(&[-cam_dir.components()[1], cam_dir.components()[0]], 0.0);

    let centered_x = rel_x * 2.0 - 1.0;
    let centered_y = rel_y * 2.0 - 1.0;
    let pos_on_sensor_x = cam_dir_ort * centered_x;
    let pos_on_sensor_y = V::pad(&[0.0, 0.0, 1.0], 0.0) * centered_y;
    let pos_on_sensor = pos_on_sensor_x + pos_on_sensor_y;

    let ray = (cam_dir * zoom + pos_on_sensor).normalize();

    let reflection_bounces = 2;
    trace(world, &world.cam_pos, &ray, reflection_bounces)
}

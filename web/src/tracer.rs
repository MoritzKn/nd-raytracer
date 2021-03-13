use crate::color::Color;
use crate::world::{Cube, Light, Sphere, Surface, World};
use ndrt_lib::{Float, Vector};

pub static BG_COLOR: Color = Color {
    array: [1.0, 1.0, 1.0, 1.0],
};

#[derive(Debug)]
pub struct DimensionalWorld<V: Vector> {
    center: V,
    cam_pos: V,
    lights: Vec<(V, Light)>,
    spheres: Vec<(V, Sphere)>,
    cubes: Vec<(V, Cube)>,
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
            cubes: world
                .cubes
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

fn axis_normalize<V: Vector>(v: &V) -> V {
    let mut most_dominat: Float = 0.0;

    for comp in v.components() {
        let comp = *comp;
        if comp.abs() > most_dominat.abs() {
            most_dominat = comp
        }
    }

    let result = v.components().iter().cloned().map(|comp| {
        if comp == most_dominat {
            most_dominat.signum()
        } else {
            0.0
        }
    });

    V::from_iter(result)
}

fn test_cube_intersection<V: Vector>(
    origin: &V,
    ray: &V,
    center: &V,
    cube: &Cube,
) -> (Option<Intersection<V>>, Option<Intersection<V>>) {
    let half_size = cube.size / 2.0;
    let bounds = center
        .components()
        .iter()
        .map(|c| (c + half_size, c - half_size));

    let mut clamped_min = -Float::INFINITY;
    let mut clamped_max = Float::INFINITY;

    for ((origin_comp, ray_comp), (bounds_min, bounds_max)) in origin
        .components()
        .into_iter()
        .zip(ray.components())
        .zip(bounds)
    {
        if *ray_comp != 0.0 {
            let tx1 = (bounds_min - origin_comp) / ray_comp;
            let tx2 = (bounds_max - origin_comp) / ray_comp;

            clamped_min = Float::max(clamped_min, Float::min(tx1, tx2));
            clamped_max = Float::min(clamped_max, Float::max(tx1, tx2));
        }
    }

    // cube is in line of the ray, no necessary in front through
    let in_line = clamped_min < clamped_max;

    let intersection_in = if in_line && clamped_min > 0.0 {
        let hit_in = origin.add(&ray.mul_scalar(clamped_min));
        Some(Intersection {
            position: hit_in,
            normal: axis_normalize(&(hit_in - *center)),
            distance: (hit_in - *origin).length(),
            surface: cube.surface.clone(),
        })
    } else {
        None
    };

    let intersection_out = if in_line && clamped_max > 0.0 {
        // let hit_out = origin.add(&ray.mul_scalar(clamped_max));
        // Some(Intersection {
        //     position: hit_out,
        //     normal: axis_normalize(&(*center - hit_out)),
        //     distance: (hit_out - *origin).length(),
        //     surface: cube.surface.clone(),
        // })

        None
    } else {
        None
    };

    (intersection_in, intersection_out)
}

fn get_all_intersections<V: Vector>(
    world: &DimensionalWorld<V>,
    origin: &V,
    ray: &V,
) -> Vec<Intersection<V>> {
    let mut all = Vec::with_capacity(8);

    for (position, sphere) in &world.spheres {
        if let Some(intersection) = test_sphere_intersection(origin, ray, &position, sphere) {
            all.push(intersection)
        }
    }

    for (position, cube) in &world.cubes {
        let (intersection_in, intersection_out) =
            test_cube_intersection(origin, ray, &position, cube);

        if let Some(intersection) = intersection_in {
            all.push(intersection);
        }
        // if let Some(intersection) = intersection_out {
        //     all.push(intersection);
        // }
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
        if let Some(next) = all.get(index + 1) {
            let next_alpha = next.surface.color.alpha();
            if next_alpha == 1.0 {
                // Hit is not shown anyways
                continue;
            }
        }

        let mut hit_color = hit.surface.color;

        // Ambient light color
        let mut lights_color = Color::rgba(0.3, 0.3, 0.3, 1.0);

        for (light_pos, light) in &world.lights {
            let hit_to_light = (*light_pos - hit.position).normalize();

            let shadow_casters = get_all_intersections(world, &hit.position, &hit_to_light);
            let mut color = get_light_color(light.color, shadow_casters);

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

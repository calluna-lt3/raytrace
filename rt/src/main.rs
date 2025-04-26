use ppm::PPM;
use vector3d::Vector3D;

struct Light {
    loc: Vector3D,
    color: (f64, f64, f64),
    intensity: f64,
}

impl Light {
    fn new(loc: Vector3D, color: (f64, f64, f64), intensity: f64) -> Self {
        Self { loc, color, intensity }
    }
}

struct Camera {
    loc: Vector3D,
    dir: Vector3D,
    dist: f64,
}

impl Camera {
    fn new(dist: f64, loc: Vector3D) -> Self {
        let dir = loc.normalize();
        let dir = dir.scale(-1.);
        Self { loc, dist, dir }
    }
}

struct Scene {
    cam: Camera,
    light: Light,
}

impl Scene {
    fn new(cam: Camera, light: Light) -> Self {
        Self {
            cam, light,
        }
    }

    fn point_to_vec(cam: &Camera, x: i32, y: i32) -> Vector3D {
        let d = cam.dist as f64;
        let v1 = Vector3D::new(x.into(), y.into(), -d);
        let v2 = v1.add(&cam.dir.scale(d));
        v2.sub(&cam.loc)
    }

    fn sphere_intersects(origin: &Vector3D, direction: &Vector3D, sphere: &Sphere) -> Option<(f64, f64)> {
        let r = sphere.radius;
        let a = direction.dot(&direction);
        let b = 2. * direction.dot(&origin);
        let c = origin.dot(&origin) - f64::powi(r, 2);
        solve_quadratic(a, b, c)
    }

}

struct Plane {
    stride: usize,
    origin: (i32, i32),
    data: Box<[(f64, f64, f64)]>,
}

impl Plane {
    fn new(w: usize, h: usize) -> Self {
        let origin = (TryInto::<i32>::try_into(h).expect("height too big") / 2,
                      TryInto::<i32>::try_into(w).expect("width too big") / 2);
        let data = vec![(1., 1., 1.); w * h].into_boxed_slice();
        Self {
            origin, data,
            stride: w,
        }
    }

    fn point(&mut self, x: i32, y: i32) -> Option<&mut (f64, f64, f64)> {
        let stride: i32 = self.stride.try_into().unwrap();
        let w: i32 = self.width().try_into().unwrap();
        let h: i32 = self.height().try_into().unwrap();
        let range = (-w / 2, w / 2);
        let domain = (-h / 2, h / 2);

        if x < range.0 || x > range.1 || y < domain.0 || y > domain.1 {
            None
        } else {
            // make this row wise
            let pos: usize = TryInto::<usize>::try_into(stride * (self.origin.0 - y) + (self.origin.1 + x)).unwrap();
            Some(&mut self.data[pos])
        }
    }

    fn width(&self) -> usize {
        self.stride
    }

    fn height(&self) -> usize {
        self.data.len() / self.stride
    }

    #[allow(unused)] fn print(&self) {
        println!("[");
        for (x, val) in self.data.iter().enumerate() {
            if x % self.stride == 0 { print!("  ["); }
            if val.0 == 1. {
                print!(" - ");
            } else {
                print!(" {} ", val.0);
            }
            if x % self.stride + 1 == self.stride { println!("]"); }
        }
        println!("]");
    }

    fn render(&mut self, scene: &Scene, objects: &Vec<Sphere>) {
        let d = &scene.cam.dist;
        let light = &scene.light;

        let w: i32 = self.width().try_into().unwrap();
        let h: i32 = self.height().try_into().unwrap();
        let range = (-w / 2, w / 2);
        let domain = (-h / 2, h / 2);

        for x in range.0..=range.1 {
            for y in domain.0..=domain.1 {
                if let Some(p) = &mut self.point(x, y) {
                    let dir = Scene::point_to_vec(&scene.cam, x, y).normalize();

                    let mut obj_visible: Option<(f64, (f64, f64, f64))> = None;
                    for sphere in objects {
                        // see if the ray hits obj
                        let origin = scene.cam.loc.sub(&sphere.loc);
                        let (t0, t1) = match Scene::sphere_intersects(&origin, &dir, &sphere) {
                            Some(ts) => {
                                ts
                            },
                            None => continue,
                        };

                        // tangent
                        if t0 == t1 { continue; }

                        // choose lowest distance from cam -> sphere
                        let visible = f64::min(t0, t1);

                        // both not visible
                        if visible < 0. { continue; }

                        let p = dir.scale(visible).add(&Vector3D::new(0., 0., -d));
                        let n = p.normalize();
                        let r = light.loc.sub(&p).normalize();
                        let collinear = r.dot(&n);

                        // if hit, dont render light on it
                        let mut light_visible = true;
                        for sphere2 in objects {
                            if sphere2 == sphere { continue; }
                            if let Some((p0, p1)) = Scene::sphere_intersects(&p, &r, &sphere2) {
                                if p0 > 0. && p1 > 0. { light_visible = false; }
                            }
                        }

                        let color: (f64, f64, f64) = if light_visible {
                            let r = sphere.color.0 * light.color.0 * light.intensity * collinear;
                            let g = sphere.color.1 * light.color.1 * light.intensity * collinear;
                            let b = sphere.color.2 * light.color.2 * light.intensity * collinear;
                            (r, g, b)
                        } else {
                            (0., 0., 0.)
                        };


                        let cur_dist = dir.scale(visible).magnitude();
                        match obj_visible {
                            Some((dist, _)) => {
                                if dist > cur_dist {
                                    obj_visible = Some((cur_dist, color))
                                }
                            },
                            None => { obj_visible = Some((cur_dist, color))},
                        };
                    }

                    if let Some((_, color)) = obj_visible {
                        **p = color;
                    }
                }
            }
        }
    }
}

#[derive(PartialEq)]
struct Sphere {
    loc: Vector3D,
    radius: f64,
    color: (f64, f64, f64),
}

impl Sphere {
    fn new(x: f64, y: f64, z: f64, radius: f64, color: (f64, f64, f64)) -> Self {
        let loc = Vector3D::new(x, y, z);
        Self { loc, radius, color }
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discr = b.powf(2.) - (4. * a * c);
    if discr < 0. {
        None
    } else if discr == 0. {
        let res = -0.5 * b / a;
        Some((res, res))
    } else {
        let q = if b > 0. {
            -0.5 * (b + f64::sqrt(discr))
        } else {
            -0.5 * (b - f64::sqrt(discr))
        };

        Some((q / a, c / q))
    }
}


use std::ops::{Div, Sub};
fn _normalize<T>(x: T, min: T, max: T) -> <<T as Sub>::Output as Div>::Output
where
    T: Div + Sub + Copy, <T as Sub>::Output: Div
{
    (x - min) / (max - min)
}

fn _flatten_tuple<T>(tuple: (T, T, T)) -> [T; 3] {
    [tuple.0, tuple.1, tuple.2]
}

fn main() {
    let w = 1201;
    let h = 1201;
    let d = 1000.;
    let cam = Camera::new(d, Vector3D::new(0., 0., -(d as f64)));
    let light = Light::new(Vector3D::new(-1000., 1000., -1000.), (1., 1., 1.), 1.);
    let scene = Scene::new(cam, light);

    let objects = vec![
        Sphere::new(0., 0., 0., 200., (1., 0., 0.)),
        Sphere::new(-200., -200., -100., 100., (0., 1., 0.)),
    ];

    let mut plane = Plane::new(w, h);
    //eprintln!("width = {}, height = {}", plane.width(), plane.height());
    //eprintln!("origin = ({}, {})", plane.origin.0, plane.origin.1);
    plane.render(&scene, &objects);
    //plane.print();

    //eprintln!("range = {range:?}, domain = {domain:?}");
    //eprintln!("origin = {:?}", plane.origin);

    // converts from 0..1 => 0..255
    let data: Box<[(u8, u8, u8)]> = plane.data.iter().map(|c| {
        let r = (c.0 * 255.) as u8;
        let g = (c.1 * 255.) as u8;
        let b = (c.2 * 255.) as u8;
        (r, g, b)
    }).collect();

    let ppm = PPM::new(w.try_into().unwrap(), h.try_into().unwrap(), data);
    ppm.print();
}

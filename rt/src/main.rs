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
    plane: Plane,
}

impl Scene {
    fn new(cam: Camera, light: Light, plane: Plane) -> Self {
        Self {
            cam, plane, light,
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

    fn render(&mut self, objects: &Vec<Sphere>) {
        let plane = &mut self.plane;
        let d = &self.cam.dist;
        let light = &self.light;

        let w: i32 = plane.w.try_into().unwrap();
        let h: i32 = plane.h.try_into().unwrap();
        let range = (-w / 2, w / 2);
        let domain = (-h / 2, h / 2);

        for x in range.0..=range.1 {
            for y in domain.0..=domain.1 {
                if let Some(p) = &mut plane.point(x, y) {
                    let dir = Scene::point_to_vec(&self.cam, x, y).normalize();

                    let mut obj_visible: Option<(f64, (f64, f64, f64))> = None;
                    for sphere in objects {
                        // see if the ray hits obj
                        let origin = self.cam.loc.sub(&sphere.loc);
                        let (t0, t1) = match Scene::sphere_intersects(&origin, &dir, &sphere) {
                            Some(ts) => ts,
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
                            if let Some((p0, p1)) = Scene::sphere_intersects(&p, &r, &sphere2) {
                                if p0 > 0. || p1 > 0. {
                                    light_visible = false;
                                }
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

struct Plane {
    w: usize,
    h: usize,
    origin: (i32, i32),
    data: Box<[Box<[(f64, f64, f64)]>]>,
}

impl Plane {
    fn new(w: usize, h: usize, ) -> Self {
        let oh: i32 = h.try_into().unwrap();
        let ow: i32 = w.try_into().unwrap();
        let origin = (oh / 2, ow / 2);
        let data = vec![vec![(1., 1., 1.); w].into_boxed_slice(); h].into_boxed_slice();
        Self { w, h, origin, data, }
    }

    fn point(&mut self, x: i32, y: i32) -> Option<&mut (f64, f64, f64)> {
        let yi: usize = (self.origin.0 - y).try_into().unwrap();
        let xi: usize = (self.origin.1 + x).try_into().unwrap();

        if yi >= self.data.len() || xi >= self.data[0].len() {
            None
        } else {
            Some(&mut self.data[yi][xi])
        }
    }


    #[allow(unused)]
    fn print(&self) {
        println!("[");
        for x in &self.data {
            print!("  [");
            for y in x {
                if y.0 == 1. {
                    print!(" - ");
                } else {
                    print!(" {} ", y.0);
                }
            }
            println!("]");
        }
        println!("]");
    }
}

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

fn main() {
    let w = 1600;
    let h = 1200;
    let d = 1000.;
    let cam = Camera::new(d, Vector3D::new(0., 0., -(d as f64)));
    let plane = Plane::new(w, h);
    let light = Light::new(Vector3D::new(-1000., 1000., -1000.), (1., 1., 1.), 1.);
    let mut scene = Scene::new(cam, light, plane);
    let objects = vec![
        Sphere::new(0., 0., -100., 200., (1., 0., 0.)),
        Sphere::new(0., 0., -400., 100., (0., 1., 0.)),
        Sphere::new(300., -300., 400., 100., (0., 0., 1.))
    ];
    scene.render(&objects);

    //eprintln!("range = {range:?}, domain = {domain:?}");
    //eprintln!("origin = {:?}", plane.origin);

    // converts 0 -> 1: 0 -> 255
    let data: Box<[Box<[(u8, u8, u8)]>]> = scene.plane.data.iter().map(|b| {
        b.iter().map(|v| {
            let r = (v.0 * 255.) as u8;
            let g = (v.1 * 255.) as u8;
            let b = (v.2 * 255.) as u8;
            (r, g, b)
        }).collect()
    }).collect();
    let ppm = PPM::new(w.try_into().unwrap(), h.try_into().unwrap(), data);
    ppm.print();
}

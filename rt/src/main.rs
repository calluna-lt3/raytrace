use ppm::PPM;
use vector3d::Vector3D;


struct Camera {
    loc: Vector3D,
    dir: Vector3D,
    dist: usize,
}

impl Camera {
    fn new(dist: usize, loc: Vector3D) -> Self {
        let dir = loc.normalize();
        let dir = dir.scale(-1.);
        Self { loc, dist, dir }
    }
}

struct Scene {
    cam: Camera,
    plane: Plane,
    objects: Vec<Sphere>,
}

impl Scene {
    fn new(cam: Camera, plane: Plane) -> Self {
        Self {
            cam, plane,
            objects: vec![],
        }
    }

    fn add(&mut self, obj: Sphere) {
        self.objects.push(obj);
    }

    fn point_to_vec(cam: &Camera, x: i32, y: i32) -> Vector3D {
        let d = cam.dist as f64;
        let v1 = Vector3D::new(x.into(), y.into(), -d);
        let v2 = v1.add(&cam.dir.scale(d));
        v2.sub(&cam.loc)
    }

    fn intersects(origin: &Vector3D, direction: &Vector3D, sphere: &Sphere) -> Option<(f64, f64)> {
        let r = sphere.radius;
        let a = direction.dot(&direction);
        let b = 2. * direction.dot(&origin);
        let c = origin.dot(&origin) - f64::powi(r, 2);
        solve_quadratic(a, b, c)
    }

    fn render(&mut self) {
        let plane = &mut self.plane;

        let w: i32 = plane.w.try_into().unwrap();
        let h: i32 = plane.h.try_into().unwrap();
        let range = (-w / 2, w / 2);
        let domain = (-h / 2, h / 2);

        for x in range.0..=range.1 {
            for y in domain.0..=domain.1 {
                if let Some(p) = &mut plane.point(x, y) {
                    let dir = Scene::point_to_vec(&self.cam, x, y).normalize();

                    // for each point, find an object in list of objs and see if it hits
                    let mut obj_visible: Option<(&Sphere, f64)> = None;
                    for sphere in &self.objects {
                        // see if the ray hits obj
                        let origin = self.cam.loc.sub(&sphere.loc);
                        let (t0, t1) = match Scene::intersects(&origin, &dir, &sphere) {
                            Some(ts) => ts,
                            None => continue,
                        };

                        // tangent
                        // if t0 == t1 { continue; }

                        // choose lowest distance from cam -> sphere
                        let visible = f64::min(t0, t1);

                        // both not visible
                        if visible < 0. { continue; }

                        let mag = dir.scale(visible).magnitude();
                        //eprintln!("({}, {}, {}) @ ({x}, {y}) => {mag}", sphere.color.0, sphere.color.1, sphere.color.2);

                        match obj_visible {
                            Some((_, d)) => {
                                if d > mag {
                                    obj_visible = Some((sphere, mag))
                                }
                            },
                            None => { obj_visible = Some((sphere, mag))},
                        };
                    }

                    if let Some((obj, _)) = obj_visible {
                        **p = (obj.color.0, obj.color.1, obj.color.2);
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
    data: Box<[Box<[(u8, u8, u8)]>]>,
}

impl Plane {
    fn new(w: usize, h: usize, ) -> Self {
        let oh: i32 = h.try_into().unwrap();
        let ow: i32 = w.try_into().unwrap();
        let origin = (oh / 2, ow / 2);
        let data = vec![vec![(255, 255, 255); w].into_boxed_slice(); h].into_boxed_slice();
        Self { w, h, origin, data, }
    }

    fn point(&mut self, x: i32, y: i32) -> Option<&mut (u8, u8, u8)> {
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
                if y.0 == 255 {
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
    color: (u8, u8, u8),
}

impl Sphere {
    fn new(x: f64, y: f64, z: f64, radius: f64, color: (u8, u8, u8)) -> Self {
        let loc = Vector3D::new(x, y, z);
        Self { loc, radius, color, }
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

fn main() {
    let w = 1801;
    let h = 1201;
    let d = 1000;
    let cam = Camera::new(d, Vector3D::new(0., 0., -(d as f64)));
    let plane = Plane::new(w, h);
    let mut scene = Scene::new(cam, plane);
    scene.add(Sphere::new(-100., 0., 0., 100., (255, 0, 0)));
    scene.add(Sphere::new(100., 0., 0., 100., (0, 255, 0)));
    scene.add(Sphere::new(0., 0., 0., 100., (0, 0, 255)));
    scene.render();

    //eprintln!("range = {range:?}, domain = {domain:?}");
    //eprintln!("origin = {:?}", plane.origin);

    let ppm = PPM::new(w.try_into().unwrap(), h.try_into().unwrap(), scene.plane.data);
    ppm.print();
}

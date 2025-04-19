use ppm::PPM;
use vector3d::Vector3D;

/*
struct Space<'a> {
    w: i32,
    h: i32,
    d: i32,
    origin: (i32, i32, i32),
    data: Box<[Box<[Box<[(u8, u8, u8)]>]>]>,
}

impl<'a> Space<'a> {
    fn new(w: i32, h: i32, d: i32) -> Self {
        let origin = (w / 2, h / 2, d/2);
        let data = vec![vec![vec![(0, 0, 0); h.try_into().unwrap()].into_boxed_slice(); w.try_into().unwrap()].into_boxed_slice(); d.try_into().unwrap()].into_boxed_slice();
        Self { w, h, d, origin, data }
    }

    fn point(&'a self, x: usize, y: i32, z: i32) -> &'a mut (i32, i32, i32) {
        self.data[x][y][z]
    }
}

fn sphere5(x: i32, y: i32, z: i32) -> i32 {
    x.pow(2) + y.pow(2) + z.pow(2) - i32::pow(5, 2)
}

fn hit_sphere(v: ((i32, i32, i32), (i32, i32, i32)), func: fn(i32, i32, i32) -> i32) -> bool {
    true
}
*/

fn _normalize(x: i32, min: i32, max: i32) -> f32 {
    (x as f32 - min as f32) / (max as f32 - min as f32)
}

fn circle(x: i32, y: i32, r: i32) -> i32 {
    x.pow(2) + y.pow(2) - r.pow(2)
}

fn in_circle(x: i32, y: i32, r: i32, func: fn(i32, i32, i32) -> i32) -> bool {
    let val = func(x, y, r);
    if val > 0 { false } else { true }
}

#[allow(unused)]
struct CameraInfo {
    origin: Vector3D,
    o_unit: Vector3D,
    d: usize,
}

impl CameraInfo {
    fn new(d: usize, origin: Vector3D) -> Self {
        let o_unit = origin.normalize();
        let o_unit = o_unit.scale(-1.);
        Self { origin, d, o_unit }
    }
}


// TODO, use domain and range to create the plane to make things make more sense
#[allow(unused)]
struct Plane {
    w: usize,
    h: usize,
    origin: (i32, i32),
    data: Box<[Box<[(u8, u8, u8)]>]>,
}

impl Plane {
    fn new(w: usize, h: usize) -> Self {
        let oh: i32 = h.try_into().unwrap();
        let ow: i32 = w.try_into().unwrap();
        let origin = (oh / 2, ow / 2);
        let data = vec![vec![(255, 255, 255); w].into_boxed_slice(); h].into_boxed_slice();
        Self { w, h, origin, data }
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

#[allow(unused)]
fn sphere(x: f64, y: f64, z: f64, radius: f64) {
    let v1 = Vector3D::new(x, y, z);
    let v2 = v1.mult(&v1);
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Result<(f64, f64), ()> {
    let discr = b.powf(2.) - (4. * a * c);
    if discr < 0. { return Err(()) }
    else if discr == 0. {
        let res = -0.5 * b / a;
        Ok((res, res))
    } else {
        let q = if b > 0. { -0.5 * (b + f64::sqrt(discr)) }
                     else { -0.5 * (b - f64::sqrt(discr)) };
        Ok((q / a, c / q))
    }
}

fn main() {
    let w = 801;
    let h = 601;
    let d = 31;
    let r = 30;
    let mut plane = Plane::new(w, h);
    let c_info = CameraInfo::new(d, Vector3D::new(0., 0., -(d as f64)));

    let w: i32 = w.try_into().unwrap();
    let h: i32 = h.try_into().unwrap();
    let range = (-w / 2, w / 2);
    let domain = (-h / 2, h / 2);

    //eprintln!("range = {range:?}, domain = {domain:?}");
    //eprintln!("origin = {:?}", plane.origin);

    for x in range.0..=range.1 {
        for y in domain.0..=domain.1 {
            if let Some(p) = plane.point(x, y) {
                let v1 = Vector3D::new(x.into(), y.into(), -(d as f64));
                let v2 = v1.add(&c_info.o_unit.scale(d as f64));
                let dir = v2.sub(&c_info.origin).normalize();

                let o = Vector3D::new(0., 0., -(d as f64));
                let a = dir.dot(&dir);
                let b = 2. * dir.dot(&o);
                let c = o.dot(&o) - f64::powi(r as f64, 2);

                let (t0, t1) = match solve_quadratic(a, b, c) {
                    Err(_) => { continue },
                    Ok(pt) => { pt },
                };

                // tangent (not rendering)
                if t0 == t1 { }

                // which is visible
                let visible = if t0 > t1 {
                    t0
                } else {
                    t1
                };

                // both not visible
                if visible < 0. { continue; }

                *p = (0, 0, 0);
            }
        }
    }

    let ppm = PPM::new(w.try_into().unwrap(), h.try_into().unwrap(), plane.data);
    ppm.print();
}

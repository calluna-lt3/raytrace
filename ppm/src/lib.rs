#![allow(unused_variables, dead_code)]

use std::io::{stdout, Write};

#[allow(unused_variables, dead_code)]
pub struct PPM {
    m_num: String,
    w: usize,
    h: usize,
    maxcval: usize,
    data: Box<[(u8, u8, u8)]>,
}

impl PPM {
    pub fn new(w: usize, h: usize, data: Box<[(u8, u8, u8)]>) -> Self {
        let m_num = String::from("P6");
        let maxcval = 255;
        Self { m_num, w, h, maxcval, data, }
    }

    pub fn print(&self) {
        let header = format!(r#"{magic}
{w} {h} {cval}
"#, magic = self.m_num, w = self.w, h = self.h, cval = self.maxcval);
        print!("{header}");
        let mut stdout = stdout().lock();
        self.data.iter().for_each(|x| {
            stdout.write_all(&x.0.to_be_bytes()).unwrap();
            stdout.write_all(&x.1.to_be_bytes()).unwrap();
            stdout.write_all(&x.2.to_be_bytes()).unwrap();
        });
        stdout.flush().unwrap();
    }
}

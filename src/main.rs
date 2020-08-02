extern crate ndarray;

use std::io::Write;
use std::ops;

struct Pixel {
  r: f32,
  g: f32,
  b: f32,
}
#[derive(Copy, Clone)]
struct Vec3f {
  x: f32,
  y: f32,
  z: f32,
}

impl Vec3f {
  fn new(x: f32, y: f32, z: f32) -> Self {
    return Self { x: x, y: y, z: z };
  }

  fn multiply_idk(&self, other: Vec3f) -> f32 {
    return self.x * other.x + self.y * other.y + self.z * other.z;
  }

  fn normalize(mut self) -> Self {
    self = self * (1.0 / self.norm());
    return self;
  }

  fn norm(&self) -> f32 {
    return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
  }
}

impl ops::Sub<Vec3f> for Vec3f {
  type Output = Self;

  fn sub(self, _rhs: Vec3f) -> Self::Output {
    return Vec3f {
      x: self.x - _rhs.x,
      y: self.y - _rhs.y,
      z: self.z - _rhs.z,
    };
  }
}

impl ops::Mul<f32> for Vec3f {
  type Output = Vec3f;

  fn mul(self, _rhs: f32) -> Self::Output {
    return Vec3f {
      x: self.x * _rhs,
      y: self.y * _rhs,
      z: self.z * _rhs,
    };
  }
}

impl ops::Mul<Vec3f> for Vec3f {
  type Output = Vec3f;

  fn mul(self, _rhs: Vec3f) -> Self::Output {
    return Vec3f {
      x: self.x * _rhs.x,
      y: self.y * _rhs.y,
      z: self.z * _rhs.z,
    };
  }
}

struct Sphere {
  center: Vec3f,
  radius: f32,
}

impl Sphere {
  fn new(c: Vec3f, r: f32) -> Self {
    return Self {
      center: c,
      radius: r,
    };
  }

  fn ray_intersect(&self, orig: Vec3f, dir: Vec3f, mut t0: f32) -> bool {
    let L: Vec3f = self.center - orig;
    let tca: f32 = L.multiply_idk(dir);
    let d2: f32 = L.multiply_idk(L) - tca * tca;
    if d2 > self.radius * self.radius {
      return false;
    }
    let thc: f32 = (self.radius * self.radius - d2).sqrt();
    t0 = tca - thc;
    let t1: f32 = tca + thc;
    if t0 < 0.0 {
      t0 = t1;
    }
    if t0 < 0.0 {
      return false;
    }
    return true;
  }
}

fn cast_ray(orig: Vec3f, dir: Vec3f, sphere: &Sphere) -> Pixel {
  let dist: f32 = std::f32::MAX;
  if !sphere.ray_intersect(orig, dir, dist) {
    return Pixel {
      r: 0.2,
      g: 0.7,
      b: 0.8,
    };
  }

  return Pixel {
    r: 0.4,
    g: 0.4,
    b: 0.3,
  };
}

fn render(sphere: &Sphere) {
  let width = 1024;
  let height = 768;
  let fov = std::f32::consts::FRAC_PI_2;

  let mut test: std::vec::Vec<Pixel> = Vec::with_capacity(width * height);

  for j in 0..height {
    for i in 0..width {
      let x =
        (2.0 * ((i as f32) + 0.5) / (width as f32) - 1.0) * (fov / 2.0).tan() * (width as f32)
          / (height as f32);
      let y =
        -(2.0 * ((j as f32) + 0.5) / (width as f32) - 1.0) * (fov / 2.0).tan() * (width as f32)
          / (height as f32);
      let dir: Vec3f = Vec3f::new(x, y, -1.0).normalize();
      test.push(cast_ray(Vec3f::new(0.0, 0.0, 0.0), dir, sphere));
    }
  }

  let mut file = std::fs::File::create("out.ppm").unwrap();
  let header = format!("P6\n{} {}\n255\n", width, height);
  file.write_all(header.as_bytes()).unwrap();
  for i in 0..(width * height) {
    file
      .write(&[(255.0 * test[i].r.min(1.0).max(0.0)) as u8])
      .unwrap();
    file
      .write(&[(255.0 * test[i].g.min(1.0).max(0.0)) as u8])
      .unwrap();
    file
      .write(&[(255.0 * test[i].b.min(1.0).max(0.0)) as u8])
      .unwrap();
  }
}

fn main() {
  //   let spheres: std::vec::Vec<Sphere> = Vec::with_capacity(4);
  let sphere = Sphere::new(Vec3f::new(-3.0, 0.0, -16.0), 2.0);
  render(&sphere);
}

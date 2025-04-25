use image::{ImageBuffer, Rgb, RgbImage};
use rayon::prelude::*;

use crate::{
    color::Color,
    ray::{HitRecord, Hittable, HittableList, Ray},
    utils::{self, INF, sample_square},
    vec::{Vec3, VecTypes},
};

// responsible for costruct and dispatch rays into world
// and use the result to build the image
#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_wid: u32,
    pub image_name: String,
    pub samples_per_pixel: u8,

    pub vfov: f64,
    pub lookfrom: Vec3, // point camera is looking from
    pub lookat: Vec3,   // point camera is looking at
    pub vup: Vec3,      // Camera-relative up direction

    image_hei: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    delta_x: Vec3,
    delta_y: Vec3,
    pixel_samples_scale: f64,
    max_deep_ray: u8,

    // Camera frame relative basis
    u: Vec3, // camera rigth
    v: Vec3, // camera up
    w: Vec3, // camera view direction
}

impl Camera {
    // based in the objects get the color of pixel pointed from the ray
    fn ray_color(r: &Ray, world: &HittableList, deep: u8) -> Color {
        if deep == 0 {
            return Color::default();
        }
        let mut h = HitRecord::default();
        if world.hit(r, INF, 0.001, &mut h) {
            // refelction based on material
            let mut reflected_r = Ray::default();
            let mut attenuation = Color::default();
            // hitted, the function of material is for check is the refelction will happen
            // and what color is attenuation be
            if h.mat.reflect(r, &mut reflected_r, &h, &mut attenuation) {
                return attenuation * Self::ray_color(&reflected_r, world, deep - 1);
            }
            return Color::default();
        }
        // background
        let unit = r.direction.unit_vec();
        let a = 0.5 * (unit[1] + 1.0);
        Color::new(1.0, 1.0, 1.0).mul(1.0 - a) + Color::new(0.5, 0.7, 1.0).mul(a)
    }

    // will get a rondom ray from camera to arround the i, j pixel
    fn get_ray(&self, i: u32, j: u32, random_near: bool) -> Ray {
        let offset = if random_near {
            sample_square() // get a random vec offset
        } else {
            // for test
            Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 0.0)
        };
        // get a offset vector in a square with diagonal size of 1.0

        // get a random pixel arround the (i, j_)
        let pixel_sample = self.pixel00_loc
            + self.delta_x.mul(i as f64 + offset[0])
            + self.delta_y.mul(j as f64 + offset[1]);

        // same used, get a ray from camera to the pixel
        let ray_dir = pixel_sample - self.center;
        Ray::new(self.center, ray_dir)
    }

    pub fn render(&mut self, world: &HittableList) -> Result<(), image::ImageError> {
        self.inititalize();
        let mut buffer: RgbImage = ImageBuffer::new(self.image_wid, self.image_hei);
        let antialiasing = true;
        let sample = if antialiasing {
            (self.samples_per_pixel, true)
        } else {
            (1, false)
        };

        buffer.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            // parallel the antialiasing
            let c = (0..sample.0)
                .into_par_iter()
                .map(|_| {
                    let r = self.get_ray(x, y, sample.1);
                    Self::ray_color(&r, world, self.max_deep_ray)
                })
                .reduce(Color::default, |a, b| a + b);
            *pixel = Rgb::from(c.mul(self.pixel_samples_scale));
        });

        buffer.save(self.image_name.as_str())
    }

    fn inititalize(&mut self) {
        self.image_hei = (self.image_wid as f64 / self.aspect_ratio) as u32;

        self.center = self.lookfrom;
        let focal_len = (self.lookfrom - self.lookat).vec_length();
        // vfov calc
        let theta = utils::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan(); // side size of view 

        let view_hei = 2.0 * h * focal_len;
        let view_wid = view_hei * (self.image_wid as f64 / self.image_hei as f64);

        self.w = (self.lookfrom - self.lookat).unit_vec();
        self.u = (self.vup * self.w).unit_vec();
        self.v = self.w * self.u;

        let view_x = self.u.mul(view_wid);
        let view_y = self.v.mul(-1.0).mul(view_hei);

        // delta's, size of each pixel
        self.delta_x = view_x.div(self.image_wid as f64);
        self.delta_y = view_y.div(self.image_hei as f64);

        let view_upper_left =
            self.center - self.w.mul(focal_len) - view_x.div(2.0) - view_y.div(2.0);

        self.pixel00_loc = view_upper_left + (self.delta_y + self.delta_x).mul(0.5);
    }

    pub fn new(aspect: f64, img_wid: u32, img_name: &str) -> Self {
        let mut f = Camera::default();
        f.aspect_ratio = aspect;
        f.image_wid = img_wid;
        f.image_name = String::from(img_name);
        f.samples_per_pixel = 7;
        f.pixel_samples_scale = 1.0 / f.samples_per_pixel as f64;
        f.max_deep_ray = 10;
        f.vfov = 90.0;

        f.lookfrom = Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 0.0);
        f.lookat = Vec3::new(VecTypes::Coordinates, 0.0, 0.0, -1.0);
        f.vup = Vec3::new(VecTypes::Coordinates, 0.0, 1.0, 0.0);

        f
    }
}

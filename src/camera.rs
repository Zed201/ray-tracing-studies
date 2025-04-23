use image::{ImageBuffer, Rgb, RgbImage};

use crate::{
    color::Color,
    ray::{Ray, hit_record, hittable, hittable_list},
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

    image_hei: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    delta_x: Vec3,
    delta_y: Vec3,
    pixel_samples_scale: f64,
    max_deep_ray: u8,
}

impl Camera {
    // based in the objects get the color of pixel pointed from the ray
    fn ray_color(r: &Ray, world: &hittable_list, deep: u8) -> Color {
        if deep <= 0 {
            return Color::default();
        }
        let mut h = hit_record::default();
        if world.hit(&r, INF, 0.0, &mut h) {
            // to simulate refelction will begin recursive calculation
            // determine the random direction of the refelction
            let n_dir = Vec3::random_on_hemisphere(&h.normal);
            let n_r = Ray::new(h.point, n_dir);
            // the we have a new ray with origin in the object
            // the multiplied number will determine the "refelction factor" of the "light"
            return Self::ray_color(&n_r, world, deep - 1).mul(0.5);
        }
        // background
        let unit = r.direction.unit_vec();
        let a = 0.5 * (unit[1] + 1.0);
        return Color::new(1.0, 1.0, 1.0).mul(1.0 - a) + Color::new(0.5, 0.7, 1.0).mul(a);
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

    pub fn render(&mut self, world: &hittable_list) -> Result<(), image::ImageError> {
        self.inititalize();
        let mut buffer: RgbImage = ImageBuffer::new(self.image_wid, self.image_hei);
        // resolve with get the pixel beffore the get ray, or passing the args to ray
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let mut c = Color::default();
            for _ in 0..self.samples_per_pixel {
                let r = self.get_ray(x, y, true);
                // add to get the "medium color" of a point
                c += Self::ray_color(&r, world, self.max_deep_ray);
            }
            *pixel = Rgb::from(c.mul(self.pixel_samples_scale));
        }
        buffer.save(self.image_name.as_str())
    }

    fn inititalize(&mut self) {
        self.image_hei = (self.image_wid as f64 / self.aspect_ratio) as u32;

        self.center = Vec3::default(); //0,0,0
        let focal_len = 1.0;
        let view_hei = 2.0;
        let view_wid = view_hei * (self.image_wid as f64 / self.image_hei as f64);

        let view_x = Vec3::new(VecTypes::Coordinates, view_wid, 0.0, 0.0);
        let view_y = Vec3::new(VecTypes::Coordinates, 0.0, -view_hei, 0.0);

        // delta's, size of each pixel
        self.delta_x = view_x.div(self.image_wid as f64);
        self.delta_y = view_y.div(self.image_hei as f64);

        let view_upper_left = self.center
            - Vec3::new(VecTypes::Coordinates, 0.0, 0.0, focal_len)
            - view_x.div(2.0)
            - view_y.div(2.0);

        self.pixel00_loc = view_upper_left + (self.delta_y + self.delta_x).mul(0.5);
    }

    pub fn new(aspect: f64, img_wid: u32, img_name: &str) -> Self {
        let mut f = Camera::default();
        f.aspect_ratio = aspect;
        f.image_wid = img_wid;
        f.image_name = String::from(img_name);
        f.samples_per_pixel = 10;
        f.pixel_samples_scale = 1.0 / f.samples_per_pixel as f64;
        f.max_deep_ray = 5;
        return f;
    }
}

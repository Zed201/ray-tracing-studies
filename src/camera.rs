use image::{ImageBuffer, Rgb, RgbImage};

use crate::{
    ray::{Ray, hittable_list},
    vec::{Vec3, VecTypes},
};

// responsible for costruct and dispatch rays into world
// and use the result to build the image
#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_wid: u32,
    pub buffer: RgbImage,
    pub image_name: String,

    image_hei: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    delta_x: Vec3,
    delta_y: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &hittable_list) -> Result<(), image::ImageError> {
        self.inititalize();
        for (x, y, pixel) in self.buffer.enumerate_pixels_mut() {
            // basically will calc the ray from the camera to the especific point
            let pixel_center =
                self.pixel00_loc + self.delta_x.mul(x as f64) + self.delta_y.mul(y as f64);
            let ray_dir = pixel_center - self.center;
            let r = Ray::new(self.center, ray_dir.unit_vec());

            let c = r.color(&world);
            *pixel = Rgb::from(c);
        }
        self.buffer.save(self.image_name.as_str())
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

        self.buffer = ImageBuffer::new(self.image_wid, self.image_hei);
    }

    pub fn new(aspect: f64, img_wid: u32, img_name: &str) -> Self {
        let mut f = Camera::default();
        f.aspect_ratio = aspect;
        f.image_wid = img_wid;
        f.image_name = String::from(img_name);
        return f;
    }
}

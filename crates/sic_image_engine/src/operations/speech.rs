use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use crate::wrapper::image_path::ImageFromPath;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sic_core::image::{ColorType, DynamicImage};
use sic_core::{image, SicImage};

pub struct Speech<'image> {
    path: &'image ImageFromPath,
}

impl<'image> Speech<'image> {
    pub fn new(path: &'image ImageFromPath) -> Self {
        Self { path }
    }
}

impl<'image> ImageOperation for Speech<'image> {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => speech_image(image, self.path),
            SicImage::Animated(image) => speech_animated_image(image.frames_mut(), self.path),
        }

        Ok(())
    }
}

fn speech_animated_image(frames: &mut [image::Frame], path: &ImageFromPath) {
    let (w, h) = frames[0].buffer_mut().dimensions();
    let base = DynamicImage::new(w, h + 150, ColorType::Rgba8);
    let speech = path.open_image().unwrap().as_ref().resize_exact(
        base.width(),
        150,
        image::imageops::FilterType::Nearest,
    );

    frames.par_iter_mut().for_each(|frame| {
        let frame_buffer = frame.buffer_mut();
        let mut base = base.clone();

        image::imageops::overlay(&mut base, frame_buffer, 0, 150);
        image::imageops::overlay(&mut base, &speech, 0, 0);

        *frame.buffer_mut() = base.into_rgba8();
    });
}

fn speech_image(image: &mut DynamicImage, path: &ImageFromPath) {
    let mut base = DynamicImage::new(image.width(), image.height() + 150, ColorType::Rgba8);
    let speech = path.open_image().unwrap().as_ref().resize_exact(
        base.width(),
        150,
        image::imageops::FilterType::Nearest,
    );

    image::imageops::overlay(&mut base, image, 0, 150);
    image::imageops::overlay(&mut base, &speech, 0, 0);

    *image = base;
}

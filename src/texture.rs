use gl::types::*;
use image::{self, DynamicImage};
use std::{os::raw::c_void, path::Path};

/// Defines the source of an image.
pub enum ImageSource<'a> {
    /// Load the image from a path.
    Path(&'a str),
    /// Create an empty image with the dimensions `(width, height)`.
    Empty(u32, u32),
}

pub fn new_texture(source: ImageSource, internal_format: GLenum) -> GLuint {
    let mut texture = 0;

    unsafe {
        // initialize the texture
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    // load image
    let img = match source {
        ImageSource::Path(path) => image::open(&Path::new(path)).expect("Failed to load texture"),
        ImageSource::Empty(width, height) => DynamicImage::new_rgb32f(width, height),
    };

    // get raw data from the image
    let data = img.as_bytes();

    unsafe {
        // create texture
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            internal_format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA, // read format
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );

        // generate mipmaps
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    texture
}

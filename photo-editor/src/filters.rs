use std::cmp;
use crate::Photo;

/// Converts the photo to grayscale using the Desaturation algorithm
/// @param photo [in/out] the photo to convert
pub fn grayscale(photo: &mut Photo) {
    for i in (0..(photo.pixels.len() - 4)).step_by(4) {
        let r = photo.pixels[i];
        let g = photo.pixels[i + 1];
        let b = photo.pixels[i + 2];

        let gray = (cmp::min(cmp::min(r, g), b) + cmp::max(cmp::max(r, g), b)) / 2;

        photo.pixels[i] = gray;
        photo.pixels[i + 1] = gray;
        photo.pixels[i + 2] = gray;
    }
}

pub fn monochrome(photo: &mut Photo, r_offset: u32, g_offset: u32, b_offset: u32) {
    for i in (0..(photo.pixels.len() - 4)).step_by(4) {
        let r = photo.pixels[i];
        let g = photo.pixels[i + 1];
        let b = photo.pixels[i + 2];
        
        let mut avg: u32 = ((r + g + b) / 3).into();
        if avg >= 255 {
            avg = 255
        }
        let new_r = if avg as u32 + r_offset < 255 {
            avg as u8 + r_offset as u8
        } else {
            255
        };

        let new_g = if avg as u32 + g_offset < 255 {
            avg as u8 + g_offset as u8
        } else {
            255
        };

        let new_b = if avg as u32 + b_offset < 255 {
            avg as u8 + b_offset as u8
        } else {
            255
        };

        photo.pixels[i] = new_r;
        photo.pixels[i + 1] = new_g;
        photo.pixels[i + 1] = new_b;
    }
}

pub fn ocean_blue(photo: &mut Photo) {
    for i in (0..(photo.pixels.len() - 4)).step_by(4) {
        let r = photo.pixels[i];
        let g = photo.pixels[i + 1];
        let b = photo.pixels[i + 2];

        // We can convert the image into gray and then apply color for a simple color overlay
        let gray = (cmp::min(cmp::min(r, g), b) + cmp::max(cmp::max(r, g), b)) / 2;
        let new_r =gray ;
        let new_g = gray + 100;
        let new_b = gray +100;
        photo.pixels[i] = new_r;
        photo.pixels[i + 1] = new_g;
        photo.pixels[i + 2] = new_b;
        
       
    }
}
pub fn purple(photo: &mut Photo) {
    for i in (0..(photo.pixels.len() - 4)).step_by(4) {
        let r = photo.pixels[i];
        let g = photo.pixels[i + 1];
        let b = photo.pixels[i + 2];

        // We can convert the image into gray and then apply color for a simple color overlay
        let gray = (cmp::min(cmp::min(r, g), b) + cmp::max(cmp::max(r, g), b)) / 2;
        let new_r =gray +50;
        let new_g = gray ;
        let new_b = gray +50;
        photo.pixels[i] = new_r;
        photo.pixels[i + 1] = new_g;
        photo.pixels[i + 2] = new_b;
        
       
    }
}

/// Converts the photo to B+W by thresholding
/// @param photo [in/out] the photo to convert
pub fn bw_threshold(photo: &mut Photo) {
    grayscale(photo);
    let threshold = 30;
    for i in (0..(photo.pixels.len() - 4)).step_by(4) {
        let cur_gray = photo.pixels[i];
        let bw = if cur_gray >= threshold { 255 } else { 0 };

        photo.pixels[i] = bw;
        photo.pixels[i + 1] = bw;
        photo.pixels[i + 2] = bw;
    }
}
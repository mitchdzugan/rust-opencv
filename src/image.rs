use std::ptr;
use ffi::core::*;
use ffi::highgui::*;
use ffi::types::{CvMat};

pub struct Image {
  pub raw: *const CvMat,
}

pub enum LoadColor {
  Unchanged = -1,
  GrayScale = 0,
  Color = 1,
}

impl Image {
  pub fn load(path: &Path, flag: Option<LoadColor>) -> Result<Image, &Path> {
    path.with_c_str(|path_c_str| unsafe {
      match cvLoadImageM(path_c_str, flag.unwrap_or(Color) as i32) {
        p if p.is_not_null() => Ok(Image { raw: p }),
        _ => Err(path),
      }
    })
  }

  pub fn save(&self, path: &Path) -> Result<(), &Image> {
    path.with_c_str(|path| unsafe {
      match cvSaveImage(path, self.raw, ptr::null()) {
        0 => Err(self),
        _ => Ok(()),
      }
    })
  }

  pub fn width(&self) -> int {
    unsafe {
      let size = cvGetSize(self.raw);
      size.width as int
    }
  }

  pub fn height(&self) -> int {
    unsafe {
      let size = cvGetSize(self.raw);
      size.height as int
    }
  }
}

impl Clone for Image {
  fn clone(&self) -> Image {
    unsafe { Image { raw: cvCloneMat(self.raw) } }
  }
}

impl Drop for Image {
  fn drop(&mut self) -> () {
    unsafe { cvReleaseMat(&self.raw); }
  }
}

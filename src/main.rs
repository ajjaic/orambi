//! Input: List of image files (JPG, JPEG, jpg, jpeg)
//!
//! Output: Renames the image files to the date when the picture was taken. If date cannot be found
//!     then, the name of the file remains unchanged.
//!
//! Example Input:
//!     `~$ ./orambi image1.jpg image2.jpeg image3.JPEG`
//!
//! Example Output:
//!     `~$ ls`
//!     `2012_22_10_image1.jpg 2012_12_11_image2.jpeg 2014_12_12_image3.JPEG`

extern crate exif;

mod timestamper;
mod errors;

fn main() {
    println!("Hello, world!");
}


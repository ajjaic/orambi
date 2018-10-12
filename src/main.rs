/// Input: List of image files (PNG, GIF, WEB8, JPG)
///
/// Output: Renames the image files to the date when the picture was taken. If date cannot be found
///     then, the name of the file remains unchanged.
///
/// Example Input:
///     `~$ ./orambi image1.jpg image2.png image3.gif`
///
/// Example Output:
///     `~$ ls`
///     `2012_22_10_image1.jpg 2012_12_11_image2.png 2014_12_12_image3.gif`

extern crate exif;

use std::path::Path;
use std::ffi::OsStr;
use exif::Reader as ExifReader;
use std::fs::File;
use std::io::BufReader;
use exif::Error;
use exif::Tag;
use exif::DateTime;
use exif::Field as ExifField;
use std::fs::rename;
use std::path::PathBuf;
use std::fs::copy;

type Year = u16;
type Month = u8;
type Day = u8;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct TimeStamper<'a> {
    file_path: &'a Path,
}

impl<'a> TimeStamper<'a> {
    fn new<S: AsRef<OsStr> + ?Sized>(path: &S) -> TimeStamper {
        let p: &Path = Path::new(path.as_ref());

        TimeStamper {
            file_path: p,
        }
    }

    fn rename_with_datestamp(&self) -> Result<(), std::io::Error> {
        let mut f = BufReader::new(File::open(self.file_path)?);
        let exifreader = match ExifReader::new(&mut f) {
            Ok(r) => r,
            Err(e) => {
                println!("{:?}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "No Reader"));
            },
        };

        let result = self.get_datestamp(&exifreader)
            .and_then(|datestamp| self.get_new_file_path(datestamp))
            .map(|fp| copy(self.file_path, fp));

        match result {
            None => Err(std::io::Error::new(std::io::ErrorKind::Other, "No Reader")),
            Some(r) => r.map(|_| ()),
        }

    }


    fn get_new_file_path(&self, datestamp: (Year, Month, Day)) -> Option<PathBuf> {
        let parent_path = self.file_path.parent();
        let file_name = self.file_path.file_name();

        match (parent_path, file_name) {
            (Some(p), Some(f)) => {
                let mut new_file_name = String::new();
                new_file_name.push_str(&datestamp.0.to_string());
                new_file_name.push('_');
                new_file_name.push_str(&format!("{:02}", datestamp.1));
                new_file_name.push('_');
                new_file_name.push_str(&format!("{:02}", datestamp.2));
                new_file_name.push('_');

                f.to_str()
                    .map(|s|  new_file_name.push_str(s))
                    .and(parent_path.map(|p| p.join(new_file_name)))
            },
            _ => None,
        }
    }

    fn get_datestamp(&self, exif_reader: &ExifReader) -> Option<(Year, Month, Day)> {
        exif_reader.fields()
            .iter()
            .filter(|f| f.tag == Tag::DateTime)
            .last()
            .map(|f| {
                if let exif::Value::Ascii(ref data) = f.value {
                    DateTime::from_ascii(data[0]).ok()
                } else {
                    None
                }
            })
            .unwrap_or_default()
            .map(|ts| (ts.year, ts.month, ts.day))
    }
}

#[cfg(test)]
mod test {
    use super::TimeStamper;
    use std::path::Path;
    use std::fs::remove_file;

    #[test]
    fn test_pic_name_datestamping() {
        let old_img_path = "./testarea/17-08-26 08-30-25 1742.jpg";
        let new_img_path = "./testarea/2017_08_26_17-08-26 08-30-25 1742.jpg";
        let img = TimeStamper::new(old_img_path);
        img.rename_with_datestamp().unwrap();

        assert!(Path::new(old_img_path).exists());
        assert!(Path::new(new_img_path).exists());
        remove_file(new_img_path).unwrap();
    }
}


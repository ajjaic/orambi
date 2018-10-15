use std::{
    path::{ Path, PathBuf, },
    fs::{ File, copy, },
    ffi::OsStr,
    io::BufReader,
};
use exif::{
    Reader as ExifReader,
    Tag,
    DateTime,
    Value,
};

use errors::Error;

type Year = u16;
type Month = u8;
type Day = u8;

#[derive(Debug)]
struct TimeStamper<'a> {
    file_path: &'a Path,
}

impl<'a> TimeStamper<'a> {

    const SUPPORTED_FILES: [&'a str; 4] = ["jpg", "jpeg", "JPG", "JPEG"];

    fn rename_with_datestamp<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<(), Error> {
        let timestamper = {
            let t = TimeStamper { file_path: Path::new(path) };
            if !t.file_path.exists() {
                return Err(Error::FileNotFound);
            } else if !t.supported() {
                return Err(Error::FileNotSupported);
            }
            t
        };

        let mut f = BufReader::new(File::open(timestamper.file_path)?);
        let exifreader = ExifReader::new(&mut f)?;

        let datestamp = timestamper.get_datestamp(&exifreader)
            .ok_or(Error::CreationDateUnavailable)?;
        let new_file_path = timestamper.get_new_file_path(datestamp)
            .ok_or(Error::InvalidPath)?;

        copy(timestamper.file_path, new_file_path)?;
        Ok(())
    }

    fn supported(&self) -> bool {
        match self.file_path.extension() {
            None => false,
            Some(s) => {
                match s.to_str() {
                    None => false,
                    Some(extension) => TimeStamper::SUPPORTED_FILES.contains(&extension)
                }
            }
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
                    .and(Some(p.join(new_file_name)))
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
                if let Value::Ascii(ref data) = f.value {
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
    use std::{
        path::Path,
        fs::remove_file,
    };
    use errors::Error;
    use exif::Error as ExifError;

    #[test]
    fn test_pic_name_datestamping() {
        let old_img_path = "./testarea/has_creation_date.jpg";
        let new_img_path= "./testarea/2012_06_13_has_creation_date.jpg";

        TimeStamper::rename_with_datestamp(old_img_path).unwrap();

        assert!(Path::new(old_img_path).exists());
        assert!(Path::new(new_img_path).exists());
        remove_file(new_img_path).unwrap();
    }

    #[test]
    fn test_no_exif_data() {
        let old_img_path = "./testarea/no_exif_data.JPG";
        let res = TimeStamper::rename_with_datestamp(old_img_path);

        assert_eq!(res, Err(Error::ModError(Box::new(ExifError::NotFound("No Exif data found")))));
    }
}

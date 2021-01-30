use rand::prelude::*;

use approx::assert_abs_diff_eq;
use libwebp_image::{
    webp_load, webp_load_from_memory, webp_load_rgb, webp_load_rgb_from_memory, webp_load_rgba,
    webp_load_rgba_from_memory,
};
use std::io::{self, Cursor, Read};

#[test]
fn test_webp_load() {
    let reader = TestReader(Cursor::new(JELLY));
    let img = webp_load(reader).unwrap();
    let img = img.as_rgba8().unwrap();
    assert_eq!(img.dimensions(), (256, 256));
    let x = img.get_pixel(0, 0);
    assert_abs_diff_eq!(x.0[..], [140, 145, 102, 255][..], epsilon = 1);
}

#[test]
fn test_webp_load_rgba() {
    let reader = TestReader(Cursor::new(JELLY));
    let img = webp_load_rgba(reader).unwrap();
    assert_eq!(img.dimensions(), (256, 256));
    let x = img.get_pixel(0, 0);
    assert_abs_diff_eq!(x.0[..], [140, 145, 102, 255][..], epsilon = 1);
}

#[test]
fn test_webp_load_rgb() {
    let reader = TestReader(Cursor::new(JELLY));
    let img = webp_load_rgb(reader).unwrap();
    assert_eq!(img.dimensions(), (256, 256));
    let x = img.get_pixel(0, 0);
    assert_abs_diff_eq!(x.0[..], [140, 145, 102][..], epsilon = 1);
}

#[test]
fn test_webp_load_from_memory() {
    let img = webp_load_from_memory(JELLY).unwrap();
    let img = img.as_rgba8().unwrap();
    assert_eq!(img.dimensions(), (256, 256));
    let x = img.get_pixel(0, 0);
    assert_abs_diff_eq!(x.0[..], [140, 145, 102, 255][..], epsilon = 1);
}

#[test]
fn test_webp_load_rgba_from_memory() {
    let img = webp_load_rgba_from_memory(JELLY).unwrap();
    assert_eq!(img.dimensions(), (256, 256));
    let x = img.get_pixel(0, 0);
    assert_abs_diff_eq!(x.0[..], [140, 145, 102, 255][..], epsilon = 1);
}

#[test]
fn test_webp_load_rgb_from_memory() {
    let img = webp_load_rgb_from_memory(JELLY).unwrap();
    assert_eq!(img.dimensions(), (256, 256));
    let x = img.get_pixel(0, 0);
    assert_abs_diff_eq!(x.0[..], [140, 145, 102][..], epsilon = 1);
}

const JELLY: &[u8] = include_bytes!("jelly.webp");

struct TestReader<R>(R);

impl<R: Read> Read for TestReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        let limit = thread_rng().gen_range(1..=buf.len());
        self.0.read(&mut buf[..limit])
    }
}

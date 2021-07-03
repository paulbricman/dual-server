use std::fs::File;
use std::io;

/// Return position of subsequence in sequence
pub fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// Fetch file from url and place it at location
pub fn fetch_file(url: &str, filename: &str) {
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut out = File::create(filename).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

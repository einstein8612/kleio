#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn new_key(b: &mut Bencher) {
        b.iter(|| test::black_box(kleio_rs::generate_key()));
    }
}

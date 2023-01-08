use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};
use quickcheck::Arbitrary;
use socket2::SockAddr;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

fn _benchmark_hash(bench: &mut Bencher) {
    let mut g = quickcheck::Gen::new(1024);
    let x: SockAddr = Arbitrary::arbitrary(&mut g);
    let mut hasher = DefaultHasher::new();

    // println!("x: {:?}", x.as_socket());
    bench.iter(|| {
        x.hash(&mut hasher);
        hasher.finish()
    })
}

fn _benchmark_as_socket_hash(bench: &mut Bencher) {
    let mut g = quickcheck::Gen::new(1024);
    let x: SockAddr = Arbitrary::arbitrary(&mut g);
    let mut hasher = DefaultHasher::new();

    // println!("x: {:?}", x.as_socket());
    bench.iter(|| {
        x.as_socket().map(|y| y.hash(&mut hasher));
        hasher.finish()
    })
}

benchmark_group!(benches, _benchmark_hash, _benchmark_as_socket_hash);
benchmark_main!(benches);

use bloomfilter::Bloom;
#[cfg(feature = "random")]
use bloomfilter::reexports::getrandom::getrandom;

#[test]
#[cfg(feature = "random")]
fn bloom_test_set() {
    let mut bloom = Bloom::<u16, 20>::new(1373);
    let k = 18u16;
    assert!(bloom.check(&k) == false);
    bloom.set(&k);
    assert!(bloom.check(&k) == true);
}

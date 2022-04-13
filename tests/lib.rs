#[cfg(test)]
mod tests {

    #![allow(deprecated)]

    use core::hash::{Hash, Hasher};
    use komihash_rs::Komihash;

    // Hash just the bytes of the slice, without length prefix
    struct Bytes<'a>(&'a [u8]);


    impl<'a> Hash for Bytes<'a> {
        #[allow(unused_must_use)]
        fn hash<H: Hasher>(&self, state: &mut H) {
            let Bytes(v) = *self;
            state.write(v);
        }
    }

    fn hash_with<H: Hasher, T: Hash>(mut st: H, x: &T) -> u64 {
        x.hash(&mut st);
        st.finish()
    }

    #[allow(dead_code)]
    fn hash<T: Hash>(x: &T) -> u64 {
        hash_with(Komihash::default(), x)
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_hash_4b(){
        let t = [0x74, 0x65, 0x73, 0x74, 0x31]; //"test1"
        let s = b"test1";
        let mut hasher = Komihash::default();
        let mut hasher2 = Komihash::default();
        hasher.write(&t);
        hasher2.write(&s[..]);
        let out1 = hasher.finish();
        let out = hasher2.finish();
        assert_eq!(out, out1);

    }
}
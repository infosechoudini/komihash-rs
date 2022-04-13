#[allow(unused)]

use std::convert::TryInto;
use std::hash::{Hasher, BuildHasher, BuildHasherDefault};
use std::collections::{HashMap, HashSet};



/// A builder for default Kh hashers.
pub type KhBuildHasher = BuildHasherDefault<KomihashBuilder>;

/// A `HashMap` using a default Kh hasher.
pub type KhHashMap<K, V> = HashMap<K, V, KhBuildHasher>;

/// A `HashSet` using a default Komihash.
pub type KhHashSet<V> = HashSet<V, KhBuildHasher>;

#[allow(dead_code)]
fn komihash_bytesw32(v: u32) -> u32 {

    ( v & (0xFF000000 as u32) ) >> 24_u32| 
    ( v & (0x00FF0000 as u32) ) >> 8_u32 | 
    ( v & (0x0000FF00 as u32) ) << 8_u32 | 
    ( v & (0x000000FF as u32) ) << 24_u32 
}


#[allow(dead_code)]
fn komihash_bytesw64(v: u64) -> u64 {
    ( v & 0xFF00000000000000_u64 ) >> 56_u64 | 
    ( v & 0x00FF000000000000_u64 ) >> 40_u64 | 
    ( v & 0x0000FF0000000000_u64 ) >> 24_u64 | 
    ( v & 0x000000FF00000000_u64 ) >> 8_u64 | 
    ( v & 0x00000000FF000000_u64 ) << 8_u64 | 
    ( v & 0x0000000000FF0000_u64 ) << 24_u64 | 
    ( v & 0x000000000000FF00_u64 ) << 40_u64 | 
    ( v & 0x00000000000000FF_u64 ) << 56_u64 
}

#[allow(dead_code)]
fn kh_lu32ec(p: &[u8] ) -> u32 {

    return komihash_bytesw32(p[..3].as_ptr() as u32);
}

pub fn kh_lu64ec(p: &[u8] ) -> u64 {

    return komihash_bytesw64(p[..7].as_ptr() as u64);
}


#[inline]
fn read_64(data: &[u8]) -> u64 {
    let mut v = 0;
    for i in 0..8 {
        v |= (data[i] as u64) << (i * 8);
    }
    v
}


#[inline]
fn read_32(data: &[u8]) -> u32 {
    let mut v = 0;
    for i in 0..4 {
        v |= (data[i] as u32) << (i * 8);
    }
    v
}


#[inline]
pub fn kh_lpu64ec_l3(msg: &[u8], fb: u64) -> u64 {
    let len = msg.len();
    if len < 4 {
        let ml8 = (len << (3 as usize)) as u64;
        let m = (msg[0] as u64 )| ((msg[1] as u64) << 8_u64) | ((msg[2] as u64) << 16_u64);
        return fb << ml8 | m >> (24 - ml8);
    }

    let ml8 = len << 3 ;
    let ml = read_32(&msg[len - 4 .. ]) as u64;
    let mh = read_64(&msg[ .. 8]);

    return fb << ml8 | ml | ((mh >> (64 - ml8)) << 32);
}




#[inline]
fn kh_lpu64ec_l4(msg: &[u8], fb: u64) -> u64 {
    let msglen = msg.len();
    if msglen < 5 {
        let ml8 = (msglen << (3 as usize)) as u64;
        return fb << ml8 | (read_32(&msg[msglen - 4 ..]) as u64) >> (32_u64 - ml8);
    }

    let ml8 = (msglen << (3 as usize)) as u64;

    return fb << ml8 | read_64(&msg[..8]) >> (64_u64 - ml8);
}




#[inline]
fn kh_m128(ab: u64, cd: u64) -> (u64, u64) {

    let r = (ab * cd) as u128;
    let rl = r as u64;
    let rh = (r >> 64) as u64;

    return (rl, rh);

}



#[derive(Debug, Clone, Copy)]
pub struct Komihash{
    _useseed: u64,
    pub seed1: u64,
    pub seed5: u64,
    r1l: u64,
    r1h: u64,
    pub r2l: u64,
    pub r2h: u64,
    seed2: u64,
    seed3: u64,
    seed4: u64,
    seed6: u64,
    seed7: u64,
    seed8: u64,
    r3l: u64,
    r3h: u64,
    r4l: u64,
    r4h: u64,
}


impl Komihash{

    //Used for anything less than 7 Bytes
    #[inline]
    fn kh_lpu64ec_nz(&mut self, msg: &[u8], mut fb: u64) -> u64 {
        let msglen = msg.len();
        if msglen < 4 {
            fb <<= msglen << 3;
            let mut m = msg[0] as u64;
            
            if msglen > 2{
                m |= (msg[1] as u64) << 8_u64;
                m |= (msg[2] as u64) << 16_u64;
            } else if msglen > 1 {
                m |= (msg[1] as u64) << 8_u64;
            }
    
            return fb | m 
    
        } else {
            let ml8 = msglen << 3;
            let mh = read_32(&msg[msglen - 4 ..]) as u64;
            let ml = read_32(&msg[..4]) as u64;
    
            return fb << ml8 | ml | (mh >> (64 - ml8)) << 32;
        }
    }


    #[inline]
    fn kh_64_bytes(&mut self, mut m_clone: &[u8]){

        
        while m_clone.len() > 63 {
            let i = 0;

            (self.r1l, self.r1h) = kh_m128(self.seed1 ^ read_64(&m_clone[i .. i + 8]), self.seed5 ^ read_64(&m_clone[i + 8.. i + 16]));
        
            (self.r2l, self.r2h) = kh_m128(self.seed2 ^ read_64(&m_clone[i + 16 .. i + 24]), self.seed6 ^ read_64(&m_clone[i + 24 .. i + 32]));
            (self.r3l, self.r3h) = kh_m128(self.seed3 ^ read_64(&m_clone[i + 32 .. i + 40]), self.seed7 ^ read_64(&m_clone[i + 40 .. i + 48]));

            (self.r4l, self.r4h) = kh_m128(self.seed4 ^ read_64(&m_clone[i + 48 .. i + 56]), self.seed8 ^ read_64(&m_clone[i + 56 .. i + 64]));
        
            self.seed5 += self.r1h;
            self.seed6 += self.r2h;
            self.seed7 += self.r3h;
            self.seed8 += self.r4h;
            self.seed2 = self.seed5 ^ self.r2l;
            self.seed3 = self.seed6 ^ self.r3l;
            self.seed4 = self.seed7 ^ self.r4l;
            self.seed1 = self.seed8 ^ self.r1l;

            m_clone = &m_clone[64..]
        }
    
    }


    #[inline]
    fn komihash_hash16(&mut self, m: &[u8]) {
        (self.r1l, self.r1h) = kh_m128(self.seed1 ^ read_64(&m[..8]), self.seed5 ^ read_64(&m[m.len() - 8..]));
        self.seed5 += self.r1h;
        self.seed1 = self.seed5 ^ self.r1l;
    }

    #[inline]
    pub fn komihash_hashround(&mut self) {
        (self.r2l, self.r2h) = kh_m128(self.seed1, self.seed5);
        self.seed5 += self.r2h;
        self.seed1 = self.seed5 ^ self.r2l;
    }
    
    #[inline]
    pub fn komihash_hashfin(&mut self) {
        (self.r1l, self.r1h) = kh_m128(self.r2l, self.r2h);
        self.seed5 += self.r1h;
        self.seed1 = self.seed5 ^ self.r1l;
        self.komihash_hashround();
    }

    #[inline]
    fn new(useseed: u64) -> Self {
        let seed1 = 0x243F6A8885A308D3_u64 ^ ( useseed & 0x5555555555555555_u64 );
        let seed5 = 0x452821E638D01377_u64 ^ ( useseed & 0xAAAAAAAAAAAAAAAA_u64 );

        let mut kh = Komihash{
            _useseed: useseed,
            seed1: seed1,
            seed5: seed5,
            r1l: 0_u64,
            r1h: 0_u64,
            r2l: 0_u64,
            r2h: 0_u64,
            seed2: 0_u64,
            seed3: 0_u64,
            seed4: 0_u64,
            seed6: 0_u64,
            seed7: 0_u64,
            seed8: 0_u64,
            r3l: 0_u64,
            r3h: 0_u64,
            r4l: 0_u64,
            r4h: 0_u64,
        };

        kh.seed2 = 0x13198A2E03707344 ^ kh.seed1;
        kh.seed3 = 0xA4093822299F31D0 ^ kh.seed1;
        kh.seed4 = 0x082EFA98EC4E6C89 ^ kh.seed1;
        kh.seed6 = 0xBE5466CF34E90C6C ^ kh.seed5;
        kh.seed7 = 0xC0AC29B7C97C50DD ^ kh.seed5;
        kh.seed8 = 0x3F84D5B5B5470917 ^ kh.seed5;

        kh
    }


    //Anything less than 16 bytes
    #[inline]
    fn write_less_b16(&mut self, m: &[u8]){
        let msglen = m.len();
        self.r2l = self.seed1;
        self.r2h = self.seed5;


        // Used if between 7..15 bytes
        if msglen > 7 {
            self.r2h ^= kh_lpu64ec_l3(&m[msglen - 8..], 1 << (&m[msglen - 1]>> 7));
            self.r2l ^= read_64(&m[..8]);
        } else {
            self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[msglen - 1] >> 7)); //anything less than 7 bytes
        }
        return
    }


    //16..31 bytes
    #[inline]
    fn write_16_31(&mut self, m: &[u8]){
        let mut msglen = m.len();
        self.komihash_hash16(&m);
        let fb = 1 << ( &m[msglen-1] >> 7 ) as u64;
        self.r2h = self.seed1 ^ kh_lpu64ec_l4(&m[msglen - 16 .. msglen - 1], fb);
        self.r2l = self.seed5;
        return
    }

    // 16..31
    #[inline]
    fn write_less_b32(&mut self, m: &[u8]){
        let mut msglen = m.len();

        self.komihash_hash16(&m);

        let fb = 1 << ( &m[msglen-1] >> 7 ) as u64;


        if msglen > 23 {
            //Used if 24..31 Bytes
            self.r2h = self.seed5 ^ kh_lpu64ec_l4(&m[msglen - 24..], fb);
            self.r2l = self.seed1 ^ read_64(&m[16..24]);
        } else {
            self.r2h = self.seed1 ^ kh_lpu64ec_l4(&m[msglen - 16 .. msglen - 1], fb);
            self.r2l = self.seed5;
        }
        return
    }

    #[inline]
    fn seed_xor(&mut self) {
        self.seed5 ^= self.seed6 ^ self.seed7 ^ self.seed8;
        self.seed1 ^= self.seed2 ^ self.seed3 ^ self.seed4;
    }

    #[inline]
    fn over_64(&mut self, mut m: &[u8]){
        let mut msglen = m.len();

        let chunk_float = (m.len() as f64 / 64.0) as f64;

        let chunks = chunk_float.floor() as usize;
        self.kh_64_bytes(&m);

        m = &m[chunks * 64 ..];
        msglen -= chunks * 64;
            
        self.seed_xor();


        //return after any multiple of 64 bytes
        if msglen == 0 {
            return;
        }

        // Used if between 32..64 bytes
        if msglen > 31 {
            self.komihash_hash16(&m[..16]);
            self.komihash_hash16(&m[16..32]);


            m = &m[32..];
            msglen -= 32;

        }
        // Used if between 16..31
        if msglen > 15 {
            self.komihash_hash16(&m[0..16]);

            m = &m[16..];
            msglen -= 16;
        }
        //Used if between 8..15
        if msglen > 7 {
            let fb = 1 << ( &m[msglen - 1] >> 7 ) as u64;
            self.r2h = self.seed5 ^ kh_lpu64ec_l4(&m[msglen - 8 ..], fb);
            self.r2l = self.seed1 ^ read_64(&m[ .. 8]);
        } else if msglen > 0 { //Used if 1..7
            let fb = 1 << ( &m[msglen - 1] >> 7 ) as u64;
            self.r2h = self.seed1 ^ kh_lpu64ec_l4(&m, fb);
            self.r2l = self.seed5;
        }
        return
    }

    fn komihash_full(&mut self, m: &[u8]){
        //placeholder
    }
}

impl Hasher for Komihash {

    #[inline]
    fn write(&mut self, mut m: &[u8]) {
        let msglen = m.len();


        /*

        if msglen == 0 {
            return;
        }

        if msglen < 8 {
            self.r2l = self.seed1;
            self.r2h = self.seed5;
            self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes
            return;          
        }

        if msglen < 16 {
            self.r2l = self.seed1;
            self.r2h = self.seed5;
            self.r2h ^= kh_lpu64ec_l3(&m, 1 << (&m[msglen - 1]>> 7));
            self.r2l ^= read_64(&m[..8]); 
            return;
        }

        if msglen < 32 {
            self.write_16_31(&m);
            return;
        }

        if msglen < 64 {
            self.seed_xor();
            self.komihash_hash16(&m[..16]);
            self.komihash_hash16(&m[16..32]);
            return;         
        }

        self.over_64(&m);
        return;

        */
    
        match msglen {
            0 => return,

            1..=7 => {
                self.r2l = self.seed1;
                self.r2h = self.seed5;
                self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes
                return;
            }

            8..=15 => {
                let msglen = m.len();
                self.r2l = self.seed1;
                self.r2h = self.seed5;
                self.r2h ^= kh_lpu64ec_l3(&m, 1 << (&m[msglen - 1]>> 7));
                self.r2l ^= read_64(&m[..8]); 
                return;
            }

            16..=31 => {
                self.write_16_31(&m);
                return;
            }
            32..=63 => {
                self.seed_xor();
                self.komihash_hash16(&m[..16]);
                self.komihash_hash16(&m[16..32]);
                return;
            }
            _ => {
                self.over_64(&m);
                return;
            }
        }
        
    }

    #[inline]
    fn finish(&self) -> u64 {
        let mut kh = *self;
        kh.komihash_hashfin();
        kh.seed1
    }

    #[inline]
    fn write_u8(&mut self, i: u8) { 
        let m = u8::to_ne_bytes(i);
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        let m = u16::to_ne_bytes(i);
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes
    }

    #[inline]
    fn write_u32(&mut self, i: u32) { 
        let m = u32::to_ne_bytes(i);
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes
 
    }

    #[inline]
    fn write_u64(&mut self, i: u64) { 
        let m = u64::to_ne_bytes(i);
        let msglen = m.len();
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2h ^= kh_lpu64ec_l3(&m, 1 << (&m[msglen - 1]>> 7));
        self.r2l ^= read_64(&m[..8]); 
    }


    #[inline]
    fn write_u128(&mut self, i: u128) { 
        let m = u128::to_ne_bytes(i);
        let msglen = m.len();
        self.komihash_hash16(&m);
        let fb = 1 << ( &m[msglen-1] >> 7 ) as u64;
        self.r2h = self.seed1 ^ kh_lpu64ec_l4(&m, fb);
        self.r2l = self.seed5;
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        let m = i8::to_ne_bytes(i);
        self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes
    }

    #[inline]
    fn write_i16(&mut self, i: i16) { 
        let m = i16::to_ne_bytes(i);
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes    
    }

    #[inline]
    fn write_i32(&mut self, i: i32) { 
        let m = i32::to_ne_bytes(i);
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes    
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        let m = i64::to_ne_bytes(i);
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2h ^= kh_lpu64ec_l3(&m, 1 << (&m[m.len() - 1]>> 7));
        self.r2l ^= read_64(&m[..8]); 
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        let m = i128::to_ne_bytes(i);
        self.komihash_hash16(&m);
        let fb = 1 << ( &m[m.len()-1] >> 7 ) as u64;
        self.r2h = self.seed1 ^ kh_lpu64ec_l4(&m, fb);
        self.r2l = self.seed5;
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn write_usize(&mut self, i: usize) {
        let m = u32::to_ne_bytes(i as u32);
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes      
    }
    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn write_usize(&mut self, i: usize) {
        let m = u64::to_ne_bytes(i as u64);
        let msglen = m.len();
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2h ^= kh_lpu64ec_l3(&m, 1 << (&m[msglen - 1]>> 7));
        self.r2l ^= read_64(&m[..8]);     
    }
    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn write_isize(&mut self, i: isize) {
        let m = i32::to_ne_bytes(i as i32);
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2l ^= self.kh_lpu64ec_nz(&m, 1 << (&m[0] >> 7)); //anything less than 7 bytes      
    }
    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn write_isize(&mut self, i: isize) {
        let m = i64::to_ne_bytes(i as i64);
        let msglen = m.len();
        self.r2l = self.seed1;
        self.r2h = self.seed5;
        self.r2h ^= kh_lpu64ec_l3(&m, 1 << (&m[msglen - 1]>> 7));
        self.r2l ^= read_64(&m[..8]);     
    }
}
    


impl Default for Komihash{
    #[inline]
    fn default() -> Self {
        KomihashBuilder::new(0_u64).build()
    }
}


#[derive(Clone, Copy, Debug)]
pub struct KomihashBuilder {
    _seed: u64,
    hasher: Komihash,
}

impl KomihashBuilder {
    #[inline]
    ///Creates builder with provided `seed`
    pub fn new(seed: u64) -> Self {
        Self {
            _seed: seed,
            hasher: Komihash::new(seed)
        }
    }

    #[inline]
    ///Creates hasher.
    pub fn build(&self) -> Komihash {
        let mut kh = self.hasher;
        kh.komihash_hashround();
        kh
    }
}


impl BuildHasher for KomihashBuilder {
    type Hasher = Komihash;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        self.build()
        
    }
}


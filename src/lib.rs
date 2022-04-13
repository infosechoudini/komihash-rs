#[allow(unused)]

use std::convert::TryInto;
use core::hash::{Hasher, BuildHasher};


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
pub fn kh_lpu64ec_nz(msg: &[u8], mut fb: u64) -> u64 {
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
    #[inline]
    fn kh_loop_roll(&mut self, m_clone: &[u8]){

        let i = 0;

        (self.r1l, self.r1h) = kh_m128(self.seed1 ^ read_64(&m_clone[i .. i + 8]), self.seed5 ^ read_64(&m_clone[i + 8.. i + 16]));
    
        (self.r2l, self.r2h) = kh_m128(self.seed2 ^ read_64(&m_clone[i + 16 .. i + 24]), self.seed6 ^ read_64(&m_clone[i + 24 .. i + 32]));
        (self.r3l, self.r3h) = kh_m128(self.seed1 ^ read_64(&m_clone[i + 32 .. i + 40]), self.seed5 ^ read_64(&m_clone[i + 40 .. i + 48]));

        (self.r4l, self.r4h) = kh_m128(self.seed1 ^ read_64(&m_clone[i + 48 .. i + 56]), self.seed5 ^ read_64(&m_clone[i + 56 .. i + 64]));
    
        self.seed5 += self.r1h;
        self.seed6 += self.r2h;
        self.seed7 += self.r3h;
        self.seed8 += self.r4h;
        self.seed2 = self.seed5 ^ self.r2l;
        self.seed3 = self.seed6 ^ self.r3l;
        self.seed4 = self.seed7 ^ self.r4l;
        self.seed1 = self.seed8 ^ self.r1l;
    
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
        Komihash{
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
        }
    }
}

impl Hasher for Komihash {
    fn write(&mut self, mut m: &[u8]) {

        self.komihash_hashround();

        let mut msglen = m.len();

        match msglen < 32 {
            true => {
                
                if msglen == 0 {
                    return
                }
                
                if msglen < 16 {
                    self.r2l = self.seed1;
                    self.r2h = self.seed5;
        
                    if msglen > 7 {
                        self.r2h ^= kh_lpu64ec_l3(&m[msglen - 8..], 1 << (&m[msglen - 1]>> 7));
                        self.r2l ^= read_64(&m[..8]);
                    } else {
                        self.r2l ^= kh_lpu64ec_nz(&m, 1 << (&m[msglen - 1] >> 7));
                    }
                    return
                }
                
                if msglen < 32 {
                    self.komihash_hash16(&m);

                    let fb = 1 << ( &m[msglen-1] >> 7 ) as u64;
        
                    if msglen > 23 {
                        self.r2h = self.seed5 ^ kh_lpu64ec_l4(&m[msglen - 24..], fb);
                        self.r2l = self.seed1 ^ read_64(&m[16..24]);
                    } else {
                        self.r2h = self.seed1 ^ kh_lpu64ec_l4(&m[msglen - 16 .. msglen - 1], fb);
                        self.r2l = self.seed5;
                    }
                    return
                }
                
            },
            
            false => {
                self.seed2 = 0x13198A2E03707344 ^ self.seed1;
                self.seed3 = 0xA4093822299F31D0 ^ self.seed1;
                self.seed4 = 0x082EFA98EC4E6C89 ^ self.seed1;
                self.seed6 = 0xBE5466CF34E90C6C ^ self.seed5;
                self.seed7 = 0xC0AC29B7C97C50DD ^ self.seed5;
                self.seed8 = 0x3F84D5B5B5470917 ^ self.seed5;
    
    
    
                loop{

                    if msglen < 64 {
                        break;
                    }

                    self.kh_loop_roll(&m);

                    m = &m[64..];
                    msglen -= 64;
                    
                }
    
                self.seed5 ^= self.seed6 ^ self.seed7 ^ self.seed8;
                self.seed1 ^= self.seed2 ^ self.seed3 ^ self.seed4;
                
            
                if msglen > 31 {
                    self.komihash_hash16(&m[..15]);
        
                    self.komihash_hash16(&m[16..]);
        
        
                    m = &m[32..];
                    msglen -= 32;
        
                }
        
                if msglen > 15 {
                    self.komihash_hash16(&m[0..15]);
        
                    m = &m[16..];
                    msglen -= 16;
                }
        
        
                if msglen > 7 {
                    let fb = 1 << ( &m[msglen - 1] >> 7 ) as u64;
                    self.r2h = self.seed5 ^ kh_lpu64ec_l4(&m[msglen - 8 ..], fb);
                    self.r2l = self.seed1 ^ read_64(&m[ .. 8]);
                } else if msglen > 0 {
                    let fb = 1 << ( &m[msglen - 1] >> 7 ) as u64;
                    self.r2h = self.seed1 ^ kh_lpu64ec_l4(&m, fb);
                    self.r2l = self.seed5;
                }
                return
            }
        }
    }

    fn finish(&self) -> u64 {
        let mut kh = *self;
        kh.komihash_hashfin();
        kh.seed1
    }
}
    


impl Default for Komihash{
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
        self.hasher
    }
}


impl BuildHasher for KomihashBuilder {
    type Hasher = Komihash;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        self.build()
    }
}


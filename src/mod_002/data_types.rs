

fn main(){

    /*
    Scalar Types
     */

    // Integers

    let _a:u8 = 255;
    let _b:u16 = 65_000;
    let _c:u32 = 32_3232;
    let _d:u64 = -32;
    let _e:u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let _f:usize = 132342323;

    let _p:i8 = 127;
    let _q:i16 = 127;
    let _r:i32 = 127;
    let _s:i64 = 127;
    let _t:i128 = 127;
    let _u:isize = 133882093748902347;

    let _h = 32_323;
    let _h = 32_323u16;
    let _h = 0xff;
    let _h = 0o77;
    let _h = 0b1010010001010101010010101;
    let _h = b"Mahesh";
    let _h = b'm';

    // Floating point mu,bers

    let _f1: f32 = 193784980374093874093749033948739847980.23223110093248093480938491219213;
    let _f2: f64 = 22371398237923798793982138972845982749234989274989810.23209820938028912839981273897908230982093098231101219213;

    // Numeric Operations

    let _sum = 5 + 10;
    let _diff = 95.5 + 4.3;

    // integer overflow with `cargo build --release`
    // let _ = 255u8 + 1; // error
    let _ = 255_u8.wrapping_add(100);

    let _t = true;
    let _c = 'a';
    let _s = "a";

}
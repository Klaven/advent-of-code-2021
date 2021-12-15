use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate bit_vec;
use bit_vec::*;

fn main() {
    println!("Hello, world!");

    let mut myTup = (0,0,0,0,0,0,0,0,0,0,0,0);
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(line) = line {
                let byteLine = line.as_bytes();
                println!("{:?}",byteLine);
                let (a,b,c,d,e,f,g,h,i,j,k,l) = myTup;
                myTup = (
                    a + translate_to_ones_and_zeros(byteLine[0]),
                    b + translate_to_ones_and_zeros(byteLine[1]),
                    c + translate_to_ones_and_zeros(byteLine[2]),
                    d + translate_to_ones_and_zeros(byteLine[3]),
                    e + translate_to_ones_and_zeros(byteLine[4]),
                    f + translate_to_ones_and_zeros(byteLine[5]),
                    g + translate_to_ones_and_zeros(byteLine[6]),
                    h + translate_to_ones_and_zeros(byteLine[7]),
                    i + translate_to_ones_and_zeros(byteLine[8]),
                    j + translate_to_ones_and_zeros(byteLine[9]),
                    k + translate_to_ones_and_zeros(byteLine[10]),
                    l + translate_to_ones_and_zeros(byteLine[11]),

                )
            }
        }
    }
    let mut bv = BitVec::from_elem(16, false);
    let (a,b,c,d,e,f,g,h,i,j,k,l) = myTup;
    println!("{:?}",myTup);
    bv.set(4,round(f64::from(a)/f64::from(1000)));
    bv.set(5,round(f64::from(b)/f64::from(1000)));
    bv.set(6,round(f64::from(c)/f64::from(1000)));
    bv.set(7,round(f64::from(d)/f64::from(1000)));
    bv.set(8,round(f64::from(e)/f64::from(1000)));
    bv.set(9,round(f64::from(f)/f64::from(1000)));
    bv.set(10,round(f64::from(g)/f64::from(1000)));
    bv.set(11,round(f64::from(h)/f64::from(1000)));
    bv.set(12,round(f64::from(i)/f64::from(1000)));
    bv.set(13,round(f64::from(j)/f64::from(1000)));
    bv.set(14,round(f64::from(k)/f64::from(1000)));
    bv.set(15,round(f64::from(l)/f64::from(1000)));
    let bytes = bv.to_bytes();
    let byte_array = [bytes[0], bytes[1]];
    let onum = u16::from_be_bytes(byte_array);

    let mut bv_test = BitVec::from_elem(16, false);
    bv_test.set(14, true);
    bv_test.set(13, true);
    bv_test.set(11, true);
    //(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
    //(0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5)
    //let test_bits = bv_test.to_bytes();
    //let test_bytes = [test_bits[0], test_bits[1]];
    //let test = u16::from_be_bytes(test_bytes);
    //println!("{}",test);
    //println!("test: {:?}",bv_test);

    //let twenty_two: &[u8;1] = &[22];
    //let mut bv_test = BitVec::from_bytes(twenty_two);
    //println!("twenty two: {:?}",bv_test);

    let new_bv = custom_negate(bv);
    let eps_bytes = new_bv.to_bytes();
    let byte_array_eps = [eps_bytes[0], eps_bytes[1]];
    let eps = u16::from_be_bytes(byte_array_eps);

    println!("{}",onum);
    println!("{}",eps);
    println!("{}",u32::from(eps) * u32::from(onum));
}


fn round(input:f64) -> bool {
    if input >= 0.5 {
        return true;
    }
    return false
}

fn custom_negate(bv: BitVec) -> BitVec {
    let mut new_bv = BitVec::from_elem(16, false);
    for (i, bit) in bv.iter().enumerate() { 
        if i > 3 {
            new_bv.set(i, !bit)
        }
    }
    return new_bv
}


fn translate_to_ones_and_zeros(input:u8) -> i32 {
    if input == 48 {
        return 0;
    }
    return 1;
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
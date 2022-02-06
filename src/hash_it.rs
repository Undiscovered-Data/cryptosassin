use rug::Float;
use sha2::{Sha512, Digest};

pub fn hash_512(inword: &str) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(inword);
    let hash = hasher.finalize();
    let a_vec = hash.to_vec();
    a_vec
}


pub fn sqroots(inword: &str) -> Vec<char> {
    let arr1 = vec![0,1,4,9,16,25,36,49,64,81,100,121,144,169,196,225];
    let uvec: Vec<u8> = hash_512(inword);
    let mut ssvec: Vec<char> = Vec::new();
    let mut the_count = 0;
    for a in uvec {
        if arr1.contains(&a) { continue; }
        the_count += 1;
        let f = Float::with_val(800, a);
        let the_root = f.sqrt();
        let str_root = format!("{:x}", &the_root);
        let sss_root = &str_root[5..185];
        let char_vec: Vec<char> = sss_root.chars().collect();
        println!("{:x}", the_root);
        println!("{:?}", &char_vec);

        for a in char_vec {
            ssvec.push(a);
        }

        if the_count >= 32 { break; }
    }
    println!("{:?}", &ssvec);
    println!("{}", the_count);
    ssvec
}

pub fn shuffle_it(unshuf: Vec<char>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut vec3 = unshuf;
    let one_half = vec3.len() / 2;
    let full_len = vec3.len();

    for _ in 0..4 {
        for a in 0..one_half {
            vec1.push(vec3[a]);
        }
        for b in one_half..full_len {
            vec2.push(vec3[b]);
        }
        vec3.clear();
        for c in 0..one_half {
            vec3.push(vec1[c]);
            vec3.push(vec2[c]);
        }
    }
    println!("{:?}", &vec3);
}

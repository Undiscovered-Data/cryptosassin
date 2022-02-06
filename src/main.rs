use std::io::stdin;
mod hash_it;

fn main() {
    // Get file name and password
/*
    let mut e_crypt: String::new();
    println!("Enter e if you want to encrypt\nEnter d if you want to decrypt");
    stdin().read_line(&mut e_crypt);
    let t_crypt = e_crypt.trim();
    let mut the_file = String::new();
*/
    let mut pword = String::new();
/*
    println!("Enter the file to encrypt:");
    stdin().read_line(&mut the_file).expect("File error");
    let t_file = the_file.trim();
*/
    println!("Enter the password to encrypt file:");
    stdin().read_line(&mut pword).expect("password?");
    let t_word = pword.trim();

    // Take hash password and put in u8 Vec
    let unshVec: Vec<char> = hash_it::sqroots(t_word);
    hash_it::shuffle_it(unshVec);

/*
    // Open file and put in Vec
    let file_vec = file_opener(t_file);
    // Run encrypt - decrypt
    if t_crypt == "e" { let w_vec = crypt::encrypt(file_vec); }
    else if t_crypt == "d" { let w_vec = crypt::decrypt(file_vec); }
    else { println!("error"); }
    // Write file
    write_it(w_vec);
*/
}

/*
Has error when carie is the password and arr1 vec does not have 0.
In file hash_it
*/

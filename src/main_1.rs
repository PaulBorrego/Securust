<<<<<<< HEAD
use orion::aead;
// use orion::hazardous::aead::streaming::SecretKey;
use orion::pwhash::{self};//, PasswordHash};
// use orion::errors::UnknownCryptoError;
use std::error::Error;
// use std::fs;
use std::fs::{self, 
    // read, read_to_string, 
    File, OpenOptions};
use std::io::{stdin, Read, Write};
// use std::io;
use std::path::Path;
use std::collections::HashMap;

mod gui;


fn main() {    
    // let _ = User::new();
    // encrypt_interface();
    encrypt_interface_user();
}

pub fn interface() -> bool {
    let mut input = String::new();          //stdin stuff

    println!("Insert Password: ");
    stdin().read_line(&mut input).unwrap();     //stdin stuff
    input.pop();                                    //gets rid of \n

=======
mod gui;

use orion::aead;
use orion::pwhash::{self, PasswordHash};
use orion::hazardous::aead::streaming::SecretKey;
use orion::errors::UnknownCryptoError;
use std::error::Error;
use std::fs::{self, read_to_string, File};
use std::io::{stdin, Write};
use std::path::Path;

//common code reasons
// --input.pop()-- gets rid of the '\n' at the end of the input
//

fn main() {    
    let _ = User::new();
    // encrypt_interface();
}


//password interface, asks for password at the start so its like not super secure. Just proof of concept
pub fn interface() -> bool {
    let mut input = String::new();          //stdin stuff

    println!("Insert Password: ");
    stdin().read_line(&mut input).unwrap();     //stdin stuff
    input.pop();                                    //gets rid of \n

>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
    let password = pwhash::Password::from_slice(&input.as_bytes()).unwrap();    // sets password variable
    let hash = pwhash::hash_password(&password, 3, 1 << 16).unwrap();   //hash from password
    

    for _n in 1..3 {
        input.clear();                                      //gets rid of previous input
        println!("Enter Password:");        
        stdin().read_line(&mut input).unwrap();         //stdin stuff
        input.pop();
        if input.eq("-1") {                                 //exit on -1
            println!("Exiting password");
            return false;
        }
        match pwhash::hash_password_verify(&hash,&pwhash::Password::from_slice(input.as_bytes()).unwrap())  {   //compares set password to input
            Ok(_) => {
                println!("Password verified!");
                return true;
            },
            Err(_) => println!("Incorrect password."),
        }
    }
    false
<<<<<<< HEAD
}

// pub fn no_hash_interface(mut mappy: HashMap<String, User>) -> Option<String> {

//     let mut input = String::new();          //stdin stuff
//     println!("(N)ew User or (R)eturning? ");
//     stdin().read_line(&mut input).unwrap();     //stdin stuff
//     input.pop();                                    //gets rid of \n


//     if input.to_lowercase().chars().next().unwrap().eq(&'n') {
//         let x = User::new();
//         mappy.insert(x.0, x.1);
    
//     }
//     User::find(&mappy)
// }

// pub fn make_new_user(mappy: &HashMap<String, User>) -> bool {
//     let x = User::new();
//     mappy.insert(x.0, x.1);
//     true

// }

pub fn write_to_file(s: &[u8], a: &str) ->  Result<File, std::io::Error> {
    let mut temp = format!("{}_encrypt.txt",a);
    let i = 0;
    while Path::new(&temp).exists() {
        temp = format!("{}{}",i.to_string(), temp);
    }
    let p = Path::new(&temp);
    let mut file = File::create(p)?;
    file.write_all(s)?;
    Ok(file)
}


=======

}

//takes data and name and makes a new file
pub fn write_to_file(s: &[u8], a: &str) ->  Result<File, std::io::Error> {
    let mut temp = format!("{}_encrypt.txt",a);
    let i = 0;
    while Path::new(&temp).exists() {   //makes new files..no repeating
        temp = format!("{}{}",i.to_string(), temp);
    }
    let p = Path::new(&temp);
    let mut file = File::create(p)?;    //creates new files 
    file.write_all(s)?;                 //writes s into file
    Ok(file)
}

//Should loop and encrypt and decrypt data.
//all of the comented section was my attempt at making a username password system in which
//each of the accounts had different aead::SecretKey so multiple people could have different
//encryptions and decryptions. Ran into ownership problems
>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
pub fn encrypt_interface() -> () {
    let mut input = String::new();
    // let mut user_map = HashMap::new();

    // let v = fs::read_to_string("~/keys.tsv").unwrap().lines();
    // for s in v {
    //     user_map.insert(s.split_whitespace().next().unwrap(),User::from_tsv(s));
    // }
    
    // println!("(r)eturn user or (n)ew");
    // stdin().read_line(&mut input).unwrap();
    // let u = match input.chars().next().unwrap() {
    //     'r' => {
    //         println!("Type username");
    //         input.clear();
    //         stdin().read_line(&mut input).unwrap();
    //         input.pop();
    //         *user_map.get(&input.as_str()).unwrap()
    //     }
    //     'n' => {
    //         // let n = User::new();
    //         // let i = n.username.clone();
    //         // fs::write("~/keys.txt", User::to_tsv(&n)).expect("Failed to add tsv");
    //         // user_map.insert(i.as_str(), n);
    //         // user_map.get(&i.as_str()).unwrap()
    //         User::new()
    //     }
    //     _ => panic!("not r or n"),
    // };

    if interface() {
        let secret_key = aead::SecretKey::default();
        loop {
            input.clear();
            println!("Would you like to (e)ncrypt or (d)ecrypt");
            stdin().read_line(&mut input).unwrap();

            match input.chars().next().unwrap() {
                'e' => {
                    input.clear();
                    println!("What would you like to encrypt? (s)tring or (f)ile");
                    stdin().read_line(&mut input).unwrap();
                    
<<<<<<< HEAD
                     match input.chars().next().unwrap() {
=======
                    match input.chars().next().unwrap() {
>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
                        's' => string_encrypt(&secret_key).unwrap(),
                        'f' => file_encrypt(&secret_key).unwrap(),
                        _ => panic!("not s or f"), 
                    };
                }
                'd' => {
                    input.clear();
                    println!("What file would you like to decrypt");
                    stdin().read_line(&mut input).unwrap();
                    input.pop();

                    file_decrypt(&input, &secret_key).unwrap();
                }
                _ => panic!("not e or d"),
<<<<<<< HEAD
            };
=======
            }
>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
        }
    }
}

<<<<<<< HEAD
pub fn encrypt_interface_user() -> () {
    let mut existing_users = User::get_existing();
    // let mut input = String::new();
    // let x = no_hash_interface(&existing_users);
    let mut input = String::new();          //stdin stuff

    println!("(N)ew User or (R)eturning? ");
    stdin().read_line(&mut input).unwrap();     //stdin stuff
    input.pop();                                    //gets rid of \n

    
    if input.to_lowercase().chars().next().unwrap().eq(&'n') {
        let a = User::new();
        existing_users.insert(a.0, a.1);        
    }

    // for _ in 0..3 {
        // let mut input = String::new();
        // println!("Enter Username: ");
        // stdin().read_line(&mut input).unwrap();    
        // input.pop();
        // let a = existing_users.get(&input);
        // println!("{:?}", a);
    // }


    let x = User::find(&existing_users);

    println!("{:?}", x);    

    if x.is_some() {
        let secret_key = &existing_users.get(&x.unwrap()).unwrap().secret_password;
        loop {
            input.clear();
            println!("Would you like to (e)ncrypt or (d)ecrypt");
            stdin().read_line(&mut input).unwrap();

            match input.chars().next().unwrap() {
                'e' => {
                    input.clear();
                    println!("What would you like to encrypt? (s)tring or (f)ile");
                    stdin().read_line(&mut input).unwrap();
                    
                     match input.chars().next().unwrap() {
                        's' => string_encrypt(secret_key).unwrap(),
                        'f' => file_encrypt(secret_key).unwrap(),
                        _ => panic!("not s or f"), 
                    };
                }
                'd' => {
                    input.clear();
                    println!("What file would you like to decrypt");
                    stdin().read_line(&mut input).unwrap();
                    input.pop();

                    file_decrypt(&input, secret_key).unwrap();
                }
                _ => panic!("not e or d"),
            };
        }

    }
}

#[derive(PartialEq, Debug)]
struct User {
    username: String,
    password: String,
    secret_password: aead::SecretKey,
}
impl User {
    fn new() -> (String, User) {
        let _ = gui::main();
        let mut buffer = Vec::new();
        let mut f = OpenOptions::new()
            .read(true)
            .open("temp.txt")
            .unwrap();

        let _ = f.read_to_end(&mut buffer).unwrap();

        fs::remove_file("temp.txt").expect("Failed To Delete the file \"temp\"");
        (String::from_utf8(buffer[..32].to_vec()).unwrap(),
        User {
            username: String::from_utf8(buffer[..32].to_vec()).unwrap(),
            password:String::from_utf8(buffer[32..64].to_vec()).unwrap(),
            secret_password: aead::SecretKey::from_slice(&buffer[64..96]).unwrap()
            }
        )
        
    }

    fn get_existing() -> HashMap<String,User> {

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("info.txt")
            .unwrap();


        let mut buffer = Vec::new();
        let a = f.read_to_end(&mut buffer).expect("Reading Problem") / 97;
        let mut returning_users: HashMap<String, User> = HashMap::new();
        for i in 0..a {
            let b = i * 97;
            returning_users.insert(
                String::from_utf8(buffer[b..b+32].to_vec()).unwrap(),
                User {
                    username: String::from_utf8(buffer[b..b+32].to_vec()).unwrap(),        
                    password: String::from_utf8(buffer[b+32..b+64].to_vec()).unwrap(),
                    secret_password: aead::SecretKey::from_slice(&buffer[b+64..b+96]).unwrap(),
    
                }
            );
            
    
            // println!("{}\t{}\t{:?}",String::from_utf8(user).unwrap(),String::from_utf8(pass).unwrap(),key);
        }
        returning_users
    }
    
    // fn to_string(&self) -> String {
    //     format!("Username: |{}| Password: |{}|\n",self.username, self.password)
    // }

    fn find(mappy: &HashMap<String,User>) -> Option<String> {
        for _ in 0..3 {
            let mut input = String::new();
            println!("Enter Username: ");
            stdin().read_line(&mut input).unwrap();    
            input.pop();
            for (k,_) in mappy {
                println!("|{}:{}|",k,input);
            }
            let a = mappy.get(&input);
            if a.is_some() {
                println!("SOME");
                for _ in 0..3 {
                    let mut in2 = String::new();
                    stdin().read_line(&mut in2).unwrap();    
                    in2.pop();
    
                    if input.eq(&a.unwrap().password) {
                        return Some(input.clone());
                    }

                }
            }
            println!("Username Not found");
        }
        None
        
    }
}

pub fn string_encrypt(secret_key: &aead::SecretKey) -> Result< (), Box<dyn Error>> {
    println!("Type string you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(& mut buf).unwrap();
=======
//Encrypts string into new file
pub fn string_encrypt(secret_key: &aead::SecretKey) -> Result<(), Box<dyn Error>> {
    println!("Type string you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
    buf.pop();

    let text = aead::seal(&secret_key, buf.as_bytes())?;

<<<<<<< HEAD

=======
>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
    match write_to_file(&text, "string") {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?,
    }
}

<<<<<<< HEAD
pub fn file_encrypt(secret_key: &aead::SecretKey) -> Result<(), Box<dyn Error>> {
    println!("Type file path you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(& mut buf).unwrap();
    buf.pop();    //gets rid of \n

    let contents = fs::read_to_string(buf.clone())?;
    let text = aead::seal(&secret_key, &contents.as_bytes())?; 

=======
//Encrypts File into new File 
pub fn file_encrypt(secret_key: &aead::SecretKey) -> Result<(), Box<dyn Error>> {
    println!("Type file path you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.pop();    //gets rid of \n

    let contents = fs::read_to_string(buf.clone())?;

    let text = aead::seal(&secret_key, &contents.as_bytes())?; 

    let f = &aead::open(&secret_key, &text).unwrap();
    println!("{}",String::from_utf8_lossy(f));
>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
    match write_to_file(&text,buf.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?
    }
}

<<<<<<< HEAD
pub fn file_decrypt(s: &str, secret_key: &aead::SecretKey) -> Result<(), Box<dyn Error>> {
    let file = fs::read(s).expect("Reading problem");
    println!("{:?}", file);
=======
//Takes a new file and decrypts it into a file 
pub fn file_decrypt(s: &str, secret_key: &aead::SecretKey ) -> Result<(), Box<dyn Error>> {
    let file = fs::read(s).expect("Reading problem");
    
>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
    let open = aead::open(secret_key, &file).expect("Open problem");
    match write_to_file(&open, s) {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?
    }
<<<<<<< HEAD
}
=======
}

#[derive(PartialEq, Debug)]
struct User {
    username: String,
    password: String,
    secret_password: SecretKey,
}

impl User {
    fn new() -> User {
        let _ = gui::main();
        //im going to do something disgusting here
        // "thisSolutionIsBadIKnow.txt"
        let a = read_to_string("thisSolutionIsBadIKnow.txt").unwrap();  //i dont know why but if i combine
        let u: Vec<&str> = a.split("\t").collect();             //these two lines it doesnt work
        fs::remove_file("thisSolutionIsBadIKnow.txt").expect("Failed To Delete the file \"thisSolutionIsBadIKnow\"");

        User{
            username: u[0].to_string(),
            password: u[1].to_string(),
            secret_password: SecretKey::from_slice(u[2].as_bytes()).unwrap(),
        }
    }

    fn get_existing() -> Vec<User> {
        let mut ret: Vec<User> = vec![];
        for a in read_to_string("users.txt").unwrap().lines() {
            let u: Vec<&str> = a.split("\t").collect();
            ret.push(
                User{
                    username: u[0].to_string(),
                    password: u[1].to_string(),
                    secret_password: SecretKey::from_slice(u[2].as_bytes()).unwrap(),
                }
            )
        }
        ret
    }
    fn to_string(u: User) -> String {
        format!("Username: |{}| Password: |{}|\n",u.username, u.password)
    }
}


//--------------------------------------------------------------------------------------------

//encrypt is testing function for aead
// pub fn encrypt() -> Result<(), UnknownCryptoError>{
//     let secret_key = aead::SecretKey::default();
//     let ciphertext = aead::seal(&secret_key, b"Secret message")?;
//     let decrypted_data = aead::open(&secret_key, &ciphertext)?;
//     let text = match String::from_utf8(ciphertext) {
//         Ok(s) => s,
//         Err(_) => String::from("Error"), 
//     };
//     println!("Cipher Text: {:?}", text);
//     println!("Decpyhered Text: {:?}", decrypted_data);
//     println!("Decpyhered Text: {:?}", String::from_utf8(decrypted_data).expect("sad"));

//     Ok(())
// }

//psswd is testing function for pwhash and PasswordHash
// pub fn psswd() -> Result<(), UnknownCryptoError> {
//     let password = pwhash::Password::from_slice(b"Secret password")?;
//     let hash = pwhash::hash_password(&password, 3, 1<<16)?;
//     assert!(pwhash::hash_password_verify(&hash, &password).is_ok());

//     println!("Password: {}", PasswordHash::unprotected_as_encoded(&hash));
//     Ok(())
// }
>>>>>>> 5778aad9ac2a41430af450e3b3fb39606cd13ff3
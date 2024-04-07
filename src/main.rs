use orion::aead;
// use orion::hazardous::aead::streaming::SecretKey;
use orion::pwhash::{self};//, PasswordHash};
// use orion::errors::UnknownCryptoError;
use std::error::Error;
use std::fs::{self, 
    File, OpenOptions};
use std::io::{stdin, Read, Write};
use std::path::Path;
use std::collections::HashMap;
use pad::PadStr;


mod gui;


fn main() {    
    encrypt_interface_user();
}

pub fn interface() -> bool {
    let mut input = String::new();          //stdin stuff

    println!("Insert Password: ");
    stdin().read_line(&mut input).unwrap();     //stdin stuff
    input.pop();                                    //gets rid of \n

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
}

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

pub fn encrypt_interface() -> () {
    let mut input = String::new();

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
                    
                    match input.chars().next().unwrap() {
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
            };
        }
    }
}

pub fn makeDirIfNeeded(path: &str) -> () {
    // Check if the directory exists
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_dir() {
            println!("Found directory!.");
        } else {
            println!("A file with the same name exists, not a directory.");
        }
    } else {
        println!("No users. Creating...");
        //let users = "./users";
        match fs::create_dir(path){
            Ok(_) => println!("Made directory"),
            Err(_err) => println!("Fcuk this shit"),
        }

}
}

pub fn encrypt_interface_user() -> () {
    let mut existing_users = User::get_existing();
    let mut input = String::new();          //stdin stuff
    let path = "./users";

    // Check if the directory exists
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_dir() {
            println!("Found users!.");
        } else {
            println!("A file with the same name exists, not a directory.");
        }
    } else {
        println!("No users. Creating...");
        let users = "./users";
        match fs::create_dir(users){
            Ok(_) => println!("Made directory"),
            Err(_err) => println!("Fcuk this shit"),
        }


    }

    
    loop {
        input.clear();
        println!("(N)ew User or (R)eturning? [(b)ack will bring you back]");
        stdin().read_line(&mut input).unwrap();     //stdin stuff
        match input.to_lowercase().chars().next().unwrap() {
            'n' => {
                if input.to_lowercase().chars().next().unwrap().eq(&'n') {
                    let a = User::new();
                    existing_users.insert(a.0, a.1);        
                }
                break;
            },
            'r' => break,
            'b' => return (),
            _ => continue,
        }
    }
    

    let x = User::find(&existing_users);
    

    if x.is_some() {
        let secret_key = &existing_users.get(&x.unwrap()).unwrap().secret_password;
        loop {
            input.clear();
            println!("Would you like to (e)ncrypt or (d)ecrypt");
            stdin().read_line(&mut input).unwrap();

            match input.chars().next().unwrap() {
                'e' => {
                    loop {
                        input.clear();
                        println!("What would you like to encrypt? (s)tring or (f)ile");
                        stdin().read_line(&mut input).unwrap();
                        
                        match input.chars().next().unwrap() {
                            's' => {
                                string_encrypt(secret_key).unwrap();
                                break;
                            },
                            'f' => {
                                file_encrypt(secret_key).unwrap();
                                break;
                            },
                            'b' => break,
                            _ => println!("Did not select (s)tring or (f)ile"), 
                        };
                    }
                }
                'd' => {
                    loop {
                        input.clear();
                        println!("What file would you like to decrypt");
                        stdin().read_line(&mut input).unwrap();
                        input.pop();
                        if input.eq("b") {
                            break;
                        }
                        match file_decrypt(&input, secret_key) {
                            Ok(_) => break,
                            Err(_) => {
                                println!("File not found");
                                continue;
                            },
                        }
                    }
                    
                }
                'b' => break,
                _ => println!("Did not select (e)ncyrpt or (d)ecrypt"),
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
            .open("info/temp.txt")
            .unwrap();

        let _ = f.read_to_end(&mut buffer).unwrap();

        fs::remove_file("info/temp.txt").expect("Failed To Delete the file \"temp\"");
        
        (String::from_utf8(buffer[..32].to_vec()).unwrap(),
        // (String::from(String::from_utf8(buffer[..32].to_vec()).unwrap().trim()),
        User {
            username: String::from_utf8(buffer[..32].to_vec()).unwrap(),
            password: String::from_utf8(buffer[32..64].to_vec()).unwrap(),


            // username: String::from(String::from_utf8(buffer[..32].to_vec()).unwrap().trim()),
            // password: String::from(String::from_utf8(buffer[32..64].to_vec()).unwrap().trim()),
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
            .open("info/info.txt")
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

            // for (k,_) in mappy {
            //     println!("|{}:{}|",k,input);
            // }
            let a = mappy.get(&input.pad_to_width(32));
            if a.is_some() {
                for _ in 0..3 {
                    println!("Enter Password");
                    let mut in2 = String::new();
                    stdin().read_line(&mut in2).unwrap();    
                    in2.pop();                  
    
                    if in2.pad_to_width(32).eq(&a.unwrap().password) {
                        return Some(input.pad_to_width(32).clone());
                    }

                    println!("Password Denied");
                }
            }
            else {
                println!("Username Not found");
            }
        }
        println!("Login Failed");
        None
        
    }
}

pub fn string_encrypt(secret_key: &aead::SecretKey) -> Result< (), Box<dyn Error>> {
    println!("Type string you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(& mut buf).unwrap();
    buf.pop();

    let text = aead::seal(&secret_key, buf.as_bytes())?;


    match write_to_file(&text, "string") {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?,
    }
}

pub fn file_encrypt(secret_key: &aead::SecretKey) -> Result<(), Box<dyn Error>> {
    let contents: String;
    let mut buf = String::new();

    loop {
        println!("Type file path you'd like to be encrypted: ");
        stdin().read_line(& mut buf).unwrap();
        buf.pop();    //gets rid of \n
        match fs::read_to_string(buf.clone()) {
            Ok(a) => {
                contents = a;
                println!("Encrypted file!");
                break;
            },
            Err(_) => (),
        };
        println!("File Not Found");
    }
    let text = aead::seal(&secret_key, &contents.as_bytes())?; 

    match write_to_file(&text,buf.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?
    }
}

pub fn file_decrypt(s: &str, secret_key: &aead::SecretKey) -> Result<(), Box<dyn Error>> {
    let file = fs::read(s)?;
    let open = aead::open(secret_key, &file).expect("Open problem");
    match write_to_file(&open, s) {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?
    }
}
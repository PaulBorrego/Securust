use clap::Parser;
use paul_commit::encrypt_interface;

mod cli;
mod paul_commit;


fn main() {
    call_paul();

    let args = cli::Cli::parse(); // Adjust based on actual module/path names

    match &args.action {
        cli::Action::Encrypt { file, key, algorithm } => {
            // Call into a separate module/function for encryption
            println!("Encrypting file: {} with key: {} with algorithm: {:?}", file, key, algorithm);
            // Example: encryption::encrypt_file(file, key);
        }
        cli::Action::Decrypt { file, key, algorithm } => {
            // Call into a separate module/function for decryption
            println!("Decrypting file: {} with key: {} with algorithm: {:?}", file, key, algorithm);
            // Example: decryption::decrypt_file(file, key);
        }
        cli::Action::Key {action, file} => {
            println!("Performing {} with file {:?}", action, file)
        }
    }
}
fn call_paul() {
    encrypt_interface();
}
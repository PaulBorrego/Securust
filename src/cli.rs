/// # Module for CLI argument parsing and command handling
///
/// This module utilizes `clap`, a powerful command-line argument parser, to define and handle
/// the commands and options for the securust File Encryption and Decryption Utility. It supports
/// encrypting and decrypting files by specifying the file path and a key.
///
/// ## Commands
/// - `encrypt`: Encrypts the specified file with the provided key.
/// -- cargo run -- encrypt --file "path/to/your/file.txt" --key "yourEncryptionKey"
/// - `decrypt`: Decrypts the specified file with the provided key.
/// -- cargo run -- decrypt --file "path/to/your/encrypted/file.txt" --key "yourDecryptionKey"
///
/// Each command requires file path and key arguments. The `file` argument specifies the path to the
/// target file, while the `key` argument provides the encryption or decryption key.

use clap::{Parser, Subcommand, ValueEnum};

/// Represents the command-line interface of the securust utility.
#[derive(Parser, Debug)]
#[command(name = "securust", about = "A File Encryption and Decryption Utility", long_about = None)]
pub struct Cli {
    /// Specifies the action to perform (encrypt, decrypt, manage keys) along with associated options.
    #[command(subcommand)]
    pub(crate) action: Action,
}

/// Represents supported cryptographic algorithms.
#[derive(ValueEnum, Clone, Debug)]
pub enum Algorithm {
    AES,
    RSA,
    // Add more algorithms as needed
}


/// Enumerates the possible actions (commands) available in the securust utility.
#[derive(Subcommand, Debug)]
pub enum Action {
    /// Encrypts a file with a specified key and algorithm.
    ///
    /// The `encrypt` command requires:
    /// - `file`: Path to the file that needs to be encrypted.
    /// - `key`: The encryption key used to encrypt the file content.
    /// - `algorithm`: The encryption algorithm to use.
    Encrypt {
        /// Path to the file to encrypt.
        #[arg(short, long)]
        file: String,

        /// Encryption key used to encrypt the file.
        #[arg(short, long)]
        key: String,

        /// Cryptographic algorithm to use for encryption.
        #[arg(short, long, value_enum)]
        algorithm: Algorithm,
    },
    /// Decrypts a file with a specified key and algorithm.
    ///
    /// The `decrypt` command requires:
    /// - `file`: Path to the file that needs to be decrypted.
    /// - `key`: The decryption key used to decrypt the file content.
    /// - `algorithm`: The decryption algorithm to use.
    Decrypt {
        /// Path to the file to decrypt.
        #[arg(short, long)]
        file: String,

        /// Decryption key used to decrypt the file.
        #[arg(short, long)]
        key: String,

        /// Cryptographic algorithm to use for decryption.
        #[arg(short, long, value_enum)]
        algorithm: Algorithm,
    },
    /// Manages cryptographic keys (generate, import, export).
    ///
    /// The `key` command can be used to manage encryption keys.
    Key {
        /// Specify the action to perform on keys (generate, import, export).
        #[arg(short, long)]
        action: String,

        /// Optionally specify a file path for import/export actions.
        #[arg(short, long)]
        file: Option<String>,
    },
}

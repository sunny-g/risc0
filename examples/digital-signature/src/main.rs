// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;
use digital_signature::sign;
use sha2::{Digest, Sha256};

use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct Cli {
    #[arg()]
    message: String,

    #[arg(long)]
    passphrase: String,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Cli::parse();

    let signing_receipt = sign(&args.passphrase, &args.message).unwrap();

    println!("Inputs");
    println!("\tmessage: {:?}", args.message);
    println!("Commitment:");
    println!("\tmessage: {:?}", &signing_receipt.get_message().unwrap());
    println!("\tidentity: {:?}", &signing_receipt.get_identity().unwrap());
    println!("Integrity Checks:");
    let message_hash = &signing_receipt.get_message().unwrap();
    let expected_message_hash = Sha256::digest(args.message);
    if message_hash.as_bytes() != expected_message_hash.as_slice() {
        tracing::error!("Message commitment does not match given message!");
        std::process::exit(1);
    }
    println!("\tmessage: valid");
    if signing_receipt.verify().is_err() {
        tracing::error!("Receipt is invalid!");
        tracing::error!("{}", signing_receipt.verify().unwrap_err());
        std::process::exit(1);
    }
    println!("\treceipt: valid");
}

#[cfg(test)]
mod tests {
    use digital_signature::sign;
    use sha2::{Digest, Sha256};

    const MESSAGE: &str = "This is a signed message";
    const PASSPHRASE: &str = "passw0rd";

    #[test]
    fn main() {
        let signing_receipt = sign(&PASSPHRASE, &MESSAGE).unwrap();
        let message_hash = signing_receipt.get_message().unwrap();
        let expected_message_hash = Sha256::digest(MESSAGE);
        assert_eq!(
            message_hash.as_bytes(),
            expected_message_hash.as_slice(),
            "Message commitment does not match given message!"
        );
        assert!(
            signing_receipt.verify().is_ok(),
            "Receipt is invalid! {}",
            signing_receipt.verify().unwrap_err()
        );
    }
}

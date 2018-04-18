extern crate ether_keygen;

const USAGE: &str = "ether-secret-to-address {hex encoded secret (must be 64 chars long)}";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    }

    let secret_string = &args[1];
    if secret_string.len() != 64 {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    }

    let secret_bytes = ether_keygen::private_from_hex(secret_string);

    let address = ether_keygen::private_to_address(&secret_bytes);
    println!("{}", ether_keygen::slice_to_hex(&address));
}

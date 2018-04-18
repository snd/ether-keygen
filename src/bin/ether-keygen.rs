extern crate ether_keygen;

fn main() {
    let private_key = ether_keygen::random_private_key();
    println!("{}", ether_keygen::slice_to_hex(&private_key));

    let address = ether_keygen::private_to_address(&private_key);
    println!("{}", ether_keygen::slice_to_hex(&address));
}

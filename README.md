# ether-keygen

**no warranty. use at your own risk. don't use any of the private keys used as examples in this project**

## install

[requires rust](https://rustup.rs/)

```
git clone https://github.com/snd/ether-keygen.git
cd ether-keygen
cargo build --release
cp target/release/ether-keygen ~/bin/
cp target/release/ether-secret-to-address ~/bin/
```

## use

generate a random private key:
```
$ ether-keygen
9105be0eb20f90d71f81b37a4344df832ca788cf118b99413016d89dc9da5e7c
de181c89f334deb4575c4e4441023e7321e3b232
```
outputs hex encoded private key on first line and corresponding address on second line

generate address from hex encoded private key:
```
$ ether-secret-to-address 9105be0eb20f90d71f81b37a4344df832ca788cf118b99413016d89dc9da5e7c
de181c89f334deb4575c4e4441023e7321e3b232
```

### licensed under either of [apache-2.0](LICENSE-APACHE) ([tl;dr](https://tldrlegal.com/license/apache-license-2.0-(apache-2.0))) or [MIT](LICENSE-MIT) ([tl;dr](https://tldrlegal.com/license/mit-license)) at your option

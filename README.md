# WAAAH

### Build
 To build for Raspberry PI, one must download an ARM compiled version of OpenSSL, and run 
 ```
 PKG_CONFIG_ALLOW_CROSS=1 OPENSSL_DIR=path_to_openssl_dir cargo build --release --target=arm-unknown-linux-gnueabihf
 ```
 The OPENSSL_DIR must point to a directory with OpenSSL's include and lib folders in.
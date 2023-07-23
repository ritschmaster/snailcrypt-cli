# snailcrypt-cli

Timed release of data

## What is snailcrypt-cli?

snailcrypt-cli allows you to use encrypt arbitary text using the central key managament platform of https://snailcrypt.com using the command line.

## Installation

Place the file snailcrypt-cli somewhere accessible to your PATH environment variable.

## Manual compilation

Before you start you have to [setup the Rust toolchain](https://www.rust-lang.org/tools/install). Afterwards you can compile this repository with the following commands:

    git clone https://github.com/ritschmaster/snailcrypt-cli
    make dist

## How can I use snailcrypt-cli?

Please run `snailcrypt-cli -h` for a list of available parameters.

If you are into rust then you can directly execute snailcrypt-cli using `cargo`. After cloning the repository just substitute `snailcrypt-cli` in the examples below with `cargo run --`.

### Simple example

You may be interested in the following very simple example (can be executed in Bash):

    echo 'Hello world' | snailcrypt-cli -e "2022-11-19T17:00:00+0100" -f | snailcrypt-cli -d
    
### Generate a QR code image

If you have [qrencode](https://fukuchi.org/works/qrencode/) installed, then you can use it to generate a QR code image from your encrypted message. The QR code will receive a link to the timer of the [snailcrypt webapp](https://webapp.snailcrypt.com).

   echo 'Hello world'| snailcrypt-cli -e "2022-11-19T17:00:00+0100" -f -u | qrencode -o message.png

## What is the license of snailcrypt-cli?

snailcrypt-cli is licensed under the GPLv2. Please see the included LICENSE file for more information.

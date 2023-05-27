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

You may be interested in the following example (can be executed in Bash):

    echo 'Hello world' | snailcrypt-cli -e "2022-11-19T17:00:00+0100" -f | snailcrypt-cli -d

## What is the license of snailcrypt-cli?

snailcrypt-cli is licensed under the MIT license. Please see the included LICENSE.txt file for more information.

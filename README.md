# Password Manager

This is a rough password manager written in Rust. It can generate and save passwords
as a (key, value) pair to a hidden file on your device.

## Usage

To generate a password, use the `g` command:
```sh
$ cargo run g gatech
$ cargo run g gatech 10
```
The parameters are `g <NAME> [OPT: length]`. This generates a password for the
key "NAME" with either specified or random length (the default length will be between 8 to 15).
If the key already was in the map, then this command will generate a new password and overwrite the old
one.

To look up a password, use the `o` command:
```sh
$ cargo run o gatech
hHifhejvnUQn
```
This prints to the console the saved password that has been put into the password map
and saved on the device.

To add a password that was not generated using the `g` command, you can use the `a` command:
```sh
$ cargo run a gatech 'MYPASSWORDLOL'
$ cargo run o gatech
MYPASSWORDLOL
```

To delete a password, use the `d` command:
```sh
$ cargo run o gatech
hHifhejvnUQn
$ cargo run d gatech
$ cargo run o gatech
Password not found
```
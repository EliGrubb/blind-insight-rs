# blind-proxy-rs

A Rust implementation of Blind Insight's [Blind Proxy](https://docs.blindinsight.io/getting-started/using-the-blind-proxy/).

>[!warning] Tread lightly, use at your own risk.
This code is not even pre-beta, let alone ready for production use.
Contributions welcome!

## Implementation Status
- [X] --help
- [X] keyring
    - [X] create
    - [X] inspect
    - [ ] move seed phrase to SecretString structure for memory safety
- [X] login
    - [X] validate credentials with blind insight api
    - [X] store returned sessionID for validation across sessions
- [ ] handling client-side encryption/decryption
    - [ ] investigate rust pgp library options
        - rpgp vs sequoia
- [X] organization
    - [X] list organization
    - [X] pretty print organization data
- [ ] dataset
    - [ ] create dataset
- [ ] schema
    - [ ] create schema
- [ ] records
    - [ ] create records
    - [ ] list records

## Future Work

- integrate keyring with KMS and HSMs

# blind-proxy-rs

A Rust implementation of Blind Insight's [Blind Proxy](https://docs.blindinsight.io/getting-started/using-the-blind-proxy/).

## Implementation Status
- [X] --help
- [ ] keyring
    - [ ] create
        - [ ] create seed phrase from 256-bit entropy using bip39
            - [ ] hand roll or use existing rust lib?
        - [ ] store in system password manager
    - [ ] inspect
- [ ] login
    - [ ] investigate rust pgp library options
        - rpgp vs sequoia
- [ ] handling client-side encryption/decryption
- [ ] organizations
    - [ ] list organizations
- [ ] dataset
    - [ ] create dataset
- [ ] schema
    - [ ] create schema
- [ ] records
    - [ ] create records
    - [ ] list records


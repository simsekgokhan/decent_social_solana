# decent_social (Decentralized Social Media DApp)

This repository demonstrates how to build/deploy/use/test an initial version of a decentralized social media program (smart contract) and its front end code on the Solana blockchain.

Summary of work done:
- Solana tools installed from scratch on Ubuntu 20.04, devnet configured, account created (all are documented in [section 2. Setup](README.md#2-setup) below)
- [decent_social](program) Solana program (smart contract)   
- [front_end](front_end) demo app. (client, utility and main.rs functions)
- [Integration tests](front_end/src/integration_tests)
- Unit tests todo
- CI testing todo

This repo consists of two independent projects, for convinience they are placed together.   
`program` directory has Solana program (smart contract).  
`front_end` directory has the client side app.  

## 1. How to use
Hint: For first time users, see section 2. Setup below.

#### Build & Deploy Solana program (smart contract)

```
cd program
cargo build-bpf
solana program deploy target/deploy/decent_social.so 
```

#### Use deployed Solana program 
```
cd front_end
cargo r user10
    >> Connected to remote solana node running version (1.16.15).
    
    >> Create account for program to read/write its data...
    ... creating program derived account
    --- result : ()
    
    >> Info
    User   : H9dYcCxtUyTSancZSxYmqQwzDL3F5e5tR9KkQWwndjAr
    Balance: 3.8330816 Sol (3_833_081_600 lamports)
    Program: J9xLr2gjyFMpczfWzshmZVZewm1wYES2GNHHznr3Xt8T
    PDA    : 3NqMxjwgK2t3VUr39FLccEWkU573d4jz3EQCMZKBQjun
      (aka Program's data account to read/write)
      (aka Derived addr for a given user and program combination)
    PDA seed: user10
    
    >> Creating new user profile onchain...
    --- result : Ok(())
    
    >> Reading chain data...
    
    Program Object for account seed 'user10':
    UserProfile {
        user_id: 3NqMxjwgK2t3VUr39FLccEWkU573d4jz3EQCMZKBQjun,
        followers: 100,
        blocked_account: false,
    }
    
    End
```


## 2. Setup

#### Versions used:  
solana-cli 1.17.1   
20.04.1-Ubuntu LTS  

#### Install Solana CLI tools

`sh -c "$(curl -sSfL https://release.solana.com/v1.17.1/install)"`

Src: https://docs.solana.com/cli/install-solana-cli-tools

#### Create Account and connect to Devnet

```
mkdir ~/my-solana-wallet
solana-keygen new --outfile ~/my-solana-wallet/my-keypair.json
```
Src: https://docs.solana.com/wallet-guide/file-system-wallet

Set Devnet config
```
solana config get
solana config set --url https://api.devnet.solana.com
```
Src: https://docs.solana.com/cli/choose-a-cluster

Make sure config has your account   
Change `keypair_path` value with your keypair path: 
```
vim ~/.config/solana/cli/config.yml  
keypair_path: /home/<user>/my-solana-wallet/my-keypair.json
```
#### Make sure you have some SOL, helper cmds: 
```
solana balance
3.834235 SOL

solana address
H9dYcCxtUyTSancZSxYmqQwzDL3F5e5tR9KkQWwndjAr

solana airdrop 2 
or https://solfaucet.com (select Devnet)
```

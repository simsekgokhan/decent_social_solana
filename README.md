# Decentralized Social App (decent_social)

## 1. Setup

#### Versions:  
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


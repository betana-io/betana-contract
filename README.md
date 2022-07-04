# betana-contract

- lib.rs: registering modules
- entrypoint.rs: entrypoint to the program
- instruction.rs: program API, (de)serializing instruction data
- processor.rs: program logic
- state.rs: program objects, (de)serializing state
- error.rs: program specific errors
- events.rs: event for each function 

# Solana Config 

- solana config set --url localhost 

Config File: /Users/teddy/.config/solana/cli/config.yml
RPC URL: http://localhost:8899 
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /Users/teddy/.config/solana/id.json 
Commitment: confirmed 


- solana config set --url devnet

Config File: /Users/teddy/.config/solana/cli/config.yml
RPC URL: https://api.devnet.solana.com 
WebSocket URL: wss://api.devnet.solana.com/ (computed)
Keypair Path: /Users/teddy/.config/solana/id.json 
Commitment: confirmed 


# Keypair path

pubkey: 3p6pfLrm5Lgx1J2p9byQCCznPtvJ7o1uUaryAGMkT4BY
- Save this seed phrase and your BIP39 passphrase to recover your new keypair:
olympic math hobby pistol happy siren grit quantum change elder sense universe


# Current local wallet address : 

6ui5RCsbD1jUB7q5wyGFGt2ZTkQ2h9txTBsjwWnUMThF

- solana account 6ui5RCsbD1jUB7q5wyGFGt2ZTkQ2h9txTBsjwWnUMThF

Public Key: 6ui5RCsbD1jUB7q5wyGFGt2ZTkQ2h9txTBsjwWnUMThF
Balance: 2 SOL
Owner: 11111111111111111111111111111111
Executable: false
Rent Epoch: 316
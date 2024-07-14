# DaoVotingProgram
Develop a DAO voting program using Anchor. This program should allow users to vote on proposals and display results. Optionally, implement "privacy" voting using Zero-Knowledge (ZK) proofs or verifiable compute. Reward points should be given to users for participation.

 - Create a DAO voting system using Anchor.

 - Implement a voting system and display the results.

 - Optionally, add privacy voting using ZK proofs or verifiable compute.

 - Reward points to users for voting participation.

## Requirements

  <ul>
    <li>Rust installation: <a href="https://www.rust-lang.org/tools/install">here</a></li>
    <li>Solana installation: <a href="https://docs.solana.com/cli/install-solana-cli-tools">here</a></li>
    <li>Yarn installation: <a href="https://yarnpkg.com/getting-started/install">here</a></li>
    <li>Anchor installation: <a href="https://www.anchor-lang.com/docs/installation">here</a>
    <li>Git installation: <a href="https://git-scm.com/book/en/v2/Getting-Started-Installing-Git">here</a>
  </ul>


## Getting Started

### Cloning project

```bash
git clone https://github.com/starlingvibes/dao-voting-program.git
code dao-voting-program
```
### Creating local wallet

```bash
solana-keygen new 
```

<h4>Verify keypair</h4>

```bash
solana-keygen pubkey ~/.config/solana/id.json
```

<h5>Output</h5>

```bash
GLjfcSPTeV6toZf7ju64boymUFrgLj6HvARfDLtNhNTC
```

```bash
solana-keygen verify <PUBKEY> ~/.config/solana/id.json
```

<h3>Anchor.toml</h3>

```
[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

### Building

```bash
yarn
```

```bash
anchor build
anchor keys list
```
  Take the output of program id. Copy and paste it into Anchor.toml ```dao_voting_program = "624QkTgSZBWeJHm9aeNuANTJY67M4742zCnEfACg2wnM"``` and ```declare_id!("624QkTgSZBWeJHm9aeNuANTJY67M4742zCnEfACg2wnM");``` here.

Build again

```bash
anchor build
```

### Test

```bash
anchor test
```
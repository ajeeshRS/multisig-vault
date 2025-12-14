# Multisig Vault 

A multisig vault on Solana built using Anchor. Allows multiple owners to manage a shared vault with deposit, withdrawal proposals, approval, and execution.

---

## Features

- **InitializeVault**: Create a vault PDA and multisig configuration.
- **Deposit**: Owners can deposit lamports into the vault.
- **ProposeWithdrawal**: Any owner can propose a withdrawal with a recipient and amount.
- **ApproveWithdrawal**: Other owners approve the proposal.
- **ExecuteTx**: Once threshold approvals are reached, the proposal is executed, transferring lamports from the vault to the recipient.

## Local Setup

### Prerequisites

- Rust
- Solana CLI
- Anchor

---

### Clone the Repository

```bash
git clone https://github.com/ajeeshRS/multisig-vault
cd multisig-vault
```

### Build the Program

```
anchor build
```

### Deploy the Program

```
anchor deploy
```

## Security Notes

> ⚠️ **Note:** Tests are **not implemented yet**.
- This contract is experimental
- Not audited
- Do not deploy to mainnet without proper testing and auditing

## License

MIT

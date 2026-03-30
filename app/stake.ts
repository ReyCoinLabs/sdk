# 📁 app/stake.ts

```ts
import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.ReyStaking;

export async function stake(stakingAccount, userAccount, amount) {
  await program.methods
    .stake(new anchor.BN(amount))
    .accounts({
      stakingAccount,
      userAccount,
    })
    .rpc();
}

export async function unstake(stakingAccount, userAccount, amount) {
  await program.methods
    .unstake(new anchor.BN(amount))
    .accounts({
      stakingAccount,
      userAccount,
    })
    .rpc();
}
```

# 📁 tests/rey_staking.ts

```ts
import * as anchor from "@coral-xyz/anchor";

describe("rey_staking", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ReyStaking;

  it("Initialize staking", async () => {
    const stakingAccount = anchor.web3.Keypair.generate();

    await program.methods
      .initialize()
      .accounts({
        stakingAccount: stakingAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([stakingAccount])
      .rpc();

    console.log("Initialized");
  });
});
```

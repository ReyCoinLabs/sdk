# 📁 migrations/deploy.ts

```ts
import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider) {
  anchor.setProvider(provider);

  const program = anchor.workspace.ReyStaking;
  console.log("Deployed:", program.programId.toString());
};
```

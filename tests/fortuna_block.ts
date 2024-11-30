import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FortunaBlock } from "../target/types/fortuna_block";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("fortuna_block", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const authority = provider.wallet;

  const program = anchor.workspace.FortunaBlock as Program<FortunaBlock>;

  it("Initializes the lottery configs", async () => {
    const tx = await program.methods
      .initializeConfig(new BN(0), new BN(1832806466), new BN(10000))
      .rpc();

    console.log("Lottery Configs Initliaze Tx::", tx);
  });

  it("Initializes the lottery account", async() => {
    const tx = await program.methods
      .initializeLotteryAccounts()
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .rpc({commitment: 'confirmed', skipPreflight: true});

    console.log("Lottery Accounts Initialize Tx::", tx)
  })
  
});

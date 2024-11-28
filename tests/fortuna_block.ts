import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FortunaBlock } from "../target/types/fortuna_block";

describe("fortuna_block", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const authority = provider.wallet;

  const program = anchor.workspace.FortunaBlock as Program<FortunaBlock>;

  it("Initializes the token lottery account", async () => {
    const tx = await program.methods
      .initialize(new BN(0), new BN(1832806466), new BN(10000))
      .rpc();

    console.log("Token Lottery Initliaze Tx::", tx);
  });
});

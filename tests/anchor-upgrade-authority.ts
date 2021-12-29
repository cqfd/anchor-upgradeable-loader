import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorUpgradeAuthority } from "../target/types/anchor_upgrade_authority";

describe("anchor-upgrade-authority", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  // @ts-ignore (anchor ts bug :/)
  const program = anchor.workspace
    .AnchorUpgradeAuthority as Program<AnchorUpgradeAuthority>;

  it("can read its own upgradeable loader state", async () => {
    const [programAccount, bump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [program.programId.toBuffer()],
        new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
      );
    // Add your test here.
    await program.rpc.readUpgradeAuthority({
      accounts: {
        programAccount: programAccount,
      },
    });
  });
});

import * as anchor from "@coral-xyz/anchor";

describe("helloworld", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.helloworld;

  it("Says hello!", async () => {
    const tx = await program.methods.sayHello().rpc();
    console.log("✅ TX Signature:", tx);
  });
});

const anchor = require("@project-serum/anchor");
const assert = require("assert");

describe("personal_token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PersonalToken;

  it("Initializes correctly", async () => {
    const tx = await program.rpc.initialize();
    console.log("Transaction signature", tx);
  });

  it("Transfers with tax", async () => {
    const tx = await program.rpc.transferWithTax(new anchor.BN(1000), {
      accounts: {
        sender: senderPublicKey,
        recipient: recipientPublicKey,
        taxAccount1: taxAccount1PublicKey,
        taxAccount2: taxAccount2PublicKey,
        taxAccount3: taxAccount3PublicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });
    console.log("Transaction signature", tx);
    
  });
});

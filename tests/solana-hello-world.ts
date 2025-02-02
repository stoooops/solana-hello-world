import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaHelloWorld } from "../target/types/solana_hello_world";
import * as assert from "assert";
const { SystemProgram } = anchor.web3;

describe("solana-hello-world", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaHelloWorld as Program<SolanaHelloWorld>;

  console.log("program.programId.toBase58():", program.programId.toBase58());
  console.log("SystemProgram.programId:", SystemProgram.programId);

  it("Can create a message", async () => {
    const message = anchor.web3.Keypair.generate();
    const messageContent = "Hello World!";
    await program.rpc.createMessage(messageContent, {
      accounts: {
        message: message.publicKey,
        author: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [message],
    });
    // await program.methods.createMessage(messageContent).rpc();

    const messageAccount = await program.account.message.fetch(
      message.publicKey
    );

    assert.equal(
      messageAccount.author.toBase58(),
      provider.wallet.publicKey.toBase58()
    );
    assert.equal(messageAccount.content, messageContent);
    assert.ok(messageAccount.timestamp);
  });

  it("Can create and then update a message", async () => {
    const message = anchor.web3.Keypair.generate();
    const messageContent = "Hello World!";
    // print programId
    await program.rpc.createMessage(messageContent, {
      accounts: {
        message: message.publicKey,
        author: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [message],
    });

    const updatedMessageContent = "Solana is cool!";
    await program.rpc.updateMessage(updatedMessageContent, {
      accounts: {
        message: message.publicKey,
        author: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    const messageAccount = await program.account.message.fetch(
      message.publicKey
    );

    assert.equal(
      messageAccount.author.toBase58(),
      provider.wallet.publicKey.toBase58()
    );
    assert.notEqual(messageAccount.content, messageContent);
    assert.equal(messageAccount.content, updatedMessageContent);
    assert.ok(messageAccount.timestamp);
  });
});

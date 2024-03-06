import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { Monitor } from "../target/types/monitor";
import { assert } from "chai";

describe("monitor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());
  const wallet = provider.wallet as Wallet;
  const program = anchor.workspace.Monitor as Program<Monitor>;
  const get_config_account = ( name: string) : anchor.web3.PublicKey => {
    return (
        anchor.web3.PublicKey.findProgramAddressSync(
            [
              Buffer.from("oracle-config"),
              Buffer.from(name),
            ],
            program.programId
        )
    )[0];
  };
  const short_description: string = "This is a description to the oracle config account...";
  const authority_pubKey_string = "BH1VCRN52ZZeuedMkEukJDiZK58sjMzcRbYw7cADHQyg";
  const authority_pubKey = new anchor.web3.PublicKey(authority_pubKey_string);
  const existed_config_pubkey = get_config_account("22QVCPu62x2Z3abSdgHzpU4PyFttu9u");

  // Positive Test Cases
  it("Should: The oracle config account is initialized!", async () => {
    // Add your test here.
    const randomKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    const random_name = randomKey.publicKey.toBase58().slice(0,31);
    console.log(random_name)
    let total_phase = 2;
    console.log("Authority Account pubKey: ", authority_pubKey.toBase58());
    const config_pubkey = get_config_account(random_name);
    console.log("Config Account pubKey: ", config_pubkey.toBase58());
    const tx = await program.methods.initializeOracleConfig(
        random_name,
        short_description,
        total_phase
    ).accounts({
      config: config_pubkey,
      user: wallet.publicKey,
      authorityPubkey: authority_pubKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should: Add authority to Oracle Configs", async () => {
    const authority_pubKey_string = "4UivMdU5ecGyEa57WWMi15K5EnGGQVXmsp2j1cmDVB6W";
    const authority_pubKey = new anchor.web3.PublicKey(authority_pubKey_string);
    const tx = await program.methods.addAuthorityToOracleConfig().accounts(
        {
          config: existed_config_pubkey,
          user: wallet.publicKey,
          authorityPubkey: authority_pubKey,
        }
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should: Remove authority from Oracle Configs", async () => {
    const authority_pubKey_string = "4UivMdU5ecGyEa57WWMi15K5EnGGQVXmsp2j1cmDVB6W";
    const authority_pubKey = new anchor.web3.PublicKey(authority_pubKey_string);
    const tx = await program.methods.removeAuthorityFromOracleConfig().accounts(
        {
          config: existed_config_pubkey,
          user: wallet.publicKey,
          authorityPubkey: authority_pubKey,
        }
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  // Negative Test Cases
  it("Should not: The oracle config account won't initialized because using the existed config name", async () => {
    let total_phase = 4;
    const authority_pubKey_string = "BH1VCRN52ZZeuedMkEukJDiZK58sjMzcRbYw7cADHQyg";
    const authority_pubKey = new anchor.web3.PublicKey(authority_pubKey_string);
    console.log("Authority Account pubKey: ", authority_pubKey.toBase58());
    console.log("Config Account pubKey: ", existed_config_pubkey.toBase58());
    try {
      const tx = await program.methods.initializeOracleConfig(
          "22QVCPu62x2Z3abSdgHzpU4PyFttu9u",
          short_description,
          total_phase
      ).accounts({
        config: existed_config_pubkey,
        user: wallet.publicKey,
        authorityPubkey: authority_pubKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      console.log("Your transaction signature", tx);
      assert.fail("The transaction should fail but it didn't.");
    } catch (err) {
      // console.error("error found:", err)
      if (err instanceof anchor.web3.SendTransactionError) {
        if (err.logs && err.logs.some(log => log.includes("already in use"))) {
          console.log("The account has already been initialized.");
        } else {
          console.error("Unexpected error type:", err);
          assert.fail("Unexpected error type");
        }
      } else {
        console.error("Unexpected error type:", err);
        assert.fail("Unexpected error type");
      }
    }
  });

  it("Should not: The oracle config account won't initialized because long description.", async () => {
    const long_description: string = short_description.repeat(5);
    const randomKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    const random_name = randomKey.publicKey.toBase58().slice(0,31);
    console.log(random_name)
    let total_phase = 2;
    console.log("Authority Account pubKey: ", authority_pubKey.toBase58());
    const config_pubkey = get_config_account(random_name);
    console.log("Config Account pubKey: ", config_pubkey.toBase58());
    try {
      const tx = await program.methods.initializeOracleConfig(
          random_name,
          long_description,
          total_phase
      ).accounts({
        config: config_pubkey,
        user: wallet.publicKey,
        authorityPubkey: authority_pubKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      console.log("Your transaction signature", tx);
      assert.fail("The transaction should fail but it didn't.");
    } catch (_err) {
      console.error("error found:", _err)
      assert.isTrue(_err instanceof anchor.AnchorError);
      const err: anchor.AnchorError = _err;
      assert.strictEqual(err.error.errorCode.number, 6007);
    }
  });

  it("Should not: Add authority to Oracle Configs should be failed due to long list", async () => {
    const pubkey_list = ["4UivMdU5ecGyEa57WWMi15K5EnGGQVXmsp2j1cmDVB6W",
      "4aFHwSUuuwUBV2SjbYfeXQwiKPAPhsVFvY2mVp2V9b4v",
      "XHM6R2fsaCMiND9vBeUH8SM5MkQ4sUKTWNGqBAPuJUb",
      "BkFom9SJ6KDgF2aX6cBKhz9DViCVACEyDwZH5ycHEBQ5",
      "FxZw4XfNs9gPSq6Pq3uFC1uPBPeKi3Pe5mCMPDEjbV3h"];
    try {
      let tx = new anchor.web3.Transaction;
      for (let i = 0; i < pubkey_list.length; i ++ ) {
        let authority_pubKey =  new anchor.web3.PublicKey(pubkey_list[i])
        let instruction = await program.methods.addAuthorityToOracleConfig().accounts(
            {
              config: existed_config_pubkey,
              user: wallet.publicKey,
              authorityPubkey: authority_pubKey,
            }
        ).instruction();
        tx.add(instruction);
      }
      const res = await program.provider.sendAndConfirm(
          tx,
          [wallet.payer]
      );
      console.log("Account: ", res);
      assert.fail("The transaction should fail but it didn't.");
    } catch (_err) {
      console.error("error found:", _err)
      assert.isTrue(_err instanceof anchor.web3.SendTransactionError);
      const err: anchor.web3.SendTransactionError = _err;
      if (err.logs && err.logs.some(log => log.includes("Authorities limit reached"))) {
        console.log("Authorities limit reached.");
      } else {
        console.error("Unexpected error type:", err);
        assert.fail("Unexpected error type");
      }
    }
  });

  it("Should not: Remove authority from Oracle Configs should be failed due to authority not found", async () => {
    const authority_pubKey_string = "3o16HLp87MR5DyAecAxSr6VLfe9cotqXmBDY17bF4XGK";
    const authority_pubKey = new anchor.web3.PublicKey(authority_pubKey_string);
    try {
      const tx = await program.methods.removeAuthorityFromOracleConfig().accounts(
          {
            config: existed_config_pubkey,
            user: wallet.publicKey,
            authorityPubkey: authority_pubKey,
          }
      ).rpc();
      console.log("Your transaction signature", tx);
      assert.fail("The transaction should fail but it didn't.");
    } catch (_err) {
      console.error("error found:", _err)
      assert.isTrue(_err instanceof anchor.AnchorError);
      const err: anchor.AnchorError = _err;
      assert.strictEqual(err.error.errorCode.number, 6009);
    }
  });
});

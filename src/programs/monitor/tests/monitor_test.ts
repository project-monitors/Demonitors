import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { Monitor } from "../target/types/monitor";
import {assert} from "chai";
import {Commitment} from "@solana/web3.js";

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

  const getOracleDataDetails = (config_pubkey: anchor.web3.PublicKey) : { dataAccount: anchor.web3.PublicKey, bump: number } => {
    const [dataAccount, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("oracle-data"),
          config_pubkey.toBuffer(),
        ],
        program.programId
    );

    return { dataAccount, bump };
  };

  const short_description: string = "This is a description to the oracle config account...";
  const authority_pubKey_string = "BH1VCRN52ZZeuedMkEukJDiZK58sjMzcRbYw7cADHQyg";
  const authority_pubKey = new anchor.web3.PublicKey(authority_pubKey_string);
  const existed_config_name = "22QVCPu62x2Z3abSdgHzpU4PyFttu9u";
  const existed_config_pubkey = get_config_account(existed_config_name);
  const randomKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  const random_name = randomKey.publicKey.toBase58().slice(0,31);
  console.log(random_name)

  // Positive Test Cases
  it("Should: The oracle config account is initialized!", async () => {
    let total_phase = 2;
    const existed_config_account_info = await
        provider.connection.getAccountInfo(existed_config_pubkey);
    // Initialize the default config account for other test cases;
    if (existed_config_account_info === null) {
      console.log("Create test config account: ", existed_config_pubkey.toBase58())
      const tx = await program.methods.initializeOracleConfig(
          existed_config_name,
          short_description,
          total_phase
      ).accounts({
            config: existed_config_pubkey,
            user: wallet.publicKey,
            authorityPubkey: wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          }).rpc();
      console.log("Create default config signature", tx);
    }

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
      authorityPubkey: wallet.publicKey,
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

  it("Should: Initialize Oracle Data Account", async () => {
    const existed_data_pubkey =  getOracleDataDetails(existed_config_pubkey).dataAccount;
    const existed_config_account_info = await
        provider.connection.getAccountInfo(existed_config_pubkey);
    const existed_data_account_info = await
        provider.connection.getAccountInfo(existed_data_pubkey);
    if (existed_config_account_info === null) {
      assert.fail("The default config account is not initialized");
    }
    // Initialize the default data account for other test cases;
    if (existed_data_account_info === null) {
      const tx = await program.methods.initializeOracleData().accounts({
            config: existed_config_pubkey,
            oracle: existed_data_pubkey,
            user: wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          }
      ).rpc();
      console.log("Initialize default oracle data account signature", tx);
    }

    const config_pubkey = get_config_account(random_name);
    const data_pubkey = getOracleDataDetails(config_pubkey).dataAccount;
    const tx = await program.methods.initializeOracleData().accounts(
        {
          config: config_pubkey,
          oracle: data_pubkey,
          user: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should: Set Oracle Data Account", async () => {
    const existed_data_pubkey =  getOracleDataDetails(existed_config_pubkey).dataAccount;
    const bump = getOracleDataDetails(existed_config_pubkey).bump;
    const existed_config_account_info = await
        provider.connection.getAccountInfo(existed_config_pubkey);
    const existed_data_account_info = await
        provider.connection.getAccountInfo(existed_data_pubkey);
    if (existed_config_account_info === null || existed_data_account_info === null) {
      assert.fail("The default config account or data account is not initialized");
    }
    const phase = 1;
    const raw_data = 92;
    const decimals = 0;


    const subscriptionId = program.addEventListener("SetOracleDataEvent", (event, slot) => {
      console.log('Event data:', event);
      console.log('Slot:', slot);
    });
    const tx = await program.methods.setOracleData(
        phase,
        new anchor.BN(raw_data),
        decimals,
        bump
    ).accounts({
      config: existed_config_pubkey,
      oracle: existed_data_pubkey,
      user: wallet.publicKey,
    }).rpc()

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,},
        "confirmed" );

    // fetch and read the data
    const oracle_data = await program.account.oracleData.fetch(existed_data_pubkey);
    console.log('Deserialized account data:', oracle_data);
    await program.removeEventListener(subscriptionId);

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

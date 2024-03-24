import * as anchor from "@coral-xyz/anchor";
import {Program, Wallet} from "@coral-xyz/anchor";
import {Factory} from "../target/types/factory";
import {Monitor} from "../../monitor/target/types/monitor";
import {readFileSync} from 'fs';
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    getAssociatedTokenAddress,
    TOKEN_2022_PROGRAM_ID,
    getMintLen,
    ExtensionType, transfer
} from "@solana/spl-token";
import {reverseSerializer} from "@metaplex-foundation/umi";


describe("factory", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(anchor.AnchorProvider.env());
    const wallet = provider.wallet as Wallet;
    const program = anchor.workspace.Factory as Program<Factory>;
    const monitor_program =  anchor.workspace.Monitor as Program<Monitor>;
    const GLOBAL_CONFIG_SEED = "global_config";
    const VISION_MINING_SEED = "ft_vision_mining_pda";
    const EVENT_MINING_SEED = "ft_event_mining_pda";
    const STAKE_MINING_SEED = "ft_stake_mining_pda";
    const MINT_SEED = "mint";
    const MINT_CONFIG_SEED = "mint_config";
    const AUTHORITY_SEED = "authority";
    const SBT_COLLECTION_SEED = "collection";
    const MPL_TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    const MONITOR_PROGRAM_ID = new anchor.web3.PublicKey(
        "DQtL5gnrsA1e6vXrFSCTU87DHj6MBmHoZoL3bsh4uFPz")
    const EDITION_MARKER_BIT_SIZE = 248;


    const get_pda = (name: string): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from(name),
            ],
            program.programId
        );

        return {account_pubkey, bump}
    };

    const get_authority = (mint_key: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from(AUTHORITY_SEED),
                mint_key.toBuffer(),
            ],
            program.programId
        );
        return {account_pubkey, bump}
    }

    const get_metadata = (mint_key: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
                mint_key.toBuffer(),
            ],
            MPL_TOKEN_METADATA_PROGRAM_ID
        );
        return {account_pubkey, bump}
    }

    const get_master_edition = (mint_key: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
                mint_key.toBuffer(),
                Buffer.from("edition")
            ],
            MPL_TOKEN_METADATA_PROGRAM_ID
        );
        return {account_pubkey, bump}
    }

    const get_oracle_config = (name: string): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("oracle-config"),
                Buffer.from(name),
            ],
            MONITOR_PROGRAM_ID
        );
        return {account_pubkey, bump}
    }

    const get_oracle_data = (config_pubkey: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("oracle-data"),
                config_pubkey.toBuffer(),
            ],
            MONITOR_PROGRAM_ID
        );
        return {account_pubkey, bump}
    }

    const get_event_config = (config_pubkey: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("event"),
                config_pubkey.toBuffer(),
            ],
            program.programId
        );
        return {account_pubkey, bump}
    }

    const get_event_market = (config_pubkey: anchor.web3.PublicKey, index: anchor.BN): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const indexBuffer= index.toBuffer('be', 8);

        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("event_market"),
                config_pubkey.toBuffer(),
                indexBuffer
            ],
            program.programId
        );
        return {account_pubkey, bump}
    }

    const get_sbt_mint = (user_pubkey: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("sbt_mint"),
                user_pubkey.toBuffer(),
            ],
            program.programId
        );
        return {account_pubkey, bump}
    }

    const get_sbt_event_mint = (event_config: anchor.web3.PublicKey, option: number): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const option_BN = new anchor.BN(option);
        const option_buffer = option_BN.toBuffer('be', 1);
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("sbt_mint"),
                event_config.toBuffer(),
                option_buffer
            ],
            program.programId
        );
        return {account_pubkey, bump}
    }

    const get_sbt_event_edition_mint = (event_config: anchor.web3.PublicKey, option: number, user_pubkey: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const option_BN = new anchor.BN(option);
        const option_buffer = option_BN.toBuffer('be', 1);
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("sbt_mint"),
                event_config.toBuffer(),
                option_buffer,
                user_pubkey.toBuffer(),
            ],
            program.programId
        );
        return {account_pubkey, bump}
    }

    const get_edition_mark_pda = (master_edition_mint: anchor.web3.PublicKey, edition: number): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        let editionNumber = new anchor.BN(Math.floor(edition/EDITION_MARKER_BIT_SIZE));
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync([
            Buffer.from("metadata"),
            MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            master_edition_mint.toBuffer(),
            Buffer.from("edition"),
            Buffer.from(editionNumber.toString())
            ], MPL_TOKEN_METADATA_PROGRAM_ID
        )
        return {account_pubkey, bump}
    }

    const get_marker = (
        user_pubkey: anchor.web3.PublicKey,
        sbt_mint_pubkey: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
            const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("marker"),
                    user_pubkey.toBuffer(),
                    sbt_mint_pubkey.toBuffer(),
                ],
                program.programId
            );
            return {account_pubkey, bump}
    }

    const get_user_position = (
        event_market_pubkey: anchor.web3.PublicKey,
        user_pubkey: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_position"),
                event_market_pubkey.toBuffer(),
                user_pubkey.toBuffer(),
            ],
            program.programId
        );
        return {account_pubkey, bump}
    }

    const get_event_position = (
        event_market_pubkey: anchor.web3.PublicKey,
        option_index: number): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const option_bn = new anchor.BN(option_index);
        const option_buffer = option_bn.toBuffer("be", 1)
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("event_position"),
                event_market_pubkey.toBuffer(),
                option_buffer
            ],
            program.programId
        );
        return {account_pubkey, bump}
    }

    async function sign_with_governor(transaction: anchor.web3.Transaction, governor_keypair: anchor.web3.Keypair) {
        let latestBlockHash = await provider.connection.getLatestBlockhash();
        transaction.recentBlockhash = latestBlockHash.blockhash;
        transaction.feePayer = wallet.publicKey;
        transaction.partialSign(wallet.payer);
        transaction.partialSign(governor_keypair);

        const txId = await provider.sendAndConfirm(transaction).catch(e => console.error(e));
        console.log("Transaction signature: ", txId);

        latestBlockHash = await provider.connection.getLatestBlockhash();

        // @ts-ignore
        await provider.connection.confirmTransaction({
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: txId},
            "confirmed" );
    }

    async function sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }


    const existed_global_config_pubkey = get_pda(GLOBAL_CONFIG_SEED).account_pubkey;
    const existed_config_name = "22QVCPu62x2Z3abSdgHzpU4PyFttu9u";
    const existed_config_pubkey = get_oracle_config(existed_config_name).account_pubkey;
    console.log("Oracle config account: ", existed_config_pubkey.toBase58());
    const existed_data_data = get_oracle_data(existed_config_pubkey);
    console.log("Oracle data account: ", existed_data_data.account_pubkey.toBase58());
    const collection_mint_pubkey = get_pda(SBT_COLLECTION_SEED).account_pubkey;
    console.log("Collection mint account: ", collection_mint_pubkey.toBase58());
    const collection_metadata_pubkey = get_metadata(collection_mint_pubkey).account_pubkey;
    console.log("Collection metadata account: ", collection_metadata_pubkey.toBase58());
    const collection_master_edition_pubkey = get_master_edition(collection_mint_pubkey).account_pubkey;
    console.log("Collection master edition account: ", collection_master_edition_pubkey.toBase58());
    const event_config_pubkey = get_event_config(existed_config_pubkey).account_pubkey;
    console.log("Event config account: ", event_config_pubkey.toBase58());


    it("Should: The global config account is initialized!", async () => {
        const existed_config_account_info = await
            provider.connection.getAccountInfo(existed_global_config_pubkey);
        if (existed_config_account_info === null) {
            console.log("[ONCE] Create Global Config Account: ", existed_global_config_pubkey.toBase58())
            const tx = await program.methods.initializeGlobalConfig()
                .accounts({
                    globalConfig: existed_global_config_pubkey,
                    user: wallet.publicKey,
                    visionMiningPda: get_pda(VISION_MINING_SEED).account_pubkey,
                    eventMiningPda: get_pda(EVENT_MINING_SEED).account_pubkey,
                    stakeMiningPda: get_pda(STAKE_MINING_SEED).account_pubkey,
                    visionMiningAdminPubkey: wallet.publicKey,
                    governor: wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }).rpc();
            console.log("[ONCE] Create Global Config Account Signature: ", tx);
        } else {
            console.log("Global Config has been created")
        }
    });

    it("Should: Change vision mining admin", async () => {
        const new_admin = anchor.web3.Keypair.generate();
        console.log("Change vision mining admin to: ", new_admin.publicKey.toBase58())
        let tx = await program.methods.changeVisionMiningAdmin()
            .accounts({
                globalConfig: existed_global_config_pubkey,
                user: wallet.publicKey,
                visionMiningAdminPubkey: new_admin.publicKey
            }).rpc();
        console.log("Change vision mining admin signature: ", tx);
        const old_admin_pubkey =
            new anchor.web3.PublicKey("BH1VCRN52ZZeuedMkEukJDiZK58sjMzcRbYw7cADHQyg");
        //     new anchor.web3.PublicKey("9BCXkJbiftuJCf8mydw7znBr3HmsAydgikYkkPwcqbWG");
        console.log("Change vision mining admin back to: ", old_admin_pubkey)
        tx = await program.methods.changeVisionMiningAdmin()
            .accounts({
                globalConfig: existed_global_config_pubkey,
                user: wallet.publicKey,
                visionMiningAdminPubkey: old_admin_pubkey,
            }).rpc();
        console.log("Change vision mining admin back signature: ", tx);
    })

    it("Should: Initialize Mint Account and Metadata Account", async () => {
        const mint_pubkey = get_pda(MINT_SEED).account_pubkey;
        console.log("Mint account: ", mint_pubkey.toBase58())
        const mint_config_pubkey = get_pda(MINT_CONFIG_SEED).account_pubkey;
        console.log("Mint config account: ", mint_config_pubkey.toBase58());
        const metadata_pubkey = get_metadata(mint_pubkey).account_pubkey;
        console.log("Metadata account: ", metadata_pubkey.toBase58());
        const authority_pubkey = get_authority(mint_pubkey).account_pubkey;
        console.log("Authority account: ", authority_pubkey.toBase58())
        const existed_mint_config_account_info = await
            provider.connection.getAccountInfo(mint_config_pubkey);
        const params = {
            name: "Moni",
            symbol: "MONI",
            uri: "monitocol.xyz",
            decimals: 9
        };
        if (existed_mint_config_account_info === null) {
            console.log("[ONCE] Create Mint, Mint Config, Metadata Account.");
            let tx = await program.methods.initializeMint(params)
                .accounts({
                    payer: wallet.publicKey,
                    authority: authority_pubkey,
                    mint: mint_pubkey,
                    mintConfig: mint_config_pubkey,
                    globalConfig: existed_global_config_pubkey,
                    metadataAccount: metadata_pubkey,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }).rpc().catch(e => console.error(e));
            console.log("[ONCE] Create Mint, Mint Config, Metadata Account Signature: ", tx);
        } else {
            console.log("Mint Accounts has been initialized");
        }
    });

    it("Should: Mint Token", async () => {
        const mint_pubkey = get_pda(MINT_SEED).account_pubkey;
        console.log("Mint account: ", mint_pubkey.toBase58());
        const vision_mining_pda = get_pda(VISION_MINING_SEED).account_pubkey;
        console.log("Vision mining pda account: ", vision_mining_pda.toBase58());
        const vision_mining_token_account = await getAssociatedTokenAddress(
            mint_pubkey, vision_mining_pda, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("Vision mining token account: ", vision_mining_token_account.toBase58());
        const authority_pubkey = get_authority(mint_pubkey).account_pubkey;
        console.log("Authority account: ", authority_pubkey.toBase58())
        console.log("Mint tokens for: ", vision_mining_token_account.toBase58());
        const params = {
            amount:  new anchor.BN(10e12)
        }
        const subscriptionId = program.addEventListener("BalanceChangeEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });
        const tx = await program.methods.mintTokens(params)
            .accounts({
                payer: wallet.publicKey,
                globalConfig: existed_global_config_pubkey,
                mint: mint_pubkey,
                user: vision_mining_pda,
                tokenAccount: vision_mining_token_account,
                authority: authority_pubkey,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId
            }).rpc().catch(e => console.error(e));
        console.log("Mint tokens signature: ", tx);

        await program.removeEventListener(subscriptionId);
    });

    it("Should: Claim vision mining", async () => {

        // load vision mining admin keypair
        const vision_mining_admin_wallet_path = '../../../test-ledger/wallet.json';
        const vision_mining_admin_wallet_string = readFileSync(vision_mining_admin_wallet_path, 'utf8');
        const vision_mining_admin_keypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array(JSON.parse(vision_mining_admin_wallet_string)));

        const mint_pubkey = get_pda(MINT_SEED).account_pubkey;
        console.log("Mint account: ", mint_pubkey.toBase58());
        const vision_mining_pda = get_pda(VISION_MINING_SEED).account_pubkey;
        console.log("Vision mining pda account: ", vision_mining_pda.toBase58());
        const vision_mining_token_account = await getAssociatedTokenAddress(
            mint_pubkey, vision_mining_pda, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("Vision mining token account: ", vision_mining_token_account.toBase58());
        const user_pubkey = wallet.publicKey;
        console.log("User system account: ", user_pubkey.toBase58());
        const user_token_account = await getAssociatedTokenAddress(
            mint_pubkey, user_pubkey, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("User token account: ", user_token_account.toBase58());

        // Client need create token account for the user first
        let user_token_account_info = await
            provider.connection.getAccountInfo(user_token_account);
        if (user_token_account_info === null) {
            console.log("Create user token account first.")
            const instruction = createAssociatedTokenAccountInstruction(
                wallet.publicKey,
                user_token_account,
                user_pubkey,
                mint_pubkey,
                TOKEN_2022_PROGRAM_ID,
                ASSOCIATED_TOKEN_PROGRAM_ID,
            );
            let tx = new anchor.web3.Transaction;
            tx.add(instruction);
            const res = await program.provider.sendAndConfirm(
                tx,
                [wallet.payer]
            );
            console.log("Account ATA Res: ", res);
        }
        console.log("Claim vision mining: \n" +
            "Transfer from vision mining token account, ", vision_mining_token_account.toBase58(), +
            "to user token account ", user_token_account.toBase58());
        const interval = 10;
        let now = new Date();
        now.setMinutes(now.getMinutes() + interval);
        const unixTimestampInSeconds = Math.floor((now.getTime() / 1000));
        const params = {
            amount:  new anchor.BN(10e8),
            validUntilTime: new anchor.BN(unixTimestampInSeconds)
        }
        const subscriptionId = program.addEventListener("BalanceChangeEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });
        const tx = await program.methods.visionMiningClaim(params)
            .accounts({
                payer: wallet.publicKey,
                visionMiningAdmin: vision_mining_admin_keypair.publicKey,
                globalConfig: existed_global_config_pubkey,
                mint: mint_pubkey,
                visionMiningPda: vision_mining_pda,
                visionMiningTokenAccount: vision_mining_token_account,
                tokenAccount: user_token_account,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
            }).transaction();

        // Partial sign with admin keypair

        const { blockhash } = await provider.connection.getLatestBlockhash();
        tx.recentBlockhash = blockhash;
        tx.feePayer = wallet.publicKey;
        console.log("Vision mining admin pubkey: ", vision_mining_admin_keypair.publicKey.toBase58())
        tx.partialSign(vision_mining_admin_keypair);


        // Serialize transaction for client signing
        let serialized_transaction = tx.serialize({ requireAllSignatures: false });
        console.log("Serialized Transaction send from admin");

        // Deserialize transaction and sign with client wallet
        let transaction_to_sign = anchor.web3.Transaction.from(serialized_transaction);
        transaction_to_sign.partialSign(wallet.payer);

        // Re-serialize and send transaction
        // serialized_transaction = transaction_to_sign.serialize({ requireAllSignatures: false });
        const txId = await provider.sendAndConfirm(transaction_to_sign).catch(e => console.error(e));

        console.log("Transaction signature: ", txId);

        const latestBlockHash = await provider.connection.getLatestBlockhash();

        // @ts-ignore
        await provider.connection.confirmTransaction({
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: txId,},
            "confirmed" );

        await program.removeEventListener(subscriptionId);
    });

    it("Should: Initialize Collection Mint, Metadata and MasterEdition Account", async () => {
        const mint_pubkey = get_pda(SBT_COLLECTION_SEED).account_pubkey;
        console.log("Mint account: ", mint_pubkey.toBase58())
        const metadata_pubkey = get_metadata(mint_pubkey).account_pubkey;
        console.log("Metadata account: ", metadata_pubkey.toBase58());
        const master_edition_pubkey = get_master_edition(mint_pubkey).account_pubkey;
        console.log("Master edition account: ", master_edition_pubkey.toBase58());
        const authority_pubkey = get_authority(mint_pubkey).account_pubkey;
        console.log("Authority account: ", authority_pubkey.toBase58())
        const existed_mint_account_info = await
            provider.connection.getAccountInfo(mint_pubkey);
        const params = {
            name: "Moni",
            symbol: "MONI",
            uri: "monitocol.xyz",
            decimals: 9
        };
        if (existed_mint_account_info === null) {
            console.log("[ONCE] Create Collection Mint, Metadata and Master Edition Account.");
            let tx = await program.methods.initializeCollection(params)
                .accounts({
                    payer: wallet.publicKey,
                    globalConfig: existed_global_config_pubkey,
                    authority: authority_pubkey,
                    mint: mint_pubkey,
                    metadata: metadata_pubkey,
                    masterEdition: master_edition_pubkey,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }).rpc().catch(e => console.error(e));
            console.log("[ONCE] Create Collection Mint, Metadata and Master Edition Account Signature: ", tx);
        } else {
            console.log("Collection Accounts has been initialized");
        }
    });

    it("Should: Create Event Config", async () => {

        console.log("Preparing oracle config and oracle data...");
        const short_description = "This is a description.";
        const total_phase = 2;

        console.log("Create oracle config account: ", existed_config_pubkey.toBase58())
        let tx = await monitor_program.methods.initializeOracleConfig(
            existed_config_name,
            short_description,
            total_phase
        ).accounts({
            config: existed_config_pubkey,
            user: wallet.publicKey,
            authorityPubkey: wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
        console.log("Create oracle config signature", tx);

        console.log("Create oracle data account: ", existed_data_data.account_pubkey.toBase58())
        tx = await monitor_program.methods.initializeOracleData().accounts({
                config: existed_config_pubkey,
                oracle: existed_data_data.account_pubkey,
                user: wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }
        ).rpc();
        console.log("Create oracle data signature", tx);

        const phase = 1;
        const raw_data = new anchor.BN(83);
        const decimals = 0

        console.log("Set oracle data.")
        tx = await monitor_program.methods.setOracleData(
            phase,
            raw_data,
            decimals,
            existed_data_data.bump
        ).accounts({
            config: existed_config_pubkey,
            oracle: existed_data_data.account_pubkey,
            user: wallet.publicKey,
        }).rpc();
        console.log('Set oracle data signature: ', tx);

        console.log("Create chain_event market chain_event subscriber.")
        const subscriptionId = program.addEventListener("EventEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });

        console.log("Create event config now.")
        const governor = wallet.payer;
        console.log("Governor account: ", governor.publicKey.toBase58());
        const resolver_pubkey = wallet.publicKey;
        console.log("Resolver account: ", resolver_pubkey.toBase58());
        const metadata_json_url = "https://monitocol.xyz";
        let transaction = await program.methods.createEventConfig({
            eventType: {rawDataEventMarket:{}},
            option: 2,
            metadataJsonUrl: metadata_json_url,
        })
            .accounts({
                payer: wallet.publicKey,
                governor: governor.publicKey,
                resolver: resolver_pubkey,
                globalConfig: existed_global_config_pubkey,
                oracleConfig: existed_config_pubkey,
                eventConfig: event_config_pubkey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).transaction();

        await sign_with_governor(transaction, wallet.payer);

        await program.removeEventListener(subscriptionId);

        const mintLen = getMintLen([ExtensionType.NonTransferable]);
        console.log(mintLen)
        const mintLen2 = getMintLen([]);
        console.log(mintLen2)
    });

    it("Should: Create Event Market", async () => {

        console.log("Create chain_event market chain_event subscriber.")
        const subscriptionId = program.addEventListener("EventEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });

        const governor = wallet.payer;
        console.log("Governor account: ", governor.publicKey.toBase58());
        const resolver_pubkey = wallet.publicKey;
        console.log("Resolver account: ", resolver_pubkey.toBase58());
        const event_config_account = await program.account.eventConfig.fetch(event_config_pubkey);
        const index = event_config_account.index;
        console.log("Index: ", index);
        const event_market_pubkey = get_event_market(existed_config_pubkey, index).account_pubkey;
        console.log("Event data account: ", event_market_pubkey.toBase58());
        let now = new Date();
        now.setSeconds(now.getSeconds() + 6);
        const unixTimestampInSeconds = Math.floor((now.getTime() / 1000));
        let transaction = await program.methods.createEventMarket({
            closeTs: new anchor.BN(unixTimestampInSeconds),
            expiryTs: new anchor.BN(unixTimestampInSeconds)
        })
            .accounts({
                payer: wallet.publicKey,
                governor: governor.publicKey,
                globalConfig: existed_global_config_pubkey,
                eventConfig: event_config_pubkey,
                oracleConfig: existed_config_pubkey,
                oracleData: existed_data_data.account_pubkey,
                eventMarketAccount: event_market_pubkey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).transaction();

        await sign_with_governor(transaction, wallet.payer);

        await program.removeEventListener(subscriptionId);

        const mintLen = getMintLen([ExtensionType.NonTransferable]);
        console.log(mintLen)
        const mintLen2 = getMintLen([]);
        console.log(mintLen2)
    });

    it("Should: I. Create SBT", async () => {
        const mint_pubkey = get_sbt_mint(wallet.publicKey).account_pubkey;
        console.log("Mint account: ", mint_pubkey.toBase58())
        const metadata_pubkey = get_metadata(mint_pubkey).account_pubkey;
        console.log("Metadata account: ", metadata_pubkey.toBase58());
        const master_edition_pubkey = get_master_edition(mint_pubkey).account_pubkey;
        console.log("Master edition account: ", master_edition_pubkey.toBase58());
        const authority_pubkey = get_authority(collection_mint_pubkey).account_pubkey;
        console.log("Authority account: ", authority_pubkey.toBase58());
        const existed_mint_account_info = await
            provider.connection.getAccountInfo(mint_pubkey);
        const params = {
            name: "Monitor SBT",
            symbol: "SBT",
            uri: "monitocol.xyz"
        };
        if (existed_mint_account_info === null) {
            console.log("[ONCE] Create SBT");
            const subscriptionId = program.addEventListener("SBTMintEvent", (event, slot) => {
                console.log('Event data:', event);
                console.log('Slot:', slot);
            });
            let tx = await program.methods.createSbt(params)
                .accounts({
                    payer: wallet.publicKey,
                    authority: authority_pubkey,
                    collectionMint: collection_mint_pubkey,
                    mint: mint_pubkey,
                    metadata: metadata_pubkey,
                    masterEdition: master_edition_pubkey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                    sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }).rpc();

            console.log(" Create SBT transaction signature: ", tx);

            const latestBlockHash = await provider.connection.getLatestBlockhash();

            // @ts-ignore
            await provider.connection.confirmTransaction({
                    blockhash: latestBlockHash.blockhash,
                    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                    signature: tx,},
                "confirmed" );

            await program.removeEventListener(subscriptionId);
        } else {
            console.log("SBT has been created");
        }
    });

    it("Should: II. Mint SBT", async () => {
        const mint_pubkey = get_sbt_mint(wallet.publicKey).account_pubkey;
        console.log("Mint account: ", mint_pubkey.toBase58())
        const metadata_pubkey = get_metadata(mint_pubkey).account_pubkey;
        console.log("Metadata account: ", metadata_pubkey.toBase58());
        const master_edition_pubkey = get_master_edition(mint_pubkey).account_pubkey;
        console.log("Master edition account: ", master_edition_pubkey.toBase58());
        const authority_pubkey = get_authority(collection_mint_pubkey).account_pubkey;
        console.log("Authority account: ", authority_pubkey.toBase58());
        const token_account_pubkey = await getAssociatedTokenAddress(
            mint_pubkey, wallet.publicKey, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("Token account: ", token_account_pubkey.toBase58());
        const marker_pubkey = get_marker(wallet.publicKey, mint_pubkey).account_pubkey;
        console.log("Marker account: ", marker_pubkey.toBase58());
        const existed_token_account_info = await
            provider.connection.getAccountInfo(token_account_pubkey);
        if (existed_token_account_info === null) {
            console.log("[ONCE] Mint SBT");
            const subscriptionId = program.addEventListener("SBTMintEvent", (event, slot) => {
                console.log('Event data:', event);
                console.log('Slot:', slot);
            });
            let tx = await program.methods.mintSbt()
                .accounts({
                    payer: wallet.publicKey,
                    authority: authority_pubkey,
                    collectionMint: collection_mint_pubkey,
                    collectionMetadata: collection_metadata_pubkey,
                    collectionMasterEdition: collection_master_edition_pubkey,
                    mint: mint_pubkey,
                    tokenAccount: token_account_pubkey,
                    marker: marker_pubkey,
                    metadata: metadata_pubkey,
                    masterEdition: master_edition_pubkey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                    sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                }).rpc().catch(e => console.error(e));

            console.log(" Mint SBT transaction signature: ", tx);

            const latestBlockHash = await provider.connection.getLatestBlockhash();

            // @ts-ignore
            await provider.connection.confirmTransaction({
                    blockhash: latestBlockHash.blockhash,
                    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                    signature: tx,},
                "confirmed" );

            await program.removeEventListener(subscriptionId);
        } else {
            console.log("SBT has been minted");
        }
    });

    it("Should: I. Create Event Master Edition SBT", async () => {
        let option = 1;
        const mint_pubkey = get_sbt_event_mint(event_config_pubkey, option).account_pubkey;
        console.log("Mint account: ", mint_pubkey.toBase58())
        const metadata_pubkey = get_metadata(mint_pubkey).account_pubkey;
        console.log("Metadata account: ", metadata_pubkey.toBase58());
        const master_edition_pubkey = get_master_edition(mint_pubkey).account_pubkey;
        console.log("Master edition account: ", master_edition_pubkey.toBase58());
        const authority_pubkey = get_authority(collection_mint_pubkey).account_pubkey;
        console.log("Authority account: ", authority_pubkey.toBase58());
        const existed_mint_account_info = await
            provider.connection.getAccountInfo(mint_pubkey);
        const params = {
            name: "Monitor SBT",
            symbol: "SBT",
            uri: "monitocol.xyz",
            option: 1
        };
        if (existed_mint_account_info === null) {
            console.log("[ONCE] Create Event Master Edition SBT for option", option);
            const subscriptionId = program.addEventListener("SBTMintEvent", (event, slot) => {
                console.log('Event data:', event);
                console.log('Slot:', slot);
            });
            let transaction = await program.methods.createEventSbt(params)
                .accounts({
                    payer: wallet.publicKey,
                    governor: wallet.publicKey,
                    globalConfig: existed_global_config_pubkey,
                    eventConfig: event_config_pubkey,
                    authority: authority_pubkey,
                    collectionMint: collection_mint_pubkey,
                    mint: mint_pubkey,
                    metadata: metadata_pubkey,
                    masterEdition: master_edition_pubkey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                    sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }).transaction();

            await sign_with_governor(transaction, wallet.payer);

            await program.removeEventListener(subscriptionId);
        } else {
            console.log("SBT has been created");
        }
    });

    it("Should: II. Mint Event Master Edition SBT", async () => {
        let option = 1;
        const mint_pubkey = get_sbt_event_mint(event_config_pubkey, option).account_pubkey;
        console.log("Mint account: ", mint_pubkey.toBase58())
        const metadata_pubkey = get_metadata(mint_pubkey).account_pubkey;
        console.log("Metadata account: ", metadata_pubkey.toBase58());
        const master_edition_pubkey = get_master_edition(mint_pubkey).account_pubkey;
        console.log("Master edition account: ", master_edition_pubkey.toBase58());
        const authority_pubkey = get_authority(collection_mint_pubkey).account_pubkey;
        console.log("Authority account: ", authority_pubkey.toBase58());
        const token_account_pubkey = await getAssociatedTokenAddress(
            mint_pubkey, authority_pubkey, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("Token account: ", token_account_pubkey.toBase58());
        const existed_token_account_info = await
            provider.connection.getAccountInfo(token_account_pubkey);
        if (existed_token_account_info === null) {
            console.log("[ONCE] Mint Event Master Edition SBT");
            const subscriptionId = program.addEventListener("SBTMintEvent", (event, slot) => {
                console.log('Event data:', event);
                console.log('Slot:', slot);
            });
            let params = {
                option: option
            }
            let transaction = await program.methods.mintEventSbtMasterEdition(params)
                .accounts({
                    payer: wallet.publicKey,
                    governor: wallet.publicKey,
                    globalConfig: existed_global_config_pubkey,
                    eventConfig: event_config_pubkey,
                    authority: authority_pubkey,
                    collectionMint: collection_mint_pubkey,
                    mint: mint_pubkey,
                    tokenAccount: token_account_pubkey,
                    metadata: metadata_pubkey,
                    masterEdition: master_edition_pubkey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                    sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                }).transaction();

            await sign_with_governor(transaction, wallet.payer);

            await program.removeEventListener(subscriptionId);
        } else {
            console.log("SBT has been minted");
        }
    });

    it("Should: toggle event market", async () => {
        const index = new anchor.BN(0);
        const event_market_pubkey = get_event_market(existed_config_pubkey, index).account_pubkey;
        console.log("Event market pubkey: ", event_market_pubkey.toBase58());
        const subscriptionId = program.addEventListener("EventEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });
        let transaction = await program.methods.toggleEventMarket()
            .accounts({
                payer: wallet.publicKey,
                governor: wallet.publicKey,
                globalConfig: existed_global_config_pubkey,
                eventConfig: event_config_pubkey,
                eventMarketAccount: event_market_pubkey
            }).transaction();

        await sign_with_governor(transaction, wallet.payer);

        await program.removeEventListener(subscriptionId);

    });

    it("Should: choose", async () => {
        const index = new anchor.BN(0);
        const indicate = 1;
        const event_market_pubkey = get_event_market(existed_config_pubkey, index).account_pubkey;
        console.log("Event market pubkey: ", event_market_pubkey.toBase58());
        const sbt_mint = get_sbt_mint(wallet.publicKey).account_pubkey;
        console.log("SBT mint pubkey: ", sbt_mint.toBase58());
        const token_account_pubkey = await getAssociatedTokenAddress(
            sbt_mint, wallet.publicKey, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("SBT token account pubkey: ", token_account_pubkey.toBase58());
        const marker = get_marker(wallet.publicKey, sbt_mint).account_pubkey;
        console.log("Marker pubkey: ", marker.toBase58());
        const user_position = get_user_position(event_market_pubkey, wallet.publicKey).account_pubkey;
        console.log("User position pubkey: ", user_position.toBase58());
        const event_position = get_event_position(event_market_pubkey, indicate).account_pubkey;
        console.log("Event position pubkey: ", event_position.toBase58());

        const subscriptionId = program.addEventListener("ChooseEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });
        const params = {
            indicate: indicate
        }
        let tx = await program.methods.choose(params)
            .accounts({
                payer: wallet.publicKey,
                sbtMint: sbt_mint,
                tokenAccount: token_account_pubkey,
                marker: marker,
                userPosition: user_position,
                eventPosition: event_position,
                eventConfig: event_config_pubkey,
                eventMarket: event_market_pubkey,
                systemProgram: anchor.web3.SystemProgram.programId
            }).rpc();

        console.log("Signature: ", tx);
        await program.removeEventListener(subscriptionId);
    });

    it("Should: withdraw", async () => {
        const index = new anchor.BN(0);
        const indicate = 1;
        const event_market_pubkey = get_event_market(existed_config_pubkey, index).account_pubkey;
        console.log("Event market pubkey: ", event_market_pubkey.toBase58());
        const sbt_mint = get_sbt_mint(wallet.publicKey).account_pubkey;
        console.log("SBT mint pubkey: ", sbt_mint.toBase58());
        const token_account_pubkey = await getAssociatedTokenAddress(
            sbt_mint, wallet.publicKey, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("SBT token account pubkey: ", token_account_pubkey.toBase58());
        const marker = get_marker(wallet.publicKey, sbt_mint).account_pubkey;
        console.log("Marker pubkey: ", marker.toBase58());
        const user_position = get_user_position(event_market_pubkey, wallet.publicKey).account_pubkey;
        console.log("User position pubkey: ", user_position.toBase58());
        const event_position = get_event_position(event_market_pubkey, indicate).account_pubkey;
        console.log("Event position pubkey: ", event_position.toBase58());
        const subscriptionId = program.addEventListener("ChooseEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });
        const params = {
            indicate: indicate
        }
        let tx = await program.methods.withdraw(params)
            .accounts({
                payer: wallet.publicKey,
                sbtMint: sbt_mint,
                marker: marker,
                userPosition: user_position,
                eventPosition: event_position,
                eventMarket: event_market_pubkey,
                systemProgram: anchor.web3.SystemProgram.programId
            }).rpc();
        console.log("Withdraw signature: ", tx);

        tx = await program.methods.choose(params)
            .accounts({
                payer: wallet.publicKey,
                sbtMint: sbt_mint,
                tokenAccount: token_account_pubkey,
                marker: marker,
                userPosition: user_position,
                eventPosition: event_position,
                eventConfig: event_config_pubkey,
                eventMarket: event_market_pubkey,
                systemProgram: anchor.web3.SystemProgram.programId
            }).rpc();

        console.log("Re-choose signature: ", tx);
        await program.removeEventListener(subscriptionId);
    });

    it("Should: resolve", async () => {
        console.log("Set oracle new data.");
        const phase = 1;
        const raw_data = new anchor.BN(80);
        const decimals = 0

        let tx = await monitor_program.methods.setOracleData(
            phase,
            raw_data,
            decimals,
            existed_data_data.bump
        ).accounts({
            config: existed_config_pubkey,
            oracle: existed_data_data.account_pubkey,
            user: wallet.publicKey,
        }).rpc();
        console.log('Set oracle data signature: ', tx);
        await sleep(5000);
        console.log('Resolve event market.')
        const prize = new anchor.BN(30000000000);
        const index = new anchor.BN(0);
        const event_market_pubkey = get_event_market(existed_config_pubkey, index).account_pubkey;
        console.log("Event market pubkey: ", event_market_pubkey.toBase58());
        const subscriptionId = program.addEventListener("EventEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });
        const params = {
            prize: prize
        }
        tx = await program.methods.resolve(params)
            .accounts({
                resolver: wallet.publicKey,
                oracleConfig: existed_config_pubkey,
                oracleData: existed_data_data.account_pubkey,
                eventConfig: event_config_pubkey,
                eventMarket: event_market_pubkey
            }).rpc();
        console.log("Resolve signature: ", tx);
        await program.removeEventListener(subscriptionId);
    });

    it("Should: claim", async () => {
        console.log("Mint token to event mining token account");
        const mint = get_pda(MINT_SEED).account_pubkey;
        console.log("FT Mint pubkey: ", mint.toBase58());
        const event_mining_pda = get_pda(EVENT_MINING_SEED).account_pubkey;
        console.log("Event mining pda pubkey: ", event_mining_pda.toBase58());
        const event_mining_token_account = await getAssociatedTokenAddress(
            mint, event_mining_pda, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("Event mining token account pubkey: ", event_mining_token_account.toBase58());
        const ft_mint_authority = get_authority(mint).account_pubkey;
        console.log("FT mint authority pubkey: ", ft_mint_authority.toBase58());
        let params = {
            amount:  new anchor.BN(10e13)
        }
        let tx = await program.methods.mintTokens(params).accounts({
            payer: wallet.publicKey,
            globalConfig: existed_global_config_pubkey,
            mint: mint,
            user: event_mining_pda,
            tokenAccount: event_mining_token_account,
            authority: ft_mint_authority,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId
            }).rpc();

        console.log("Mint token to event mining token account signature: ", tx);

        console.log("User claim prize")
        const index = new anchor.BN(0);
        const event_market_pubkey = get_event_market(existed_config_pubkey, index).account_pubkey;
        console.log("Event market pubkey: ", event_market_pubkey.toBase58())
        const sbt_mint = get_sbt_mint(wallet.publicKey).account_pubkey;
        console.log("SBT mint pubkey: ", sbt_mint.toBase58());
        const marker = get_marker(wallet.publicKey, sbt_mint).account_pubkey;
        console.log("Marker pubkey: ", marker.toBase58());
        const user_position = get_user_position(event_market_pubkey, wallet.publicKey).account_pubkey;
        console.log("User position pubkey: ", user_position.toBase58());
        const token_account = await getAssociatedTokenAddress(
            mint, wallet.publicKey, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("User FT token account pubkey: ", token_account.toBase58());
        let indicate = 1;
        const event_sbt_edition_mint = get_sbt_event_edition_mint(
            event_config_pubkey, indicate, wallet.publicKey).account_pubkey;
        console.log("Event sbt edition mint pubkey: ", event_sbt_edition_mint.toBase58());
        const event_sbt_edition_token_account = await getAssociatedTokenAddress(
            event_sbt_edition_mint, wallet.publicKey, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("Event sbt edition token account pubkey: ", event_sbt_edition_token_account.toBase58());
        const event_sbt_edition_metadata = get_metadata(event_sbt_edition_mint).account_pubkey;
        console.log("Event sbt edition metadata pubkey: ", event_sbt_edition_metadata.toBase58());
        const event_sbt_edition = get_master_edition(event_sbt_edition_mint).account_pubkey;
        console.log("Event sbt edition pubkey: ", event_sbt_edition.toBase58());
        const authority = get_authority(collection_mint_pubkey).account_pubkey;
        console.log("Print authority pubkey: ", authority.toBase58())
        const event_sbt_master_edition_mint = get_sbt_event_mint(event_config_pubkey, indicate).account_pubkey;
        console.log("Event sbt master edition mint: ", event_sbt_master_edition_mint.toBase58());
        const event_sbt_master_edition_token_account = await getAssociatedTokenAddress(
            event_sbt_master_edition_mint, authority, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("Event sbt master edition token account: ", event_sbt_master_edition_token_account.toBase58());
        const event_sbt_master_edition_metadata = get_metadata(event_sbt_master_edition_mint).account_pubkey;
        console.log("Event sbt master edition metadata: ", event_sbt_master_edition_metadata.toBase58());
        const event_sbt_master_edition = get_master_edition(event_sbt_master_edition_mint).account_pubkey;
        console.log("Event sbt master edition master edition: ", event_sbt_master_edition.toBase58());
        const event_sbt_edition_pda = get_edition_mark_pda(event_sbt_master_edition_mint, 1).account_pubkey;
        console.log("Event sbt edition pda: ", event_sbt_edition_pda.toBase58());
        const subscriptionId = program.addEventListener("SBTMintEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });
        const subscriptionId2 = program.addEventListener("BalanceChangeEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });
        const subscriptionId3 = program.addEventListener("ChooseEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });

        let params2 = {
            indicate: indicate,
            sbtMint: sbt_mint,
        }
        const modifyComputeUnits = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
            units: 1000000
        });

        const addPriorityFee = anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
            microLamports: 10
        });

        let ix = await program.methods.claim(params2)
            .accounts({
                payer: wallet.publicKey,
                globalConfig: existed_global_config_pubkey,
                eventConfig: event_config_pubkey,
                eventMarket: event_market_pubkey,
                marker: marker,
                userPosition: user_position,
                mint: mint,
                tokenAccount: token_account,
                eventMiningPda: event_mining_pda,
                eventMiningTokenAccount: event_mining_token_account,
                eventSbtEditionMint: event_sbt_edition_mint,
                eventSbtEditionTokenAccount: event_sbt_edition_token_account,
                eventSbtEditionMetadata: event_sbt_edition_metadata,
                eventSbtEdition: event_sbt_edition,
                eventSbtEditionPda: event_sbt_edition_pda,
                authority: authority,
                collectionMint: collection_mint_pubkey,
                eventSbtMasterEditionMint: event_sbt_master_edition_mint,
                eventSbtMasterEditionTokenAccount: event_sbt_master_edition_token_account,
                eventSbtMasterEditionMetadata: event_sbt_master_edition_metadata,
                eventSbtMasterEdition: event_sbt_master_edition,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,

            }).instruction();

        let transaction = new anchor.web3.Transaction()
            .add(modifyComputeUnits)
            .add(addPriorityFee)
            .add(ix);

        const txId = await provider.sendAndConfirm(transaction).catch(e => console.error(e));
        console.log("Resolve signature: ", txId);
        await program.removeEventListener(subscriptionId);
        await program.removeEventListener(subscriptionId2);
        await program.removeEventListener(subscriptionId3);
    });
});

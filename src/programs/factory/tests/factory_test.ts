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
    ExtensionType
} from "@solana/spl-token";


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

    const get_event_market = (config_pubkey: anchor.web3.PublicKey): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("event_market"),
                config_pubkey.toBuffer(),
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

    const get_marker = (
        user_pubkey: anchor.web3.PublicKey,
        event_market_pubkey: anchor.web3.PublicKey | null): { account_pubkey: anchor.web3.PublicKey, bump: number } => {
        if (event_market_pubkey === null) {
            const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("marker"),
                    user_pubkey.toBuffer(),
                ],
                program.programId
            );
            return {account_pubkey, bump}
        } else {
            const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("marker"),
                    user_pubkey.toBuffer(),
                    event_market_pubkey.toBuffer(),
                ],
                program.programId
            );
            return {account_pubkey, bump}
        }
    }

    const existed_global_config_pubkey = get_pda(GLOBAL_CONFIG_SEED).account_pubkey;

    function getMintAuthority(accountInfo: anchor.web3.AccountInfo<Buffer>): anchor.web3.PublicKey | null {
        if (!accountInfo.data) {
            throw new Error('Account data is missing');
        }

        // 确保缓冲区足够大，可以包含 mintAuthorityOption 和 mintAuthority
        if (accountInfo.data.length < 1 + 32) {
            throw new Error('Invalid mint account data');
        }

        // 解析 mintAuthorityOption
        const mintAuthorityOption = accountInfo.data[0];

        // 如果 mintAuthorityOption 是 1，代表存在 mintAuthority
        if (mintAuthorityOption === 1) {
            return new anchor.web3.PublicKey(accountInfo.data.slice(1, 33));
        }

        // 如果 mintAuthorityOption 不是 1，则没有 mintAuthority
        return null;
    }


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

    it("Should: Create Event Market", async () => {

        console.log("Preparing oracle config and oracle data...");
        const existed_config_name = "22QVCPu62x2Z3abSdgHzpU4PyFttu9u";
        const existed_config_pubkey = get_oracle_config(existed_config_name).account_pubkey;
        const existed_data_data = get_oracle_data(existed_config_pubkey);
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
        const subscriptionId = program.addEventListener("EventMarketEvent", (event, slot) => {
            console.log('Event data:', event);
            console.log('Slot:', slot);
        });

        console.log("Create chain_event market now.")
        const governor = wallet.payer;
        console.log("Governor account: ", governor.publicKey.toBase58());
        const resolver = wallet.publicKey;
        console.log("Resolver account: ", governor.publicKey.toBase58());
        const event_market_pubkey = get_event_market(existed_config_pubkey).account_pubkey;
        let now = new Date();
        now.setMinutes(now.getSeconds() + 120);
        const unixTimestampInSeconds = Math.floor((now.getTime() / 1000));
        const metadata_json_url = "https://monitocol.xyz";
        let transaction = await program.methods.createEventMarket({
            eventType: {rawDataEventMarket:{}},
            option: 2,
            closeTs: new anchor.BN(unixTimestampInSeconds),
            expiryTs: new anchor.BN(unixTimestampInSeconds),
            metadataJsonUrl: metadata_json_url,
        })
            .accounts({
                payer: wallet.publicKey,
                governor: governor.publicKey,
                resolver: resolver,
                globalConfig: existed_global_config_pubkey,
                oracleConfig: existed_config_pubkey,
                oracleData: existed_data_data.account_pubkey,
                mint: null,
                eventMarketAccount: event_market_pubkey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).transaction();

        const { blockhash } = await provider.connection.getLatestBlockhash();
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = wallet.publicKey;
        transaction.partialSign(wallet.payer);
        transaction.partialSign(wallet.payer);

        const txId = await provider.sendAndConfirm(transaction).catch(e => console.error(e));
        console.log("Transaction signature: ", txId);

        const latestBlockHash = await provider.connection.getLatestBlockhash();

        // @ts-ignore
        await provider.connection.confirmTransaction({
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: txId,},
            "confirmed" );

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
        const collection_mint_pubkey = get_pda(SBT_COLLECTION_SEED).account_pubkey;
        console.log("Collection mint account: ", collection_mint_pubkey.toBase58());
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
                    // tokenAccount: token_account_pubkey,
                    metadata: metadata_pubkey,
                    masterEdition: master_edition_pubkey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                    sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }).rpc().catch(e => console.error(e));

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
        const mint_ai = await provider.connection.getAccountInfo(mint_pubkey);
        const mint_authority_from_ai = getMintAuthority(mint_ai);
        console.log("Mint authority read from account info: ", mint_authority_from_ai);
        const metadata_pubkey = get_metadata(mint_pubkey).account_pubkey;
        console.log("Metadata account: ", metadata_pubkey.toBase58());
        const master_edition_pubkey = get_master_edition(mint_pubkey).account_pubkey;
        console.log("Master edition account: ", master_edition_pubkey.toBase58());
        const collection_mint_pubkey = get_pda(SBT_COLLECTION_SEED).account_pubkey;
        console.log("Collection mint account: ", collection_mint_pubkey.toBase58());
        const collection_metadata_pubkey = get_metadata(collection_mint_pubkey).account_pubkey;
        console.log("Collection metadata account: ", metadata_pubkey.toBase58());
        const collection_master_edition_pubkey = get_master_edition(collection_mint_pubkey).account_pubkey;
        console.log("Collection master edition account: ", collection_master_edition_pubkey.toBase58());
        const authority_pubkey = get_authority(collection_mint_pubkey).account_pubkey;
        console.log("Authority account: ", authority_pubkey.toBase58());
        const token_account_pubkey = await getAssociatedTokenAddress(
            mint_pubkey, wallet.publicKey, true,
            TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
        console.log("Token account: ", token_account_pubkey.toBase58());
        const marker_pubkey = get_marker(wallet.publicKey, null).account_pubkey;
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
});

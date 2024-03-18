import * as anchor from "@coral-xyz/anchor";
import {Program, Wallet} from "@coral-xyz/anchor";
import {Factory} from "../target/types/factory";
import {} from '@metaplex-foundation/mpl-token-metadata';
import {ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID,
    getAssociatedTokenAddress, createAssociatedTokenAccountInstruction} from "@solana/spl-token";
import {assert} from "chai";


describe("factory", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(anchor.AnchorProvider.env());
    const wallet = provider.wallet as Wallet;
    const program = anchor.workspace.Factory as Program<Factory>;

    const GLOBAL_CONFIG_SEED = "global_config";
    const VISION_MINING_SEED = "ft_vision_mining_pda";
    const EVENT_MINING_SEED = "ft_event_mining_pda";
    const STAKE_MINING_SEED = "ft_stake_mining_pda";
    const MINT_SEED = "mint";
    const MINT_CONFIG_SEED = "mint_config";
    const AUTHORITY_SEED = "authority";
    const MPL_TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    const get_random_seed = (): string => {
        return anchor.web3.Keypair.generate().publicKey.toBase58().slice(0, 31);
    }

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

    const existed_global_config_pubkey = get_pda(GLOBAL_CONFIG_SEED).account_pubkey;

    it("Should: The global config account is initialized!", async () => {
        // Add your test here.
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
        const old_admin_pubkey = wallet.publicKey;
        console.log("Change vision mining admin back to: ", old_admin_pubkey)
        tx = await program.methods.changeVisionMiningAdmin()
            .accounts({
                globalConfig: existed_global_config_pubkey,
                user: wallet.publicKey,
                visionMiningAdminPubkey: wallet.publicKey,
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
            name: "Entropy",
            symbol: "ETP",
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
                globalConfig: existed_global_config_pubkey,
                mint: mint_pubkey,
                visionMiningPda: vision_mining_pda,
                visionMiningTokenAccount: vision_mining_token_account,
                tokenAccount: user_token_account,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
            }).rpc().catch(e => console.error(e));
        console.log("Signature: ", tx);

        await program.removeEventListener(subscriptionId);
    });
});

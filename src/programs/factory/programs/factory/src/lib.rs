mod error;
mod instructions;
mod state;
mod event;
mod utils;

use anchor_lang::prelude::*;
use error::ErrorCode;
use instructions::*;


declare_id!("EHQpL4Q8hMEsufdDR5bsSeSahqt8ShkHbxzPyGsurgR5");

pub const FT_MAX_SUPPLY: u64 = 1_000_000_000 * 1_000_000_000;

fn check_context<T: anchor_lang::Bumps>(ctx: &Context<T>) -> Result<()> {
    if !check_id(ctx.program_id) {
        return err!(ErrorCode::InvalidProgramId);
    }
    // make sure there are no extra accounts
    if !ctx.remaining_accounts.is_empty() {
        return err!(ErrorCode::UnexpectedAccount);
    }
    Ok(())
}


#[program]
pub mod factory {
    use super::*;

    // admin instructions

    pub fn initialize_global_config(
        ctx: Context<InitializeGlobalConfig>,
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts
            .process()?;
        Ok(())
    }

    pub fn change_vision_mining_admin(
        ctx: Context<ChangeVisionMiningAdmin>
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process()
    }

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        params: InitializeMintParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        params: MintTokensParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    pub fn initialize_collection(
        ctx: Context<InitializeCollection>,
        params: InitializeCollectionParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    // privilege instructions:

    pub fn vision_mining_claim(
        ctx: Context<VisionMiningClaim>,
        params: VisionMiningClaimParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    pub fn create_event_market(
        ctx: Context<CreateEventMarket>,
        params: CreateEventMarketParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    pub fn mint_tag_sbt(
        ctx: Context<MintTagSBT>,
    ) -> Result<()> {
        check_context(&ctx)?;
        Ok(())
    }

    // Initialize EventMarket config (pda of oracle config)
    // contains: Oracle Config
    // Market Type
    // Orientation

    // Initialize Market

    // Initialize SBT



    // Position Account

    // Orient( Deposit for SBT)


    // Initialize monitor NFT

    // Initialize Event SBT
    // EventSBTConfig = event_config + OracleConfigPubkey



    // initial Master Edition: master editionMint master edition
    // MasterEditionMint "event_sbt_me" + EventSBTConfigPubkey + Phase

    // mint_edition_sbt: edition mint, edition token account, edition metadata
    // EditionMint = "event_sbt_e" + MasterEditionMintPubkey + UserPubkey

}


mod error;
pub mod instructions;
pub mod state;
mod chain_event;
mod utils;

use anchor_lang::prelude::*;
use error::ErrorCode;
use instructions::*;
use instructions::resolvers::*;


declare_id!("36KZHRWMKbGNsMZ2jMVuRbnMUrjzRr8kmjHyPJ9ipvFW");

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

    pub fn create_event_config(
        ctx: Context<CreateEventConfig>,
        params: CreateEventConfigParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params, ctx.bumps.event_config)
    }

    pub fn create_event_market(
        ctx: Context<CreateEventMarket>,
        params: CreateEventMarketParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params, ctx.bumps.event_market_account)
    }


    pub fn toggle_event_market(
        ctx: Context<ToggleEventMarket>,
        params: ToggleEventMarketParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }


    pub fn create_event_sbt(
        ctx: Context<CreateEventSBT>,
        params: CreateEventSBTParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params, ctx.bumps.mint, ctx.bumps.authority)
    }

    pub fn mint_event_sbt_master_edition(
        ctx: Context<MintEventSBTMasterEdition>,
        params: MintEventSBTMasterEditionParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    pub fn resolve(
        ctx: Context<FearAndGreedEventMarketResolve>,
        params: FearAndGreedEventMarketResolveParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    // public instructions:

    pub fn create_sbt(
        ctx: Context<CreateSBT>,
        params: CreateSBTParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params, ctx.bumps.mint)
    }

    pub fn mint_sbt(
        ctx: Context<MintSBT>
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process()
    }

    pub fn choose(
        ctx: Context<Choose>,
        params: ChooseParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        params: WithdrawParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

    pub fn claim(
        ctx: Context<Claim>,
        params: ClaimParams
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
    }

}


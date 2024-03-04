mod state;
mod error;
mod instructions;
mod event;

use anchor_lang::prelude::*;
use state::{ OracleData, OracleConfig};
use instructions::*;
use error::ErrorCode;


declare_id!("9vDvoPvmq68icnHWXowjqoEKgSs8TvBmmFvTcFztephV");


fn check_context<T>(ctx: &Context<T>) -> Result<()> {
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
pub mod monitor {
    use anchor_lang::solana_program::short_vec::deserialize;
    use super::*;

    pub fn initialize_oracle_config(
        ctx: Context<InitializeOracleConfig>,
        name: String,
        description: String,
        total_phase: u8,
    ) -> Result<()> {
        check_context(&ctx)?;
        // let bump = ctx.bumps.config;
        // msg!(bump);
        ctx.accounts.process(name, description, total_phase, *ctx.bumps.get("config").unwrap())?;
        Ok(())
    }

    pub fn add_authority_to_oracle_config(
        ctx: Context<AddAuthorityToOracleConfig>
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process()
    }

    pub fn remove_authority_from_oracle_config(
        ctx: Context<RemoveAuthorityFromOracleConfig>
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process()
    }

    pub fn initialize_oracle_data(ctx: Context<InitializeOracleConfig>) -> Result<()>{
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeOracleData<'info> {
    // space: 8 discriminator + 32 config + 1 phase + 8 raw_data + 1 decimals + 8 timestamp + 1 bump
    pub config: Account<'info, OracleConfig>,
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 1 + 8 + 1 + 8 + 1)]
    pub oracle: Account<'info, OracleData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}




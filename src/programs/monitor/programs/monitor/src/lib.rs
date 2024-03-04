mod state;
mod error;

use anchor_lang::prelude::*;
use state::{ OracleData, OracleConfig};
use error::ErrorCode;

declare_id!("9vDvoPvmq68icnHWXowjqoEKgSs8TvBmmFvTcFztephV");

#[program]
pub mod monitor {
    use super::*;

    pub fn initialize_oracle_config(
        ctx: Context<InitializeOracleConfig>,
        name: String,
        description: String,
        total_phase: u8,
    ) -> Result<()> {
        require!(name.as_bytes().len() < 50, ErrorCode::StringTooLong);
        require!(description.as_bytes().len() < 200, ErrorCode::StringTooLong);
        require!(ctx.accounts.authority_pubkeys.len() <= 4, ErrorCode::InvalidArgument);
        let config_account = &mut ctx.accounts.config;
        config_account.name = name;
        config_account.description = description;
        config_account.authority_pubkeys = ctx.accounts.authority_pubkeys.clone();
        config_account.admin = ctx.accounts.user.key();
        config_account.bump = ctx.bumps.config;
        Ok(())
    }

    pub fn initialize_oracle_data(ctx: Context<InitializeOracleConfig>) -> Result<()>{
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeOracleConfig<'info> {
    // space: 8 discriminator + 4 name length + 50 name + 4 description length + 200 description
    //        + 1 total_phases + 4 authority_pubkeys + 4 * 32 authority_pubkeys + 32 admin pubkey + 1 bump
    #[account(
        init,
        payer = user,
        space = 8 + 4 + 50 + 4 + 200 + 1 + 4 + 128 + 32 + 1,
        seeds = [b"oracle-config", name.as_bytes(), user.key().as_ref()],
        bump)]
    pub config: Account<'info, OracleConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub authority_pubkeys: Vec<Pubkey>,
    pub system_program: Program<'info, System>,
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




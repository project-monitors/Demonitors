mod state;
mod error;
mod instructions;
mod event;

use anchor_lang::prelude::*;
use instructions::*;
use error::ErrorCode;


declare_id!("DQtL5gnrsA1e6vXrFSCTU87DHj6MBmHoZoL3bsh4uFPz");


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
pub mod monitor {
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
        ctx.accounts.process(name, description, total_phase, ctx.bumps.config)?;
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

    pub fn initialize_oracle_data(
        ctx: Context<InitializeOracleData>
    ) -> Result<()>{
        check_context(&ctx)?;
        ctx.accounts.process(ctx.bumps.oracle)?;
        Ok(())
    }

    pub fn set_oracle_data(
        ctx: Context<SetOracleData>,
        phase: u8,
        raw_data: u64,
        decimals: u8,
        bump: u8
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(phase, raw_data, decimals, bump)
    }
}





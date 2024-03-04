use anchor_lang::prelude::*;
use crate::state::OracleConfig;
use crate::error::ErrorCode;


//TODO: Fix Account vec
#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeOracleConfig<'info> {
    // space: 8 discriminator + 4 name length + 50 name + 4 description length + 200 description
    //        + 1 total_phases + 4 authority_pubkeys + 4 * 32 authority_pubkeys + 32 admin pubkey + 1 bump
    #[account(
    init,
    payer = user,
    space = 8 + 4 + 50 + 4 + 200 + 1 + 4 + 128 + 32 + 1,
    seeds = [b"oracle-config", name.as_bytes()],
    bump)]
    pub config: Account<'info, OracleConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub authority_pubkeys: Vec<Pubkey>,
    pub system_program: Program<'info, System>,
}

impl<'info>  InitializeOracleConfig<'info>  {

    pub fn process(&mut self, name: String, description: String, total_phase: u8, bump: u8) -> Result<()> {
        require!(name.as_bytes().len() < 50, ErrorCode::StringTooLong);
        require!(description.as_bytes().len() < 200, ErrorCode::StringTooLong);
        require!(self.authority_pubkeys.len() <= 4, ErrorCode::InvalidArgument);
        let config_account = &mut self.config;
        config_account.name = name;
        config_account.description = description;
        config_account.authority_pubkeys = self.authority_pubkeys.clone();
        config_account.admin = self.user.key();
        config_account.bump = bump;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddAuthorityToOracleConfig<'info> {
    #[account(
    mut,
    seeds = [b"oracle-config", config.name.as_bytes()],
    bump = config.bump)]
    pub config: Account<'info, OracleConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: System Account For Authority, won't read or write
    pub authority_pubkey: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddAuthorityToOracleConfig<'info> {

    pub fn process(&mut self) -> Result<()> {
        let config_account = &mut self.config;
        config_account.add_authority(self.user.key(), self.authority_pubkey.key())
    }
}

#[derive(Accounts)]
pub struct RemoveAuthorityFromOracleConfig<'info> {
    // space: 8 discriminator + 4 name length + 50 name + 4 description length + 200 description
    //        + 1 total_phases + 4 authority_pubkeys + 4 * 32 authority_pubkeys + 32 admin pubkey + 1 bump
    #[account(
    mut,
    seeds = [b"oracle-config", config.name.as_bytes()],
    bump = config.bump)]
    pub config: Account<'info, OracleConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: System Account For Authority, won't read or write
    pub authority_pubkey: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveAuthorityFromOracleConfig<'info> {

    pub fn process(&mut self) -> Result<()> {
        let config_account = &mut self.config;
        config_account.remove_authority(self.user.key(), self.authority_pubkey.key())
    }
}

use crate::state::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;



#[derive(Accounts)]
pub struct ChangeVisionMiningAdmin<'info> {
    #[account(
    mut,
    seeds = [b"global_config"],
    bump=global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: System account for vision mining admin, won't read or write
    pub vision_mining_admin_pubkey: UncheckedAccount<'info>,
}

impl<'info> ChangeVisionMiningAdmin<'info> {
    pub fn process(
        &mut self,
    ) -> Result<()> {
        require_keys_eq!(self.user.key(), self.global_config.admin, ErrorCode::Unauthorized);
        let global_config_account = &mut self.global_config;
        global_config_account.vision_mining_admin = self.vision_mining_admin_pubkey.key();
        Ok(())
    }
}
use crate::state::GlobalConfig;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;



#[derive(Accounts)]
pub struct InitializeGlobalConfig<'info> {
    #[account(
    init,
    payer = user,
    space = 8 + GlobalConfig::LEN,
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump)]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: PDA
    #[account(
    seeds = [GlobalConfig::VISION_MINING_SEED],
    bump)]
    pub vision_mining_pda: UncheckedAccount<'info>,
    /// CHECK: PDA
    #[account(
    seeds = [GlobalConfig::EVENT_MINING_SEED],
    bump)]
    pub event_mining_pda: UncheckedAccount<'info>,
    /// CHECK: PDA
    #[account(
    seeds = [GlobalConfig::STAKE_MINING_SEED],
    bump)]
    pub stake_mining_pda: UncheckedAccount<'info>,
    /// CHECK: System account for vision mining admin, won't read or write
    pub vision_mining_admin_pubkey: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGlobalConfig<'info> {
    pub fn process(
        &mut self,
    ) -> Result<()> {
        let (global_pda, global_bump) = GlobalConfig::find_global_config_account();
        let (vision_pda, vision_bump) = GlobalConfig::find_vision_mining_authority();
        let (event_pda, event_bump) = GlobalConfig::find_event_mining_authority();
        let (stake_pda, stake_bump) = GlobalConfig::find_stake_mining_authority();
        require_keys_eq!(global_pda, self.global_config.key(), ErrorCode::UnexpectedAccount);
        require_keys_eq!(vision_pda, self.vision_mining_pda.key(), ErrorCode::UnexpectedAccount);
        require_keys_eq!(event_pda, self.event_mining_pda.key(), ErrorCode::UnexpectedAccount);
        require_keys_eq!(stake_pda, self.stake_mining_pda.key(), ErrorCode::UnexpectedAccount);

        let global_config_account = &mut self.global_config;
        global_config_account.admin = self.user.key();
        global_config_account.vision_mining_pda = self.vision_mining_pda.key();
        global_config_account.event_mining_pda = self.event_mining_pda.key();
        global_config_account.stake_mining_pda = self.event_mining_pda.key();
        global_config_account.vision_mining_admin = self.vision_mining_admin_pubkey.key();
        global_config_account.global_config_bump = global_bump;
        global_config_account.vision_mining_bump = vision_bump;
        global_config_account.event_mining_bump = event_bump;
        global_config_account.stake_mining_bump = stake_bump;
        Ok(())
    }
}


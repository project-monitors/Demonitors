use crate::state::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{get_associated_token_address_with_program_id};
use anchor_spl::token_interface::{
    Mint, TokenAccount, TokenInterface,
    TransferChecked, transfer_checked};
use crate::chain_event::balance::{BalanceChangeEvent, BalanceChangeEventType, U64ValueChange};

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VisionMiningClaimParams {
    pub amount: u64,
    pub valid_until_time: u64,
}


#[derive(Accounts)]
pub struct VisionMiningClaim<'info> {
    #[account(
    mut)]
    pub payer: Signer<'info>,
    #[account(
    address = global_config.vision_mining_admin)]
    pub vision_mining_admin: Signer<'info>,
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump = global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
    seeds = [MintConfig::MINT_SEED],
    bump)]
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: PDA
    #[account(
    seeds = [GlobalConfig::VISION_MINING_SEED],
    bump = global_config.vision_mining_bump)]
    pub vision_mining_pda: UncheckedAccount<'info>,
    #[account(
    mut)]
    pub vision_mining_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
    mut)]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> VisionMiningClaim<'info> {

    //noinspection ALL
    pub fn get_ata(&self) -> Pubkey {
        get_associated_token_address_with_program_id(
            &self.vision_mining_pda.key(),
            &self.mint.key(),
            &self.token_program.key())
    }

    pub fn transfer_ctx(
        &self) -> CpiContext<'_, '_, '_,'info, TransferChecked<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.vision_mining_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.vision_mining_pda.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn process(
        &mut self,
        params: VisionMiningClaimParams) -> Result<()> {
        require_keys_eq!(self.get_ata(), self.vision_mining_token_account.key(), ErrorCode::UnexpectedAccount);
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;
        require!(current_time > 0, ErrorCode::Overflow);
        let current_time_u64 = current_time as u64;
        require!(params.valid_until_time > current_time_u64, ErrorCode::TransactionTimeout);
        let old_from_balance = self.vision_mining_token_account.amount.clone();
        let old_to_balance = self.token_account.amount.clone();
        let new_from_balance = old_from_balance.checked_sub(params.amount)
            .ok_or_else(|| error!(ErrorCode::NotSufficientBalance))?;
        let new_to_balance = old_to_balance.checked_add(params.amount)
            .ok_or_else(|| error!(ErrorCode::Overflow))?;
        let signer_seeds: [&[&[u8]]; 1] = [&[
            GlobalConfig::VISION_MINING_SEED,
            &[self.global_config.vision_mining_bump],
        ]];
        transfer_checked(
            self.transfer_ctx().with_signer(&signer_seeds),
            params.amount,
            self.mint.decimals)?;
        let event = BalanceChangeEvent{
            event_type: BalanceChangeEventType::Transfer,
            mint: self.mint.key(),
            from_token_account: Some(self.vision_mining_token_account.key()),
            from_change: Some(U64ValueChange{
                old: old_from_balance,
                new: new_from_balance,
            }),
            to_token_account: Some(self.token_account.key()),
            to_change: Some(U64ValueChange{
                old: old_to_balance,
                new: new_to_balance,
            })
        };

        emit!(event);

        Ok(())

    }
}
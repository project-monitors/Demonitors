use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{
        TokenAccount, Mint, MintTo, mint_to, TokenInterface},
    associated_token::{ AssociatedToken, get_associated_token_address_with_program_id}
};
use crate::state::{GlobalConfig, MintConfig};
use crate::FT_MAX_SUPPLY;
use crate::error::ErrorCode;
use crate::chain_event::balance::{BalanceChangeEvent, BalanceChangeEventType, U64ValueChange};

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MintTokensParams {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(
    mut,
    address = global_config.admin @ ErrorCode::Unauthorized)]
    pub payer: Signer<'info>,
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump = global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
    mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: We won't read or write to this account, just use it to calculate ATA
    pub user: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: PDA
    #[account(
    seeds = [MintConfig::AUTHORITY_SEED, &mint.key().to_bytes()],
    bump)]
    pub authority: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintTokens<'info> {

    //noinspection ALL
    pub fn get_ata(&self) -> Pubkey {
        get_associated_token_address_with_program_id(
            &self.user.key(),
            &self.mint.key(),
            &self.token_program.key())
    }

    pub fn mint_ctx(
        &self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>>{

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn process(
        &mut self,
        params: MintTokensParams) -> Result<()> {
        require_keys_eq!(self.get_ata(), self.token_account.key(), ErrorCode::UnexpectedAccount);
        let supply_after_mint = self.mint.supply.checked_add(params.amount)
            .ok_or_else(|| error!(ErrorCode::Overflow))?;
        require!(supply_after_mint <= FT_MAX_SUPPLY, ErrorCode::MintExceedMaxSupply);
        let old_to_balance = self.token_account.amount.clone();
        let new_to_balance = old_to_balance.checked_add(params.amount)
            .ok_or_else(|| error!(ErrorCode::Overflow))?;
        let (_, authority_bump) = MintConfig::find_authority(self.mint.key());
        let mint_key = self.mint.key();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            MintConfig::AUTHORITY_SEED,
            &mint_key.as_ref()[..],
            &[authority_bump],
        ]];
        mint_to(self.mint_ctx().with_signer(&signer_seeds), params.amount)?;

        let event = BalanceChangeEvent {
            event_type: BalanceChangeEventType::Mint,
            mint: self.mint.key(),
            from_token_account: None,
            from_change: None,
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
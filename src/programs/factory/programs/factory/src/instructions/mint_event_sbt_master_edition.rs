use crate::state::*;
use crate::error::ErrorCode;
use crate::chain_event::*;

use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::id as INSTRUCTIONS_ID,
};
use anchor_spl::{
    token_interface::{Mint, TokenInterface, TokenAccount},
    metadata::{
        Metadata,
    },
    metadata::mpl_token_metadata::instructions::MintV1CpiBuilder,
    associated_token::AssociatedToken,
};


#[derive(Accounts)]
pub struct MintEventSBTMasterEdition<'info> {
    #[account(
    mut)]
    pub payer: Signer<'info>,
    #[account(
    address = global_config.governor @ ErrorCode::Unauthorized)]
    pub governor: Signer<'info>,
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump = global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,
    /// CHECK: pda
    #[account(
    seeds = [MintConfig::AUTHORITY_SEED, &collection_mint.key().to_bytes()],
    bump)]
    pub authority: UncheckedAccount<'info>,
    #[account(
    seeds = [MintConfig::SBT_COLLECTION_SEED],
    bump)]
    pub collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
    mut,
    seeds = [MintConfig::SBT_MINT_SEED, &payer.key().to_bytes()],
    bump)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
    init,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = authority)]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: This is safe and will be checked by metaplex program
    #[account(
    mut,
    address = MintConfig::find_metadata(mint.key())?.0 @ ErrorCode::UnexpectedAccount)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is safe and will be checked by metaplex program
    #[account(
    mut,
    address = MintConfig::find_master_edition(mint.key())?.0 @ ErrorCode::UnexpectedAccount)]
    pub master_edition: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
    #[account(address = INSTRUCTIONS_ID())]
    /// CHECK: no need to check this
    pub sysvar_instruction: UncheckedAccount<'info>, // The sysvar instruction account
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> MintEventSBTMasterEdition<'info> {

    // noinspection RsTypeCheck
    pub fn process(
        &mut self,
    ) -> Result<()> {

        let mint = &self.mint;
        require_eq!(mint.supply, 0, ErrorCode::MintExceedMaxSupply);

        let (_, authority_bump) = MintConfig::find_authority(self.collection_mint.key());
        let collection_mint = self.collection_mint.key();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            MintConfig::AUTHORITY_SEED,
            &collection_mint.as_ref()[..],
            &[authority_bump],
        ]];

        MintV1CpiBuilder::new(&self.token_metadata_program.to_account_info())
            .token(&self.token_account.to_account_info())
            .metadata(&self.metadata.to_account_info())
            .master_edition(Some(&self.master_edition.to_account_info()))
            .mint(&self.mint.to_account_info())
            .authority(&self.authority.to_account_info())
            .payer(&self.payer.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .sysvar_instructions(&self.sysvar_instruction.to_account_info())
            .spl_token_program(&self.token_program.to_account_info())
            .spl_ata_program(&self.associated_token_program.to_account_info())
            .amount(1)
            .invoke_signed(&signer_seeds)?;

        emit!(
            SBTMintEvent{
                event_type: SBTMintEventType::MintSBT,
                mint_key: self.mint.key(),
                user_key: self.payer.key(),
            });

        Ok(())
    }
}
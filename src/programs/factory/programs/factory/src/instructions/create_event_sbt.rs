use crate::state::*;
use crate::error::ErrorCode;
use crate::chain_event::*;

use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::id as INSTRUCTIONS_ID,
    solana_program::system_instruction::create_account,
    solana_program::program::invoke,
    solana_program::program::invoke_signed
};
use anchor_spl::{
    token_interface::{Mint, TokenInterface},
    token_interface::spl_token_2022::instruction::{
        initialize_non_transferable_mint,
        initialize_mint,
    },
    metadata::{
        Metadata,
    },
    metadata::mpl_token_metadata::{
        types::{
            TokenStandard, PrintSupply
        },
        instructions::{
            CreateV1CpiBuilder
        },
    },
};


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateEventSBTParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub option: u8,
}


#[derive(Accounts)]
#[instruction(params: CreateEventSBTParams)]
pub struct CreateEventSBT<'info> {
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
    pub event_config: Account<'info, EventConfig>,
    /// CHECK: pda
    #[account(
    seeds = [MintConfig::AUTHORITY_SEED, &collection_mint.key().to_bytes()],
    bump)]
    pub authority: UncheckedAccount<'info>,
    #[account(
    seeds = [MintConfig::SBT_COLLECTION_SEED],
    bump)]
    pub collection_mint: InterfaceAccount<'info, Mint>,
    /// CHECK: create token 2022 mint manually
    /// PDA: b"sbt_mint" + event_config.key + option
    #[account(
    mut,
    seeds = [
    MintConfig::SBT_MINT_SEED,
    &event_config.key().to_bytes(),
    &[params.option]],
    bump)]
    pub mint: UncheckedAccount<'info>,
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
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateEventSBT<'info> {

    //noinspection RsTypeCheck
    pub fn process(
        &mut self,
        params: CreateEventSBTParams,
        bump: u8,
        authority_bump: u8
    ) -> Result<()> {
        require_neq!(params.option, 0, ErrorCode::InvalidArgument);
        require!(params.option <= self.event_config.option, ErrorCode::InvalidArgument);
        let signer_seeds: [&[&[u8]]; 1] = [&[
            MintConfig::SBT_MINT_SEED,
            &self.event_config.key().to_bytes(),
            &[params.option],
            &[bump],
        ]];

        // Create SPL token 2022 non-transferable manually
        let mint_ai = self.mint.to_account_info();
        let payer_ai = self.payer.to_account_info();
        let rent_ai = self.rent.to_account_info();

        // Mint account with non-transferable extension size 170
        let size: usize = 170;
        let lamports = Rent::get()?.minimum_balance(size);

        // 1. create mint account
        let create_account_ix = create_account(
            &self.payer.key(),
            &self.mint.key(),
            lamports,
            170,
            &self.token_program.key(),
        );

        invoke_signed(
            &create_account_ix,
            &[
                payer_ai.clone(),
                mint_ai.clone(),
            ],
            &signer_seeds
        )?;

        // 2. initialize non transferable extensions
        let init_non_transferable_ix = initialize_non_transferable_mint(
            &self.token_program.key(),
            &self.mint.key(),
        )?;

        invoke(
            &init_non_transferable_ix,
            &[
                mint_ai.clone()
            ],
        )?;

        // 3. initialize the mint
        let initialize_mint_ix = initialize_mint(
            &self.token_program.key(),
            &self.mint.key(),
            &self.authority.key(),
            Some(&self.authority.key()),
            0
        )?;

        invoke(
            &initialize_mint_ix,
            &[
                mint_ai.clone(),
                rent_ai.clone(),
            ]
        )?;

        // 4. Create metadata account

        let collection_mint_key = &self.collection_mint.key();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            MintConfig::AUTHORITY_SEED,
            &collection_mint_key.as_ref()[..],
            &[authority_bump],
        ]];

        CreateV1CpiBuilder::new(&self.token_metadata_program.to_account_info())
            .metadata(&self.metadata.to_account_info())
            .master_edition(Some(&self.master_edition.to_account_info()))
            .mint(&self.mint.to_account_info(), false)
            .authority(&self.authority.to_account_info())
            .payer(&self.payer.to_account_info())
            .update_authority(&self.authority.to_account_info(), false)
            .system_program(&self.system_program.to_account_info())
            .sysvar_instructions(&self.sysvar_instruction.to_account_info())
            .spl_token_program(&self.token_program.to_account_info())
            .name(params.name)
            .symbol(params.symbol)
            .uri(params.uri)
            .seller_fee_basis_points(0)
            .is_mutable(true)
            .token_standard(TokenStandard::NonFungible)
            .decimals(0)
            .print_supply(PrintSupply::Unlimited)
            .invoke_signed(&signer_seeds)?;

        emit!(
            SBTMintEvent{
                event_type: SBTMintEventType::CreateSBT,
                mint_key: self.mint.key(),
                user_key: self.payer.key(),
            });

        Ok(())
    }
}
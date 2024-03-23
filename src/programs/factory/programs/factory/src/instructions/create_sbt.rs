use crate::state::*;
use crate::error::ErrorCode;
use crate::chain_event::*;

use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::id as INSTRUCTIONS_ID,
    solana_program::system_instruction::create_account,
};
use anchor_lang::solana_program::program::invoke;
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
            TokenStandard, PrintSupply, Collection
        },
        instructions::{
            CreateV1CpiBuilder
        },
    },
};
use anchor_spl::token::spl_token::solana_program::program::invoke_signed;


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateSBTParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}


#[derive(Accounts)]
#[instruction(params: CreateSBTParams)]
pub struct CreateSBT<'info> {
    #[account(
    mut)]
    pub payer: Signer<'info>,
    // TODO: Add governor for multi-signing metadata.
    // pub governor: Signer<'info>,
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
    #[account(
    mut,
    seeds = [MintConfig::SBT_MINT_SEED, &payer.key().to_bytes()],
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

impl<'info> CreateSBT<'info> {

    //noinspection RsTypeCheck
    pub fn process(
        &mut self,
        params: CreateSBTParams,
        bump: u8
    ) -> Result<()> {


        let signer_seeds: [&[&[u8]]; 1] = [&[
            MintConfig::SBT_MINT_SEED,
            &self.payer.key().to_bytes(),
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

        // msg!("THE AUTHORITY: {}", &self.authority.key().to_string());

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

        // let mint_ai = self.mint.to_account_info();
        // let a = Self::get_mint_authority(&mint_ai)?;
        //
        // msg!("THE AUTHORITY READ FROM ACCOUNT INFO: {}", &a.to_string());

        // Create metadata account

        let (_, authority_bump ) = MintConfig::find_authority(self.collection_mint.key());
        let mint_key = &self.collection_mint.key();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            MintConfig::AUTHORITY_SEED,
            &mint_key.as_ref()[..],
            &[authority_bump],
        ]];

        let collection_key = self.collection_mint.key();

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
            .is_mutable(false)
            .collection(Collection{verified: false, key: collection_key})
            .token_standard(TokenStandard::NonFungible)
            .decimals(0)
            .print_supply(PrintSupply::Zero)
            .invoke_signed(&signer_seeds)?;

        emit!(
            SBTMintEvent{
                event_type: SBTMintEventType::CreateSBT,
                mint_key: self.mint.key(),
                user_key: self.payer.key(),
            });

        // let mint_ai = self.mint.to_account_info();
        // let a = Self::get_mint_authority(&mint_ai)?;
        //
        // msg!("THE AUTHORITY READ FROM ACCOUNT INFO AFTER METADATA_PROGRAM: {}", &a.to_string());

        Ok(())
    }
}
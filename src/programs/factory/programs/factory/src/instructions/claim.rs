use anchor_lang::{
    prelude::*,
    solana_program::{
        program::{invoke, invoke_signed},
        system_instruction::create_account,
        sysvar::instructions::id as INSTRUCTIONS_ID,
    }
};
use anchor_spl::{
    metadata::{
        Metadata,
        mpl_token_metadata::instructions::PrintV1CpiBuilder,
    },
    token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked,
                      spl_token_2022::instruction::{initialize_mint, initialize_non_transferable_mint}}
};
use anchor_spl::associated_token::AssociatedToken;

use crate::chain_event::*;
use crate::state::*;
use crate::error::ErrorCode;
use crate::utils::get_ata;
use crate::ID;


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ClaimParams {
    pub indicate: u8,
    pub sbt_mint: Pubkey
}

#[derive(Accounts)]
#[instruction(params: ClaimParams)]
pub struct Claim<'info> {
    // payer
    #[account(
    mut)]
    pub payer: Signer<'info>,

    // config
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump = global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,

    // event accounts
    #[account(
    address = event_market.event_config @ ErrorCode::UnexpectedAccount)]
    pub event_config: Account<'info, EventConfig>,
    #[account(
    constraint = event_market.result != 0 @ ErrorCode::EventIsOngoing)]
    pub event_market: Account<'info, EventMarket>,
    #[account(
    mut,
    constraint = marker.indicate == params.indicate @ ErrorCode::InvalidArgument,
    seeds = [Marker::MARKER_SEED, &payer.key().to_bytes(), &params.sbt_mint.to_bytes()],
    bump)]
    pub marker: Account<'info, Marker>,
    #[account(
    mut,
    close = payer,
    seeds = [UserPosition::POSITION_SEEDS, &event_market.key().to_bytes(), &payer.key().to_bytes()],
    bump)]
    pub user_position: Account<'info, UserPosition>,

    // FT token accounts
    #[account(
    seeds = [MintConfig::MINT_SEED],
    bump)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = payer,
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: pda
    #[account(
    address = global_config.event_mining_pda)]
    pub event_mining_pda: UncheckedAccount<'info>,
    #[account(
    mut,
    address = get_ata(&event_mining_pda.key(), &mint.key(), &token_program.key()))]
    pub event_mining_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    // SBT edition account
    /// CHECK: create token 2022 mint manually
    #[account(
    mut,
    seeds = [MintConfig::SBT_MINT_SEED,
    &event_config.key().to_bytes(),
    &[params.indicate],
    &payer.key().to_bytes()],
    bump)]
    pub event_sbt_edition_mint: UncheckedAccount<'info>,
    /// CHECK: This is safe and will be checked and created by metaplex program
    #[account(
    mut,
    address = get_ata(&payer.key(), &event_sbt_edition_mint.key(), &token_program.key()))]
    pub event_sbt_edition_token_account:UncheckedAccount<'info>,
    /// CHECK: This is safe and will be checked by metaplex program
    #[account(
    mut,
    address = MintConfig::find_metadata(mint.key())?.0 @ ErrorCode::UnexpectedAccount)]
    pub event_sbt_edition_metadata: UncheckedAccount<'info>,
    /// CHECK: This is safe and will be checked by metaplex program
    #[account(
    mut)]
    pub event_sbt_edition: UncheckedAccount<'info>,
    /// CHECK: This is safe and will be checked by metaplex program
    #[account(
    mut)]
    pub event_sbt_edition_pda: UncheckedAccount<'info>,

    // SBT master edition accounts
    /// CHECK: PDA
    pub authority: UncheckedAccount<'info>,
    pub collection_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
    mut,
    seeds = [
    MintConfig::SBT_MINT_SEED,
    &event_config.key().to_bytes(),
    &[params.indicate]],
    bump)]
    pub event_sbt_master_edition_mint: Box<InterfaceAccount<'info, Mint>>,
    pub event_sbt_master_edition_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: This is safe and will be checked by metaplex program
    pub event_sbt_master_edition_metadata: UncheckedAccount<'info>,
    /// CHECK: This is safe and will be checked by metaplex program
    #[account(
    mut,
    address = MintConfig::find_master_edition(event_sbt_master_edition_mint.key())?.0 @ ErrorCode::UnexpectedAccount)]
    pub event_sbt_master_edition: UncheckedAccount<'info>,

    // program accounts
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = INSTRUCTIONS_ID())]
    /// CHECK: no need to check this
    pub sysvar_instruction: UncheckedAccount<'info>, // The sysvar instruction account
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Claim<'info> {

    pub fn preflight(
        &self,
        params: ClaimParams
    ) -> Result<()> {
        let sbt_mint = Pubkey::find_program_address(
            &[MintConfig::SBT_MINT_SEED,
                self.payer.key().as_ref(),
            ],
            &ID
        ).0;
        require_eq!(params.sbt_mint, sbt_mint, ErrorCode::InvalidArgument);
        require_neq!(self.user_position.existed, false, ErrorCode::PositionNotFound);
        Ok(())
    }

    pub fn transfer_ctx(
        &self) -> CpiContext<'_, '_, '_,'info, TransferChecked<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.event_mining_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.event_mining_pda.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn process(
        &mut self,
        params: ClaimParams,
        event_sbt_mint_bump: u8
    ) -> Result<()> {
        let indicate = params.indicate;

        self.preflight(params)?;

        let marker = &mut self.marker;
        marker.indicate = 0;

        if indicate != self.event_market.result {
            return Ok(())
        } else {
            // Transfer tokens
            let old_from_balance = self.event_mining_token_account.amount.clone();
            let old_to_balance = self.token_account.amount.clone();
            let new_from_balance =
                self.event_mining_token_account.amount.checked_sub(
                    self.event_market.prize).ok_or(ErrorCode::NotSufficientBalance)?;
            let new_to_balance =
                self.token_account.amount.checked_add(
                    self.event_market.prize).ok_or(ErrorCode::Overflow)?;
            let signer_seeds: [&[&[u8]]; 1] = [&[
                GlobalConfig::EVENT_MINING_SEED,
                &[self.global_config.event_mining_bump],
            ]];
            transfer_checked(
                self.transfer_ctx().with_signer(&signer_seeds),
                self.event_market.prize,
                self.mint.decimals)?;
            let event = BalanceChangeEvent {
                event_type: BalanceChangeEventType::Transfer,
                mint: self.mint.key(),
                from_token_account: Some(self.event_mining_token_account.key()),
                from_change: Some(U64ValueChange {
                    old: old_from_balance,
                    new: new_from_balance,
                }),
                to_token_account: Some(self.token_account.key()),
                to_change: Some(U64ValueChange {
                    old: old_to_balance,
                    new: new_to_balance,
                })
            };
            emit!(event);

            // check if own event sbt

            let sbt_edition_mint_ai = self.event_sbt_edition_mint.to_account_info();
            let data_len = sbt_edition_mint_ai.data.borrow().len();
            if data_len == 0 {
                msg!("This mint account doesn't existed");

                let collection_mint_key = self.collection_mint.key();
                let (authority_key, authority_bump) = MintConfig::find_authority(collection_mint_key);
                require_keys_eq!(authority_key, self.authority.key(), ErrorCode::UnexpectedAccount);



                let payer_ai = self.payer.to_account_info();
                let rent_ai = self.rent.to_account_info();

                let size: usize = 170;
                let lamports = Rent::get()?.minimum_balance(size);

                let signer_seeds: [&[&[u8]]; 1] = [&[
                    MintConfig::SBT_MINT_SEED,
                    &self.event_config.key().to_bytes(),
                    &[indicate],
                    &self.payer.key().to_bytes(),
                    &[event_sbt_mint_bump],
                ]];

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
                        sbt_edition_mint_ai.clone(),
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
                        sbt_edition_mint_ai.clone()
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
                        sbt_edition_mint_ai.clone(),
                        rent_ai.clone(),
                    ]
                )?;

                // Print new edition to payer

                let signer_seeds: [&[&[u8]]; 1] = [&[
                    MintConfig::AUTHORITY_SEED,
                    &collection_mint_key.as_ref(),
                    &[authority_bump],
                ]];

                PrintV1CpiBuilder::new(&self.token_metadata_program.to_account_info())
                    .edition_metadata(&self.event_sbt_edition_metadata.to_account_info())
                    .edition(&self.event_sbt_edition.to_account_info())
                    .edition_mint(&self.event_sbt_edition_mint.to_account_info(), false)
                    .edition_token_account_owner(&self.payer.to_account_info())
                    .edition_token_account(&self.event_sbt_edition_token_account.to_account_info())
                    .edition_mint_authority(&self.authority.to_account_info())
                    .master_edition(&self.event_sbt_master_edition.to_account_info())
                    .edition_marker_pda(&self.event_sbt_edition_pda)
                    .payer(&self.payer.to_account_info())
                    .master_token_account_owner(&self.authority.to_account_info())
                    .master_token_account(&self.event_sbt_master_edition_token_account.to_account_info())
                    .master_metadata(&self.event_sbt_master_edition_metadata.to_account_info())
                    .update_authority(&self.authority.to_account_info())
                    .spl_token_program(&self.token_program.to_account_info())
                    .spl_ata_program(&self.associated_token_program.to_account_info())
                    .sysvar_instructions(&self.sysvar_instruction.to_account_info())
                    .system_program(&self.system_program.to_account_info())
                    .edition_number(1)
                    .invoke_signed(&signer_seeds)?;

                emit!(SBTMintEvent{
                    event_type: SBTMintEventType::PrintSBT,
                    mint_key: self.event_sbt_edition_mint.key(),
                    user_key: self.payer.key(),
                });
            }
        }
        Ok(())
    }
}
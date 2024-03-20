use crate::state::*;
use crate::error::ErrorCode;
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::id as INSTRUCTIONS_ID,
};
use anchor_spl::{
    token_interface::{Mint, TokenInterface },
    metadata::{
        Metadata,
    },
    metadata::mpl_token_metadata::{
        types::{
            TokenStandard, CollectionDetails, Creator, PrintSupply
        },
        instructions::{
            CreateV1CpiBuilder
        },
    },
};


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct InitializeCollectionParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}


#[derive(Accounts)]
#[instruction(params: InitializeCollectionParams)]
pub struct InitializeCollection<'info> {
    #[account(
    mut,
    address = global_config.admin @ ErrorCode::Unauthorized)]
    pub payer: Signer<'info>,
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump = global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,
    /// CHECK: pda
    #[account(
    seeds = [MintConfig::AUTHORITY_SEED, &mint.key().to_bytes()],
    bump)]
    pub authority: UncheckedAccount<'info>,
    #[account(
    init,
    payer = payer,
    mint::decimals = 0,
    mint::authority = authority,
    mint::freeze_authority = authority,
    seeds = [MintConfig::SBT_COLLECTION_SEED],
    bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
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

impl<'info> InitializeCollection<'info> {

    pub fn process(
        &mut self,
        params: InitializeCollectionParams
    ) -> Result<()> {

        let (_, authority_bump ) = MintConfig::find_authority(self.mint.key());
        let mint_key = &self.mint.key();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            MintConfig::AUTHORITY_SEED,
            &mint_key.as_ref()[..],
            &[authority_bump],
        ]];
        let creator = vec![
            Creator {
                address: self.authority.key.clone(),
                verified: true,
                share: 100,
            }
        ];

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
            .creators(creator)
            .is_mutable(true)
            .token_standard(TokenStandard::NonFungible)
            .collection_details(CollectionDetails::V1 {size:0})
            .decimals(0)
            .print_supply(PrintSupply::Zero)
            .invoke_signed(&signer_seeds)?;

        Ok(())

    }
}

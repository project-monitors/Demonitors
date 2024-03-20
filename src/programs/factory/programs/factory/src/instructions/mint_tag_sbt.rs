use crate::state::*;
use crate::error::ErrorCode;
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::id as INSTRUCTION_ID,
};
use anchor_spl::{
    token_interface::{Mint, TokenInterface },
    associated_token::AssociatedToken,
    metadata::{
        Metadata,
    },
    metadata::mpl_token_metadata::{
        types::TokenStandard,
        instructions::CreateV1CpiBuilder
    },
};



#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct MintTagSBTParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}


#[derive(Accounts)]
#[instruction(params: MintTagSBTParams)]
pub struct MintTagSBT<'info> {
    #[account(
    mut)]
    pub payer: Signer<'info>,
}
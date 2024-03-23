use crate::state::*;
use crate::error::ErrorCode;
use crate::chain_event::*;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct WithdrawParams {
    pub indicate: u8
}

#[derive(Accounts)]
#[instruction(params: WithdrawParams)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
    seeds = [MintConfig::SBT_MINT_SEED, &payer.key().to_bytes()],
    bump)]
    pub sbt_mint: InterfaceAccount<'info, Mint>,
    #[account(
    mut,
    constraint = marker.indicate == params.indicate @ ErrorCode::PositionNotFound,
    seeds = [Marker::MARKER_SEED, &payer.key().to_bytes(), &sbt_mint.key().to_bytes()],
    bump)]
    pub marker: Account<'info, Marker>,
    #[account(
    mut,
    close = payer,
    seeds = [UserPosition::POSITION_SEEDS, &event_market.key().to_bytes(), &payer.key().to_bytes()],
    bump)]
    pub user_position: Account<'info, UserPosition>,
    #[account(
    mut,
    seeds = [EventPosition::POSITION_SEEDS, &event_market.key().to_bytes(), &[params.indicate]],
    bump)]
    pub event_position: Account<'info, EventPosition>,
    pub event_market: Account<'info, EventMarket>,
    pub system_program: Program<'info, System>
}

impl<'info> Withdraw<'info> {

    pub fn process(
        &mut self
    ) -> Result<()> {

        let marker = &mut self.marker;
        let event_position = &mut self.event_position;
        marker.indicate = 0;
        event_position.amount = event_position.amount.checked_sub(1).ok_or(ErrorCode::Overflow)?;

        emit!(ChooseEvent{
            event_type: ChooseEventType::Withdraw,
            event_market: self.event_market.key(),
            user_key: self.payer.key(),
            indicate: 0,
        });

        Ok(())
    }
}
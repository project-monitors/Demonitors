use crate::state::*;
use crate::utils::get_ata;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenAccount, ID as TOKEN_2022_PROGRAM_ID},
};
use crate::chain_event::*;


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ChooseParams {
    pub indicate: u8
}


#[derive(Accounts)]
#[instruction(params: ChooseParams)]
pub struct Choose<'info> {
    #[account(
    mut)]
    pub payer: Signer<'info>,
    #[account(
    seeds = [MintConfig::SBT_MINT_SEED, &payer.key().to_bytes()],
    bump)]
    pub sbt_mint: InterfaceAccount<'info, Mint>,
    #[account(
    address = get_ata(&payer.key(), &sbt_mint.key(), &TOKEN_2022_PROGRAM_ID)
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
    mut,
    seeds = [Marker::MARKER_SEED, &payer.key().to_bytes(), &sbt_mint.key().to_bytes()],
    bump)]
    pub marker: Account<'info, Marker>,
    #[account(
    init_if_needed,
    payer = payer,
    space = 8 + UserPosition::LEN,
    seeds = [UserPosition::POSITION_SEEDS, &event_market.key().to_bytes(), &payer.key().to_bytes()],
    bump)]
    pub user_position: Account<'info, UserPosition>,
    #[account(
    init_if_needed,
    payer = payer,
    space = 8 + EventPosition::LEN,
    seeds = [EventPosition::POSITION_SEEDS, &event_market.key().to_bytes(), &[params.indicate]],
    bump)]
    pub event_position: Account<'info, EventPosition>,
    #[account(
    address = event_market.event_config @ ErrorCode::UnexpectedAccount)]
    pub event_config: Account<'info, EventConfig>,
    pub event_market: Account<'info, EventMarket>,
    pub system_program: Program<'info, System>,
}

impl<'info> Choose<'info> {

    pub fn process(
        &mut self,
        params: ChooseParams
    ) -> Result<()> {

        require_eq!(self.event_market.is_opened, true, ErrorCode::EventIsNotOpen);
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        require!(now > 0, ErrorCode::Overflow);
        let now_u64 = now as u64;
        require!(self.event_market.expiry_ts > now_u64, ErrorCode::EventIsOutOfTime);
        require!(self.event_market.close_ts > now_u64, ErrorCode::EventIsOutOfTime);
        require!(params.indicate <= self.event_market.option, ErrorCode::IndicateToNonExistedOption);
        require!(self.token_account.amount > 0, ErrorCode::SBTNotFound);
        require_eq!(self.marker.indicate, 0, ErrorCode::SBTHasBeenInUse);
        require_eq!(self.user_position.existed, false, ErrorCode::AlreadyInEvent);

        let marker = &mut self.marker;
        let user_position = &mut self.user_position;
        let event_position = &mut self.event_position;
        marker.event_market = Some(self.event_market.key());
        marker.indicate = params.indicate;
        user_position.position_type = PositionType::UserSBTPosition;
        user_position.existed = true;
        user_position.marker = self.marker.key();
        event_position.position_type = PositionType::EventSBTPosition;
        event_position.amount = event_position.amount.checked_add(1).ok_or(ErrorCode::Overflow)?;

        emit!(ChooseEvent{
            event_type: ChooseEventType::Choose,
            event_market: self.event_market.key(),
            user_key: self.payer.key(),
            indicate: params.indicate,
        });

        Ok(())
    }
}
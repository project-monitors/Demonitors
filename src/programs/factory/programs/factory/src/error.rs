use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("The account has already been initialized.")]
    ReInitialize,
    #[msg("The account has not been initialized.")]
    UnInitialize,
    #[msg("Argument is invalid.")]
    InvalidArgument,
    #[msg("Program ID is invalid.")]
    InvalidProgramId,
    #[msg("Unexpected Account.")]
    UnexpectedAccount,
    #[msg("An overflow occurs.")]
    Overflow,
    #[msg("The string variable is too long.")]
    StringTooLong,
    #[msg("Authorities limit reached")]
    TooManyAuthorities,
    #[msg("Authority not found")]
    AuthorityNotFound,
    #[msg("Oracle config mismatched")]
    ConfigMismatched,
    #[msg("Minting exceeds max supply limit")]
    MintExceedMaxSupply,
    #[msg("The transfer_from account does not have sufficient balance")]
    NotSufficientBalance,
    #[msg("The transaction is timeout")]
    TransactionTimeout,
    #[msg("Close account failed")]
    CloseAccountFailed,
    #[msg("Unsupported now")]
    UnsupportedNow,
    #[msg("Get oracle data error")]
    OracleDataError,
    #[msg("Event is finalized")]
    EventIsFinalized,
    #[msg("Event is not open")]
    EventIsNotOpen,
    #[msg("Event is out of time for choosing")]
    EventIsOutOfTime,
    #[msg("Indicate to a non-existed option")]
    IndicateToNonExistedOption,
    #[msg("SBT which use for indicating is not found")]
    SBTNotFound,
    #[msg("SBT has been in use")]
    SBTHasBeenInUse,
    #[msg("user takes in this event already")]
    AlreadyInEvent,
    #[msg("user's position is not found")]
    PositionNotFound,
    #[msg("event is still ongoing, can not be resolved now")]
    EventIsOngoing,
    #[msg("event market does not have original data")]
    EventMarketDataError,
}
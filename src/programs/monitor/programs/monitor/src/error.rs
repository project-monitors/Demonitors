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

}

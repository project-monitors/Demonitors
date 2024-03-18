use crate::state::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenInterface },
    associated_token::AssociatedToken,
    metadata::{
        CreateMetadataAccountsV3,
        Metadata,
    },
    metadata::self,
    metadata::mpl_token_metadata::types::DataV2
};



/// Parameters for initializing the governance token mint account
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct InitializeMintParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}


/// Initializes the governance token mint account
#[derive(Accounts)]
#[instruction(params: InitializeMintParams)]
pub struct InitializeMint<'info> {
    #[account(
    mut,
    address = global_config.admin @ ErrorCode::Unauthorized)]
    pub payer: Signer<'info>,
    /// CHECK: pda
    #[account(
    seeds = [MintConfig::AUTHORITY_SEED, &mint.key().to_bytes()],
    bump)]
    pub authority: UncheckedAccount<'info>,
    #[account(
    init,
    payer = payer,
    mint::decimals = params.decimals,
    mint::authority = authority,
    mint::freeze_authority = authority,
    seeds = [MintConfig::MINT_SEED],
    bump)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
    init,
    payer = payer,
    space = 8 + MintConfig::LEN,
    seeds = [MintConfig::MINT_CONFIG_SEED],
    bump)]
    pub mint_config: Account<'info, MintConfig>,
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump)]
    pub global_config: Account<'info, GlobalConfig>,
    // /// CHECK: PDA
    // #[account(
    // seeds = [GlobalConfig::VISION_MINING_SEED],
    // bump = global_config.vision_mining_bump,
    // constraint = vision_mining_pda.key() == global_config.vision_mining_pda @ ErrorCode::UnexpectedAccount
    // )]
    // pub vision_mining_pda: UncheckedAccount<'info>,
    // /// CHECK: PDA
    // #[account(
    // seeds = [GlobalConfig::EVENT_MINING_SEED],
    // bump = global_config.event_mining_bump,
    // constraint = event_mining_pda.key() == global_config.event_mining_pda @ ErrorCode::UnexpectedAccount
    // )]
    // pub event_mining_pda: UncheckedAccount<'info>,
    // /// CHECK: PDA
    // #[account(
    // seeds = [GlobalConfig::STAKE_MINING_SEED],
    // bump = global_config.stake_mining_bump,
    // constraint = stake_mining_pda.key() == global_config.stake_mining_pda @ ErrorCode::UnexpectedAccount
    // )]
    // pub stake_mining_pda: UncheckedAccount<'info>,
    // #[account(
    // init,
    // payer = payer,
    // associated_token::mint = mint,
    // associated_token::authority = vision_mining_pda,
    // )]
    // pub vision_mining_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    // #[account(
    // init,
    // payer = payer,
    // associated_token::mint = mint,
    // associated_token::authority = event_mining_pda,
    // )]
    // pub event_mining_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    // #[account(
    // init,
    // payer = payer,
    // associated_token::mint = mint,
    // associated_token::authority = stake_mining_pda,
    // )]
    // pub stake_mining_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK - create token metadata account manually
    #[account(
    zero,
    owner = crate::ID,
    address = MintConfig::find_metadata(mint.key())?.0 @ ErrorCode::UnexpectedAccount,
    )]
    pub metadata_account: AccountInfo<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeMint<'info> {

    //noinspection ALL
    pub fn create_metadata_accounts_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, CreateMetadataAccountsV3<'info>> {
        let program = self.token_metadata_program.to_account_info();

        let accounts = CreateMetadataAccountsV3 {
            metadata: self.metadata_account.to_account_info(),
            mint: self.mint.to_account_info(),
            mint_authority: self.authority.to_account_info(),
            payer: self.payer.to_account_info(),
            update_authority: self.authority.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }


    pub fn process(
        &mut self,
        params: InitializeMintParams) -> Result<()> {
        let (global_pda, _) = GlobalConfig::find_global_config_account();
        let (mint_pda, mint_bump) = MintConfig::find_mint(None);
        let (mint_config_pda, mint_config_bump) = MintConfig::find_mint_config(None);
        let (authority_pda, authority_bump ) = MintConfig::find_authority(mint_pda.key());
        let (_, metadata_bump) = MintConfig::find_metadata(self.mint.key())?;
        require_keys_eq!(global_pda, self.global_config.key(), ErrorCode::UnexpectedAccount);
        require_keys_eq!(mint_pda, self.mint.key(), ErrorCode::UnexpectedAccount);
        require_keys_eq!(mint_config_pda, self.mint_config.key(), ErrorCode::UnexpectedAccount);
        require_keys_eq!(authority_pda, self.authority.key(), ErrorCode::UnexpectedAccount);
        let mint_key = self.mint.key();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            MintConfig::AUTHORITY_SEED,
            &mint_key.as_ref()[..],
            &[authority_bump],
        ]];
        metadata::create_metadata_accounts_v3(
            self.create_metadata_accounts_ctx()
                .with_signer(&signer_seeds),
            DataV2 {
                name: params.name,
                symbol: params.symbol,
                uri: params.uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;
        let mint_config = &mut self.mint_config;
        mint_config.mint = self.mint.key();
        mint_config.metadata = self.metadata_account.key();
        mint_config.mint_bump = mint_bump;
        mint_config.config_bump = mint_config_bump;
        mint_config.metadata_bump = metadata_bump;
        mint_config.authority_bump = authority_bump;
        Ok(())
    }
}



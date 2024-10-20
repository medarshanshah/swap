use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,}
};

use crate::Offer;

use super::transfer_tokens;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed, // the taker may not have any balance previously or is their account is not already created
        payer = taker,  
        associated_token::mint = token_mint_a, 
        associated_token::authority = taker,    // authority is account level owner
        associated_token::token_program = token_program
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,   //tokens from vault will be put to takers's token account a

    #[account(
        mut,  
        associated_token::mint = token_mint_b, 
        associated_token::authority = taker,    // authority is account level owner
        associated_token::token_program = token_program
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,   //tokens from vault will be put to takers's token account b

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b, 
        associated_token::authority = maker,    // maker since it is maker's token_account,authority is account level owner
        associated_token::token_program = token_program
    )]
    pub maker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,            // mutable since the account should be closed once offer is taken
        close = maker,  // while closing the account, the SOL will be refunded to maker, because the maker made the offer account
        has_one = maker,    
        has_one = token_mint_a,
        has_one = token_mint_b,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,    // mutable since all the tokens will be taken out and given to the taker
        associated_token::mint = token_mint_a,  // token_mint_a because token of maker is added to the vault
        associated_token::authority = offer,    // neither be maker or taker, controlled by the offer account
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_wanted_tokens_to_maker(context: &Context<TakeOffer>) -> Result<()> {
    transfer_tokens(
        &context.accounts.taker_token_account_b,
        &context.accounts.maker_token_account_b,
        &context.accounts.offer.token_b_wanted_amount,
        &context.accounts.token_mint_b,
        &context.accounts.taker,
        &context.accounts.token_program,
    )
}

// The transfer will not take place from the regular account. Instead it will be from vault account(owned by offer account)
    // This is a PDA transfer, will be signed by the offer account
    // The helper function transfer_tokens will not be reused since it is foucused on different kind of transfer
    // The transfer_checked will be called with slightly different options
pub fn withdraw_and_close_vault(context: Context<TakeOffer>) -> Result<()> {

    let seeds = &[
        b"offer",
        context.accounts.maker.to_account_info().key.as_ref(),
        &context.accounts.offer.id.to_le_bytes()[..],
        &[context.accounts.offer.bump],
    ];

    let signer_seeds = [&seeds[..]];

    let accounts = TransferChecked {
        from: context.accounts.vault.to_account_info(),
        to: context.accounts.taker_token_account_a.to_account_info(),   // to is taker, because we take the tokens from the vult and put them in taker's token account a because the token in the vault are of type token a
        mint: context.accounts.token_mint_a.to_account_info(),          
        authority: context.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),   // Combining the token program, 
        accounts,                                                   // accounts (options for transfer_checked)
        &signer_seeds,                                              // and signer_seeds into a context
    );

    transfer_checked(
        cpi_context,
        context.accounts.vault.amount,  // amount from vault
        context.accounts.token_mint_a.decimals,
    )?;

    let accounts = CloseAccount {
        account: context.accounts.vault.to_account_info(),      // vault that is to be closed
        destination: context.accounts.taker.to_account_info(),  // refund lamports to maker or taker
        authority: context.accounts.offer.to_account_info(),    // offer account owns the vault
    };

    // context to close vault account
    let cpi_context = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        accounts,
        &signer_seeds,
    );

    close_account(cpi_context)

}
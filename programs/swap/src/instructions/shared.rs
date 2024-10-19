use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,   // InterfaceAccount is used for program to work on both old token program
    to: &InterfaceAccount<'info, TokenAccount>,     // or the new token extensions program. With TokenAccount we can only work on either of those
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>
) -> Result<()> {
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        token_program.to_account_info(), 
        transfer_accounts_options);

    //Transferring tokens is done by calling the transfer_checked function of tokens program. 
    // It has some logic to verify mints and a bunch of useful safety features
    transfer_checked(cpi_context, *amount, mint.decimals);
    Ok(())
}
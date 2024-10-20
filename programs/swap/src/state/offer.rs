use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,          // the one who makes the offer
    pub token_mint_a: Pubkey,   // token_mint_a
    pub token_mint_b: Pubkey,   // token_mint_b
    pub token_b_wanted_amount: u64, //token they want in exchange of what they have put into vault
    pub bump: u8,
}
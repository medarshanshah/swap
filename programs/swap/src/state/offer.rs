use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: PubKey,          // the one who makes the offer
    pub token_mint_a: PubKey,   // token_mint_a
    pub token_mint_b: PubKey,   // token_mint_b
    pub token_b_wanted_amount: u64, //token they want in exchange of what they have put into vault
    pub bump: u8,
}
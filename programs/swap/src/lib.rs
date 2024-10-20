pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3grHHMnoDqkd3qHmBUoXGaz22C2fhnwsVsywRbLwbUwt");

#[program]
pub mod swap {
    use super::*;

    //Instruction handler: make_offer
    // This will send tokens into a vault and write information about what the person wants in exchange of those tokens
    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&context, token_a_offered_amount)?;  // ? -> this is for error handling, to tell the compiler that error is expected
        instructions::make_offer::save_offer(context, id, token_b_wanted_amount)
    }

    //Instrution handler: take_offer
    // This allows somebody to take the tokens from the vault and also sens their tokens directly to the person that made the offer
    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_wanted_tokens_to_maker(context)?;
        instructions::take_offer::withdraw_and_close_vault(context)
    }
}

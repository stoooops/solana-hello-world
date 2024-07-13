use anchor_lang::prelude::*;

declare_id!("3D6yHdWhzmmaR1rSinGfqCp9DcgAG6nEpD97Wx7CkXzi");

#[program]
pub mod solana_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// #[account] is a custom Rust attribute that defines a Solana account.
// Anchor essentially is telling the Rust compiler this is a Solana account.
// There’s a lot of detail that Anchor abstracts away from us to be able to do this!
// Pretty cool 😄!
#[account]
// pub struct Message is a Rust struct that defines the properties of a Message we want our program to interact with.
// You’ll note we want to store the content of the most recent message, the timestamp when it was published, and the author.
pub struct Message {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}

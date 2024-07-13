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

// #[derive(Accounts)] is a custom Rust attribute that defines the context for an instruction as a Rust struct.
// Again, Anchor abstracts a lot away so we can develop faster.

#[derive(Accounts)]
// The message property is the actual account that the instruction will create.
// When we call the instruction, we’re going to pass in a public key to use for this account.

pub struct CreateMessage<'info> {
    // The [account(init, payer = author, space = 1000)] is an account constraint on this property - it tells Rust that message is a Solana account.
    // - The init says that this instruction will create the account, through the System Program.
    // - The payer says that the author property is who will pay to initialize and keep the data on Solana.
    // - The space says the big the account data will be in bytes. For sake of simplicity, we’re saying a message can be at most 1000 bytes.
    //   Normally we would spend time getting efficient about how much space our data will take up, since it determines how much SOL we have to spend.
    //   That is out of the scope of this introductory tutorial, though 🙂.
        #[account(init, payer = author, space = 1000)]
    pub message: Account<'info, Message>,
    // The author property is the actual sender of the message. It’s a Signer account because we will pass in their signature.
    // It’s has a mutable account constraint since we will be modifying their balance of SOL when initializing the message account.
        #[account(mut)]
    pub author: Signer<'info>,
    // The system_program is Solana’s System Program that will initialize the message account.
    pub system_program: Program<'info, System>,
    // There’s also the 'info which is a Rust lifetime, or essentially a generic which in this case represents an AccountInfo class - a struct that contains the fields of account, listed here.
    // No need to worry about this deeply yet
}
#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    // For this one, we only need to include message and author.
    // Note that message has a different account constraint - since the instruction using this context will be updating the message,
    // we need to make that account mutable.
		#[account(mut)]
    pub message: Account<'info, Message>,
		#[account(mut)]
    pub author: Signer<'info>,
}

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

// Now that we’ve defined the Message account, we can implement instructions as part our the Solana program.
// We’re going implement two instructions - one to create the first message, and one to update that message.

// Because Solana programs are stateless, we need to provide relevant context for each of those instructions.
// To do so, Anchor gives us a way to easily define a context in Rust.

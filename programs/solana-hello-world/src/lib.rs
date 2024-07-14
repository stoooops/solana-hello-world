use anchor_lang::prelude::*;

declare_id!("AGg9tfdZ1sAazoYCdxFgWsEb7KgZQi3Ce64RgSnNh1wz");

#[program]
pub mod solana_hello_world {
    use super::*;

    // we’ll define two instructions for our Solana program - one to read the message, and one to update the message.
    // Instructions are Rust functions defined within a module with the Rust attribute #[program]

    // As inputs to the create_message function, we take in the Context ctx that we defined in the last section for type
    // CreateMessage, and content we want for store as the message.
    pub fn create_message(ctx: Context<CreateMessage>, content: String) -> Result<()> {
        // The next line initializes a message variable by accessing the account that was created as part of this
        // function call through that init account constraint in the context.
        // We want a reference to to that account so we use the & as a referencer, and we use mutto be able to mutate
        // the data.
        let message: &mut Account<Message> = &mut ctx.accounts.message;
        // The next line does something similar, initializing the author variable as aSigner account to save it on the
        // message account.
        let author: &Signer = &ctx.accounts.author;
        // The next line creates a timestamp clock using Solana’s Clock system variable. Don’t worry too much about the
        // syntax here,
        // but it can only work if the System Program is provided.
        let clock: Clock = Clock::get().unwrap();

        // The next line then saves all three variables as properties to the created message account.
        // We dereference the author.key to store it as a Pubkey property in the message.
        message.author = *author.key;
        message.timestamp = clock.unix_timestamp;
        message.content = content;

        // Lastly, Ok(()), of the type ProgramResult, is the output of an instruction.
        Ok(())
    }

    // this is identical to the create_message function. It just uses a difference context type since we’re not
    // initializing a new message, but rather will be pointing to the account containing the current message our
    // Program will be accessing.
    pub fn update_message(ctx: Context<UpdateMessage>, content: String) -> Result<()> {
        let message: &mut Account<Message> = &mut ctx.accounts.message;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        
        message.author = *author.key;
        message.timestamp = clock.unix_timestamp;
        message.content = content;
        
        Ok(())
      }
}

// #[derive(Accounts)] is a custom Rust attribute that defines the context for an instruction as a Rust struct.
// Again, Anchor abstracts a lot away so we can develop faster.

#[derive(Accounts)]
// The message property is the actual account that the instruction will create.
// When we call the instruction, we’re going to pass in a public key to use for this account.

pub struct CreateMessage<'info> {
    // The [account(init, payer = author, space = 1000)] is an account constraint on this property - it tells Rust that
    // message is a Solana account.
    // - The init says that this instruction will create the account, through the System Program.
    // - The payer says that the author property is who will pay to initialize and keep the data on Solana.
    // - The space says the big the account data will be in bytes. For sake of simplicity, we’re saying a message can be
    //   at most 1000 bytes. Normally we would spend time getting efficient about how much space our data will take up,
    //   since it determines how much SOL we have to spend. That is out of the scope of this introductory tutorial,
    //   though 🙂.
    #[account(init, payer = author, space = 1000)]
    pub message: Account<'info, Message>,
    // The author property is the actual sender of the message. It’s a Signer account because we will pass in their
    // signature. It has a mutable account constraint since we will be modifying their balance of SOL when initializing
    // the message account.
    #[account(mut)]
    pub author: Signer<'info>,
    // The system_program is Solana’s System Program that will initialize the message account.
    pub system_program: Program<'info, System>,
    // There’s also the 'info which is a Rust lifetime, or essentially a generic which in this case represents an
    // AccountInfo class - a struct that contains the fields of account, listed here.
    // No need to worry about this deeply yet
}
#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    // For this one, we only need to include message and author.
    // Note that message has a different account constraint - since the instruction using this context will be updating
    // the message, we need to make that account mutable.
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
// You’ll note we want to store the content of the most recent message, the timestamp when it was published, and the
// author.
pub struct Message {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}

// Now that we’ve defined the Message account, we can implement instructions as part our the Solana program.
// We’re going implement two instructions - one to create the first message, and one to update that message.

// Because Solana programs are stateless, we need to provide relevant context for each of those instructions.
// To do so, Anchor gives us a way to easily define a context in Rust.

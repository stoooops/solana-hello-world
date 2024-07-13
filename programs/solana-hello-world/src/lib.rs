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

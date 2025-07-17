use anchor_lang::prelude::*;

declare_id!("44ZNtWtQ1VRK6jXo1nHYij3CWBqDFmCYqeYM9LsnPuPv");

#[program]
pub mod stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

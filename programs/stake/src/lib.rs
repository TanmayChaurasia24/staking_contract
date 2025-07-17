use anchor_lang::prelude::*;

declare_id!("44ZNtWtQ1VRK6jXo1nHYij3CWBqDFmCYqeYM9LsnPuPv");

#[program]
pub mod stake {
    use super::*;

    pub fn initialize_pda(ctx: Context<CreatePdaAccount>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let pda_account: &mut Account<'_, StakeAccount> = &mut ctx.accounts.pda_account;
        let clock: Clock = Clock::get()?;

        pda_account.owner = ctx.accounts.signer.key();
        pda_account.staked_amount = 0;
        pda_account.total_points = 0;
        pda_account.last_update_time = clock.unix_timestamp;
        pda_account.bump = ctx.bumps.pda_account;

        msg!("pda account created successfully!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePdaAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 8 + 8 + 8 + 1,
        seeds = [b"client1", signer.key().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub total_points: u64,
    pub last_update_time: i64,
    pub bump: u8,
}

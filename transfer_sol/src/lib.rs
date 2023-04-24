use anchor_lang::{
    solana_program::{
        system_instruction::,
        program::invoke,
    },
    prelude::*
};

declare_id!("97ico5tgMcM8xyeumNUroM51bKgnjjWgSbVdxqYPJYVb"); // Declare the program ID

#[program] // Declare the program module
pub mod transfer_one_sol {
    use super::*;
    pub fn send_one_sol( // Define the program function
        ctx: Context<Transaction>,
        msg: String
    ) -> Result<()> {
        let transfer = transfer( // Create a transfer instruction using the Solana system program
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            1000000000,
        );
        invokec( // Invoke the transfer instruction using the Solana program interface
            &transfer,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info().clone(),
            ],
        )
        .expect("Error");
        msg!("{}", msg); // Print user message to the console
        Ok(())
    }
}

#[derive(Accounts)] // Declare the program accounts using the `#[derive(Accounts)]` macro
pub struct Transaction<'info> {
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

// // https://github.com/0xDerked/solana-anchor-examples/blob/main/anchor-counter/programs/anchor-counter/src/lib.rs
// use anchor_lang::prelude::*;

// declare_id!("3cTUttpK3K84oG9Ax5WJDiCjPqqdz3KN9qWbXa7mRYbs");

// #[program]
// pub mod anchor_counter {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         let counter = &mut ctx.accounts.counter;
//         counter.count = 0;
//         Ok(())
//     }

//     pub fn increment(ctx: Context<Increment>) -> Result<()> {
//         let counter = &mut ctx.accounts.counter;
//         counter.count += 1;
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init, payer=user, space=8+8, seeds=[b"counter"], bump= )]
//     pub counter: Account<'info, Counter>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct Increment<'info> {
//     #[account(mut, seeds=[b"counter"], bump)]
//     pub counter: Account<'info, Counter>,
//     #[account(mut)]
//     pub user: Signer<'info>,
// }

// #[account]
// pub struct Counter {
//     pub counter: u64,
// }

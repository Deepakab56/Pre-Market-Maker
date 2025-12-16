use anchor_lang::prelude::*;

use crate::state::MarketIds;

#[derive(Accounts)]
pub struct MakerId<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + MarketIds::INIT_SPACE,
        seeds = [b"create_id"],
        bump
    )]
    pub id_account: Account<'info, MarketIds>,
    pub system_program: Program<'info, System>,
}

pub fn process_init(ctx: Context<MakerId>) -> Result<()> {
    let acc = &mut ctx.accounts.id_account;
    acc.create_id = 0;
    acc.order_id = 0;
    Ok(())
}

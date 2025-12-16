use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenAccount, TokenInterface },
};

use crate::state::{ OrderBook, Users };

#[derive(Accounts)]

#[instruction(create_id:u64,order_id:u64,user:Pubkey)]
pub struct SettleOrdered<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds=[
            b"user_details",
            user.key().as_ref(),
            &create_id.to_be_bytes(),
            &order_id.to_be_bytes()
        ],
        bump

    )]
    pub user_account: Account<'info, Users>,
    #[account(
        mut,
        seeds = [
            b"create_order",
            order_account_details.order_creator.key().as_ref(),
            &order_account_details.create_id.to_le_bytes(),
            &order_account_details.order_id.to_le_bytes(),
        ],
        bump
    )]
    pub order_account_details: Account<'info, OrderBook>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = order_account_details,
        associated_token::token_program = token_program
    )]
    pub details_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_settle_order(
    ctx: Context<SettleOrdered>,
    create_id: u64,
    order_id: u64,
    user: Pubkey
) -> Result<()> {
    Ok(())
}

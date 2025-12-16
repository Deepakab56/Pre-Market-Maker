use std::ops::{ Div, Mul };

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked },
};

use crate::{ error::ErrorCodes, state::{ OrderBook, OrderCancelled, TokenDetails } };

#[derive(Accounts)]
#[instruction(create_id: u64, order_id: u64)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token_details", &create_id.to_le_bytes()],
        bump
    )]
    pub token_details_account: Account<'info, TokenDetails>,

    #[account(
        mut,
        seeds = [
            b"create_order",
            &create_id.to_le_bytes(),
            &order_id.to_le_bytes(),
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
        associated_token::authority = token_details_account,
        associated_token::token_program = token_program
    )]
    pub details_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn process_cancel_order(
    ctx: Context<CancelOrder>,
    create_id: u64,
    order_id: u64
) -> Result<()> {
    // let token_details = &mut ctx.accounts.token_details_account;
    let order_details = &mut ctx.accounts.order_account_details;

    if !order_details.is_active {
        return Err(ErrorCodes::OrderInActive.into());
    }

    let remaining_amount = order_details.point - order_details.total_filled_points;
    let remining_collateral = remaining_amount.mul(order_details.point_price).div(10000);
    if remining_collateral > order_details.available_collaleral_amt {
        return Err(ErrorCodes::InsufficentOrder.into());
    }

    order_details.is_active = false;
    order_details.available_collaleral_amt -= remining_collateral;

    let account = TransferChecked {
        from: ctx.accounts.order_account_details.to_account_info(),
        to: ctx.accounts.signer.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.order_account_details.to_account_info(),
    };

    let program = ctx.accounts.token_program.to_account_info();
    let decimals = ctx.accounts.mint.decimals;
    let signer = ctx.accounts.signer.key();
    let seeds: &[&[&[u8]]] = &[
        &[
            b"create_order",
            &create_id.to_le_bytes(),
            &order_id.to_le_bytes(),
            &[ctx.bumps.order_account_details],
        ],
    ];

    let cpi_ctx = CpiContext::new(program, account).with_signer(seeds);
    token_interface::transfer_checked(cpi_ctx, remining_collateral, decimals)?;

    emit!(OrderCancelled {
        create_id,
        order_id,
        maker: signer,
        refund: remining_collateral,
    });

    Ok(())
}

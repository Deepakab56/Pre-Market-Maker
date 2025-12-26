use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked },
};

use crate::{ error::ErrorCodes, state::{ OfferType, OrderBook, OrderFilled, Users } };

#[derive(Accounts)]
#[instruction(create_id: u64, order_id: u64)]
pub struct User<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + Users::INIT_SPACE,
        seeds = [
            b"user_details",
            signer.key().as_ref(),
            &create_id.to_le_bytes(),
            &order_id.to_le_bytes(),
        ],
        bump
    )]
    pub user_account: Account<'info, Users>,

    #[account(
        mut,
        seeds = [
            b"create_order",
            create_id.to_le_bytes().as_ref(),
            order_id.to_le_bytes().as_ref(),
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

pub fn process_user_order(
    ctx: Context<User>,
    amount: u64,
    create_id: u64,
    order_id: u64
) -> Result<()> {
    let order = &mut ctx.accounts.order_account_details;
    let user = &mut ctx.accounts.user_account;

    // ───── Safety Checks ─────
    // require!(order.is_active, ErrorCodes::OrderNotActive);
    require!(order.order_id == order_id, ErrorCodes::InvalidOrderId);

    // require!(
    //     order.total_filled_points + amount <= order.point,
    //     // ErrorCodes::OrderOverFilled
    // );

    let collateral_amount = (((amount as u128) * (order.point_price as u128)) / 10_000) as u64;
    let decimals = ctx.accounts.mint.decimals;

    // ───── Transfer collateral ─────
    let accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.details_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts);

    token_interface::transfer_checked(cpi_ctx, collateral_amount, decimals)?;

    // ───── Update user state ─────
    user.collect_point = amount;
    user.collerateral_amount = collateral_amount;
    user.order_id = order_id;
    user.create_id = create_id;
    user.user_account = ctx.accounts.signer.key();
    user.is_buyer = order.offer_type == OfferType::SELL;

    // ───── Update order state ─────
    order.total_filled_points += amount;
    order.total_filled_collater_amt += collateral_amount;
    order.available_collaleral_amt += collateral_amount;
    order.user_list.push(user.key());

    if order.total_filled_points >= order.point {
        order.is_active = false;
    }

    emit!(OrderFilled {
        create_id,
        order_id,
        user: ctx.accounts.signer.key(),
        to_be_fill_amt: amount,
        collerateral_amt: collateral_amount,
    });

    Ok(())
}

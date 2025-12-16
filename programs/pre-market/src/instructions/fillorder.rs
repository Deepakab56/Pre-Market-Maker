use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked },
};

use crate::{ error::ErrorCodes, state::{ OfferType, OrderBook, OrderFilled, TokenDetails, Users } };

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
            &create_id.to_be_bytes(),
            &order_id.to_be_bytes(),
        ],
        bump
    )]
    pub user_account: Account<'info, Users>,
    // #[account(
    //     mut,
    //     seeds = [b"token_details", signer.key().as_ref(), &create_id.to_le_bytes()],
    //     bump
    // )]
    // pub token_details_account: Account<'info, TokenDetails>,

    #[account(
        mut,
        seeds = [
            b"create_order",
            order_account_details.order_creator.as_ref(),
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

pub fn process_user_order(
    ctx: Context<User>,
    amount: u64,
    create_id: u64,
    order_id: u64
) -> Result<()> {
    let order_details = &mut ctx.accounts.order_account_details;
    let user_details = &mut ctx.accounts.user_account;
    let collateral_amount = (((amount as u128) * (order_details.point_price as u128)) /
        10000) as u64;
    let decimals = ctx.accounts.mint.decimals;

    if order_details.order_id != order_id {
        return Err(ErrorCodes::InvalidOrderId.into());
    }

    let accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.details_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(program, accounts);

    token_interface::transfer_checked(cpi_ctx, collateral_amount, decimals)?;

    user_details.collect_point = amount;
    user_details.collerateral_amount = collateral_amount;
    user_details.order_id = order_id;
    user_details.create_id = create_id;
    user_details.user_account = ctx.accounts.signer.key();
    if order_details.offer_type == OfferType::SELL {
        user_details.is_buyer = true;
    } else {
        user_details.is_buyer = false;
    }

    order_details.total_filled_points += amount;
    order_details.total_filled_collater_amt += amount;
    order_details.available_collaleral_amt += amount;
    order_details.user_list.push(user_details.key());

    if order_details.total_filled_points == order_details.point {
        order_details.is_active = false;
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

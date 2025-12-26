use anchor_lang::{ prelude::* };
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked },
};

use crate::state::{ MarketIds, OfferType, OrderBook, OrderCreated, TokenDetails };

#[derive(Accounts)]
#[instruction(create_id: u64)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"create_id"],
        bump
    )]
    pub id_account: Account<'info, MarketIds>,

    #[account(
        mut,
        seeds = [
            b"token_details".as_ref(),   
            &create_id.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub token_details_account: Account<'info, TokenDetails>,
    #[account(
        init,
        payer = signer,
        space = 8 + OrderBook::INIT_SPACE,
        seeds = [
            b"create_order".as_ref(),
            create_id.to_le_bytes().as_ref(),
            id_account.order_id.to_be_bytes().as_ref(),
        ],
        bump
    )]
    pub order_account_details: Account<'info, OrderBook>,

    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = order_account_details,
        associated_token::token_program = token_program
    )]
    pub details_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_order(
    ctx: Context<CreateOrder>,
    point_amount: u64,
    point_price: u64,
    create_id: u64,
    is_partial: bool
) -> Result<()> {
    let decimals = ctx.accounts.mint.decimals;
    let ids = &mut ctx.accounts.id_account;

    let order_id = ids.order_id;
    let signer = ctx.accounts.signer.key();

    let collateral_amount = (((point_amount as u128) * (point_price as u128)) / 10000) as u64;

    let accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.details_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(program, accounts);

    // FIX: Using token_interface
    token_interface::transfer_checked(cpi_ctx, collateral_amount, decimals)?;

    let acc = &mut ctx.accounts.order_account_details;
    acc.order_id = order_id;
    acc.collateral = ctx.accounts.mint.key(); // FIXED
    acc.is_partial = is_partial;
    acc.offer_type = OfferType::SELL;
    acc.collateral_amt = collateral_amount;
    acc.order_creator = signer;
    acc.create_id = create_id;
    ids.order_id += order_id;
    ctx.accounts.token_details_account.order_list.push(ctx.accounts.order_account_details.key());
    let time_stamp = Clock::get()?.unix_timestamp;
    emit!(OrderCreated {
        create_id,
        order_id,
        user: signer,
        amount: point_amount,
        price: point_price,
        is_partial,
        is_buy: true,
        timestamp: time_stamp as u64,
    });

    Ok(())
}

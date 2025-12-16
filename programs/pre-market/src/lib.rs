use anchor_lang::prelude::*;
use instructions::*;
mod state;
mod error;
mod constants;
mod instructions;

declare_id!("5v8sL5hRV9StiPw8kDrLV2PMXKPz5sp84axEVecAH9Rt");

#[program]
pub mod pre_market {
    use super::*;

    pub fn initialize(ctx: Context<MakerId>) -> Result<()> {
        process_init(ctx)
    }
    pub fn init_token_details(
        ctx: Context<CreateToken>,
        token_name: String,
        token_symbol: String
    ) -> Result<()> {
        process_token(ctx, token_name, token_symbol)
    }

    pub fn init_full_fill_order(
        ctx: Context<CreateToken>,
        token_id: Pubkey,
        token_price: u64,
        end_time: u64
    ) -> Result<()> {
        process_full_fill_order(ctx, token_id, token_price, end_time)
    }

    pub fn init_order(
        ctx: Context<CreateOrder>,
        point_amount: u64,
        point_price: u64,
        is_partial: bool,
        create_id: u64
    ) -> Result<()> {
        process_order(ctx, point_amount, point_price, create_id, is_partial)
    }

    pub fn init_user_order(
        ctx: Context<User>,
        amount: u64,
        create_id: u64,
        order_id: u64
    ) -> Result<()> {
        process_user_order(ctx, amount, create_id, order_id)
    }

    pub fn init_cancel_order(
        ctx: Context<CancelOrder>,
        create_id: u64,
        order_id: u64
    ) -> Result<()> {
        process_cancel_order(ctx, create_id, order_id)
    }

    pub fn init_settle_order(
        ctx: Context<SettleOrdered>,
        create_id: u64,
        order_id: u64,
        user: Pubkey
    ) -> Result<()> {
        process_settle_order(ctx, create_id, order_id, user)
    }
}

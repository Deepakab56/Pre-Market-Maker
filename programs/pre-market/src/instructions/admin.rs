use anchor_lang::{ prelude::* };

use crate::{
    error::ErrorCodes,
    state::{ MarketIds, TokenDetails, TokenDetailsCreated, TokenFulfillTimeSet, TokenStage },
};

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"create_id"],
        bump
    )]
    pub id_account: Account<'info, MarketIds>,

    #[account(
        init,
        payer = signer,
        space = 8 +32+ TokenDetails::INIT_SPACE,
        seeds = [b"token_details", signer.key().as_ref(), &id_account.create_id.to_le_bytes()],
        bump
    )]
    pub token_details_account: Account<'info, TokenDetails>,

    pub system_program: Program<'info, System>,
}

pub fn process_token(
    ctx: Context<CreateToken>,
    token_name: String,
    token_symbol: String
) -> Result<()> {
    let acct = &mut ctx.accounts.token_details_account;
    let ids = &mut ctx.accounts.id_account;
    let signer = ctx.accounts.signer.key();

    if acct.is_trading {
        return Err(ErrorCodes::AlreadyExists.into());
    }

    // Save creator id before incrementing
    let creator_id_value = ids.create_id;

    // Fill token details
    acct.token_name = token_name.clone();
    acct.token_symbol = token_symbol.clone();
    acct.token_address = Pubkey::default();
    acct.creator = signer;
    acct.creator_id = creator_id_value;
    acct.total_val = 0;
    acct.start_settle_time = 0;
    acct.end_settle_time = 0;
    acct.is_trading = false;
    acct.stage = TokenStage::Trading;

    // Increment create_id
    ids.create_id += 1;

    // Emit event
    emit!(TokenDetailsCreated {
        token_name,
        token_symbol,
        creator_id: creator_id_value,
    });

    Ok(())
}

pub fn process_full_fill_order(
    ctx: Context<CreateToken>,
    token_id: Pubkey,
    token_price: u64,
    end_time: u64
) -> Result<()> {
    let acc = &mut ctx.accounts.token_details_account;

    // Token must be registered
    if !acc.is_trading {
        return Err(ErrorCodes::NotRegisterTokens.into());
    }

    // Validate token address
    if token_id == Pubkey::default() {
        return Err(ErrorCodes::InvalidTokenAddress.into());
    }

    // Validate end time
    let current_time = Clock::get()?.unix_timestamp as u64;
    if end_time <= current_time {
        return Err(ErrorCodes::InvalidEndTime.into());
    }

    // Validate price
    if token_price == 0 {
        return Err(ErrorCodes::InvalidPrice.into());
    }

    // Correct: creator_id is a u64
    let order_id = acc.creator_id;

    // Update token details
    acc.stage = TokenStage::FullFilled;
    acc.end_settle_time = end_time;
    acc.total_val = token_price;
    acc.token_address = token_id;

    // Emit event
    emit!(TokenFulfillTimeSet {
        order_id,
        token_id,
        token_price,
        end_time,
    });

    Ok(())
}

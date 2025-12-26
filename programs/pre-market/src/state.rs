use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketIds {
    pub create_id: u64,
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub enum TokenStage {
    Trading,
    FullFilled,
    CANCELED,
    Ended,
}

pub const MAX_NAME_LEN: usize = 32;
pub const MAX_SYMBOL_LEN: usize = 10;

#[account]
#[derive(InitSpace)]
pub struct TokenDetails {
    #[max_len(10)]
    pub token_name: String,
    #[max_len(10)]
    pub token_symbol: String,
    pub token_address: Pubkey,
    pub creator_id: u64,
    pub creator: Pubkey,
    pub total_val: u64,
    pub start_settle_time: u64,
    pub end_settle_time: u64,
    pub is_trading: bool,
    pub stage: TokenStage,
    #[max_len(500)]
    pub order_list: Vec<Pubkey>,
}

impl TokenDetails {
    pub const INIT_SPACE: usize =
        4 +
        MAX_NAME_LEN + // token_name
        4 +
        MAX_SYMBOL_LEN + // token_symbol
        32 + // token_address
        32 + // creator
        8 + // creator_id
        8 + // total_val
        8 + // start_settle_time
        8 + // end_settle_time
        1 + // is_trading
        1; // stage
}

#[derive(Debug, PartialEq, Eq, Clone, AnchorDeserialize, AnchorSerialize, InitSpace)]
pub enum OfferType {
    SELL,
    BUY,
}

#[account]
#[derive(InitSpace)]
pub struct OrderBook {
    pub order_id: u64,
    pub create_id: u64,
    pub offer_type: OfferType,
    pub user: Pubkey,
    pub collateral: Pubkey,
    pub order_creator: Pubkey,
    pub point: u64,
    pub point_price: u64,
    pub collateral_amt: u64,
    pub total_filled_collater_amt: u64,
    pub total_filled_points: u64,
    pub available_collaleral_amt: u64,
    pub available_filled_collateral_amt: u64,
    pub is_partial: bool,
    pub is_active: bool,
    pub settled: bool,
    #[max_len(500)]
    pub user_list: Vec<Pubkey>,
}

#[event]
pub struct TokenDetailsCreated {
    pub token_name: String,
    pub token_symbol: String,
    pub creator_id: u64,
}

#[event]
pub struct OrderCreated {
    pub create_id: u64,
    pub order_id: u64,
    pub user: Pubkey,
    pub amount: u64,
    pub price: u64,
    pub is_partial: bool,
    pub is_buy: bool,
    pub timestamp: u64,
}

#[event]
pub struct TokenFulfillTimeSet {
    pub order_id: u64,
    pub token_id: Pubkey,
    pub token_price: u64,
    pub end_time: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Users {
    pub user_account: Pubkey,
    pub collect_point: u64,
    pub collerateral_amount: u64,
    pub order_id: u64,
    pub create_id: u64,
    pub is_buyer: bool,
}

#[event]
pub struct OrderFilled {
    pub create_id: u64,
    pub order_id: u64,
    pub user: Pubkey,
    pub to_be_fill_amt: u64,
    pub collerateral_amt: u64,
}

#[event]
pub struct OrderCancelled {
    pub create_id: u64,
    pub order_id: u64,
    pub maker: Pubkey,
    pub refund: u64,
}

use anchor_lang::prelude::*;
use anchor_lang::system_program;
declare_id!("7DpC8YGUFh2yWSZvHuoTmAa4koBFngZnMKibuACuujw4");

#[program]
pub mod nft_sales {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        token_id: u64,
        metadata_uri: String,
    ) -> Result<()> {
        let nft = &mut ctx.accounts.nft;

        // 验证 metadata_uri 长度
        require!(
            metadata_uri.len() <= NFTState::MAX_METADATA_URI_LEN,
            ErrorCode::MetadataUriTooLong
        );

        // 初始化 NFTState
        nft.token_id = token_id;
        nft.owner = ctx.accounts.minter.key();
        nft.metadata_uri = metadata_uri;

        Ok(())
    }

    pub fn list_nft(ctx: Context<ListNFT>, price: u64) -> Result<()> {
        let nft = &ctx.accounts.nft;
        let sale = &mut ctx.accounts.sale;
        let seller = &ctx.accounts.seller;

        // 验证卖家是 NFT 所有者
        require!(nft.owner == seller.key(), ErrorCode::NotOwner);

        // 初始化 SaleState
        sale.nft_account = nft.key();
        sale.seller = seller.key();
        sale.price = price;
        sale.active = true;

        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()> {
        let sale = &mut ctx.accounts.sale;
        let nft = &mut ctx.accounts.nft;
        let buyer = &ctx.accounts.buyer;
        let seller = &ctx.accounts.seller;

        // 验证销售活跃
        require!(sale.active, ErrorCode::SaleInactive);
        // 验证 NFT 匹配
        require!(sale.nft_account == nft.key(), ErrorCode::InvalidNFT);

        // 转账 SOL
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, sale.price)?;

        // 更新 NFT 所有权
        nft.owner = buyer.key();
        // 关闭销售
        sale.active = false;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(token_id: u64)]
pub struct MintNFT<'info> {
    #[account(
        init,
        payer = minter,
        space = NFTState::LEN,
        seeds = [b"nft", token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub nft: Account<'info, NFTState>,
    #[account(mut)]
    pub minter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account()]
    pub nft: Account<'info, NFTState>,
    #[account(
        init,
        payer = seller,
        space = SaleState::LEN,
        seeds = [b"sale", nft.key().as_ref()],
        bump
    )]
    pub sale: Account<'info, SaleState>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyNFT<'info> {
    #[account(mut)]
    pub sale: Account<'info, SaleState>,
    #[account(mut)]
    pub nft: Account<'info, NFTState>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    /// CHECK:safe
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NFTState {
    pub token_id: u64,
    pub owner: Pubkey,
    pub metadata_uri: String,
}

impl NFTState {
    pub const MAX_METADATA_URI_LEN: usize = 100;
    pub const LEN: usize = 8 + 8 + 32 + 4 + Self::MAX_METADATA_URI_LEN;
}

#[account]
pub struct SaleState {
    pub nft_account: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub active: bool,
}

impl SaleState {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The sale is not active")]
    SaleInactive,
    #[msg("Invalid NFT account for this sale")]
    InvalidNFT,
    #[msg("Seller is not the owner of the NFT")]
    NotOwner,
    #[msg("Metadata URI exceeds maximum length")]
    MetadataUriTooLong,
}
use anchor_lang::prelude::*;

// TODO: Update this after deployment
// this is my test net id update it with your company public id before deployment or for running in you test net s
declare_id!("B6UasArGY3jpGk9wXif5yvgmgB24NMrms9RCGDhn4v7r");

// Replace with your company public id
pub const AUTHORIZED_COMPANY: &str = "YOUR_COMPANY_PUBLIC_KEY_HERE";

#[program]
pub mod brand_validator_on_chain {
    use super::*;

    
    pub fn add_product(ctx: Context<AddProduct>, product_id: String) -> Result<()> {
        // authorise company   
        require!(
            ctx.accounts.owner.key().to_string() == AUTHORIZED_COMPANY,
            ErrorCode::UnauthorizedCompany
        );
        
        require!(product_id.len() <= 500, ErrorCode::StringTooLong);
        
        let product_account = &mut ctx.accounts.product_account;
        
        product_account.product_id = product_id;
        product_account.is_buyed = false;
        product_account.owner = ctx.accounts.owner.key();
        product_account.created_at = Clock::get()?.unix_timestamp;
        product_account.updated_at = Clock::get()?.unix_timestamp;
        
        msg!("Product added successfully");
        Ok(())
    }


    pub fn validate_product(ctx: Context<ValidateProduct>) -> Result<(String, bool)> {
        let product_account = &ctx.accounts.product_account;
        Ok((product_account.product_id.clone(), product_account.is_buyed))
    }


    pub fn buy_product(ctx: Context<BuyProduct>, product_id: String) -> Result<()> {
        let product_account = &mut ctx.accounts.product_account;
        require!(product_account.product_id == product_id, ErrorCode::InvalidProductId);        
        require!(!product_account.is_buyed, ErrorCode::ProductAlreadyBought);
        
        product_account.is_buyed = true;
        product_account.updated_at = Clock::get()?.unix_timestamp;
        
        msg!("Product bought successfully");
        Ok(())
    }

    
    pub fn delete_product(ctx: Context<DeleteProduct>) -> Result<()> {
        // Verify company authorization
        require!(
            ctx.accounts.owner.key().to_string() == AUTHORIZED_COMPANY,
            ErrorCode::UnauthorizedCompany
        );
        
        require!(ctx.accounts.product_account.owner == ctx.accounts.owner.key(), ErrorCode::Unauthorized); 
        msg!("Product deleted successfully");
        Ok(())
    }
}

// Create new product account
#[derive(Accounts)]
#[instruction(product_id: String)]
pub struct AddProduct<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 4 + 500 + 1 + 32 + 8 + 8, // Account size
        seeds = [b"product", owner.key().as_ref()],
        bump
    )]
    pub product_account: Account<'info, ProductAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Read product data
#[derive(Accounts)]
pub struct ValidateProduct<'info> {
    pub product_account: Account<'info, ProductAccount>,
}

// Update product status
#[derive(Accounts)]
pub struct BuyProduct<'info> {
    #[account(mut)]
    pub product_account: Account<'info, ProductAccount>,
    
    pub buyer: Signer<'info>,
}

// Delete product account
#[derive(Accounts)]
pub struct DeleteProduct<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [b"product", owner.key().as_ref()],
        bump
    )]
    pub product_account: Account<'info, ProductAccount>,
    
    pub owner: Signer<'info>,
}

// Product structure
#[account]
pub struct ProductAccount {
    pub product_id: String,      
    pub is_buyed: bool,          
    pub owner: Pubkey,           
    pub created_at: i64,         
    pub updated_at: i64,         
}

// Exceptions
#[error_code]
pub enum ErrorCode {
    #[msg("Product ID exceeds maximum length of 500 characters")]
    StringTooLong,
    
    #[msg("Only the owner can perform this action")]
    Unauthorized,
    
    #[msg("Invalid product ID provided")]
    InvalidProductId,
    
    #[msg("Product has already been bought")]
    ProductAlreadyBought,

    #[msg("Only authorized company can add products")]
    UnauthorizedCompany,
}

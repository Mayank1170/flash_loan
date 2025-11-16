use anchor_lang::prelude::*;
use anchor_spl::{
    token:: {Token, TokenAccount, Mint, Transfer, transfer}.
    asociated_token:: AssociatedToken
};

use anchor_lang::{
    Descriminator,
    solana_program::sysvar::instruction::{
        ID as INSTRUCTION_SYSVAR_ID,
        load_instruction_at_checked
    }
}


declare_id!("3gJyGpTNRDTJN4f1eMNDZ8VdvRhnXGFwW2ZZB4jnGNUP");

#[program]
pub mod anchor_flash_loan {
    use super::*;

    pub fn borrow(ctx: Context<Loan>, borrow_amount: u64) -> Result<()> {

        Ok(())
    }
    pub fn repay(ctx:Context<Loan>, borrow_amount:u64) -> Result<()>{
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Loan<'info> {}

#[error_code]
pub enum ProtocolError{
    
}
use anchor_lang::prelude::*;
use anchor_spl::{
    token:: {Token, TokenAccount, Mint, Transfer, transfer},
    associated_token:: AssociatedToken
};

use anchor_lang::{
    Descriminator,
    solana_program::sysvar::instruction::{
        ID as INSTRUCTION_SYSVAR_ID,
        load_instruction_at_checked
    }
};


declare_id!("3gJyGpTNRDTJN4f1eMNDZ8VdvRhnXGFwW2ZZB4jnGNUP");

#[program]
pub mod sountsnchor_flash_loan {
    use super::*;

    pub fn borrow(ctx: Context<Loan>, borrow_amount: u64) -> Result<()> {

        Ok(())
    }
    pub fn repay(ctx:Context<Loan>, borrow_amount:u64) -> Result<()>{
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Loan<'info> {
    #[account(&mut)]
    pub borrower: Signer<'info>,

    #[account(
        seeds = [b"protocol".as_ref()],
        bump,
    )]
    pub protocol: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = borrower,
        associated_token::mint = mint, 
        associated_token::authority = borrower,
    )]
    pub borrower_ata: Accounts<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::authority = protocol,
        associated_token::mint = mint,
    )]
    pub protocol_ata : Accounts<'info, TokenAccount>,

    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    instructions: UncheckedAccount<'info>,
    pub token_program: Program<'info ,Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>

}

#[error_code]
pub enum ProtocolError{
        #[msg("Invalid instruction")]
        InvalidIx,
        #[msg("Invalid instruction index")]
        InvalidInstructionIndex,
        #[msg("Invalid amount")]
        InvalidAmount,
        #[msg("Not enough funds")]
        NotEnoughFunds,
        #[msg("Program Mismatch")]
        ProgramMismatch,
        #[msg("Invalid program")]
        InvalidProgram,
        #[msg("Invalid borrower ATA")]
        InvalidBorrowerAta,
        #[msg("Invalid protocol ATA")]
        InvalidProtocolAta,
        #[msg("Missing repay instruction")]
        MissingRepayIx,
        #[msg("Missing borrow instruction")]
        MissingBorrowIx,
        #[msg("Overflow")]
        Overflow,
}

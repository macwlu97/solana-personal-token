use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer, Token};

declare_id!("YourProgramIDHere"); // Wstaw ID swojego programu po deployu.

#[program]
pub mod personal_token {
    use super::*;

    // Funkcja inicjalizacji programu
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Program initialized!");
        Ok(())
    }

    // Funkcja transferu z podatkiem
    pub fn transfer_with_tax(ctx: Context<TransferWithTax>, amount: u64) -> Result<()> {
        let tax = amount / 10; // 10% podatku
        let net_amount = amount - tax;

        // Transfer główny
        token::transfer(ctx.accounts.into_transfer_context(), net_amount)?;

        // Transfer podatku
        token::transfer(ctx.accounts.into_tax_context1(), tax / 2)?; // 50% na address1
        token::transfer(ctx.accounts.into_tax_context2(), tax / 4)?; // 25% na address2
        token::transfer(ctx.accounts.into_tax_context3(), tax / 4)?; // 25% na address3

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct TransferWithTax<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,          // Token Sender
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>, // Main recipient
    #[account(mut)]
    pub tax_account1: Account<'info, TokenAccount>, // Tax: address1
    #[account(mut)]
    pub tax_account2: Account<'info, TokenAccount>, // Tax: address2
    #[account(mut)]
    pub tax_account3: Account<'info, TokenAccount>, // Tax: address3
    pub token_program: Program<'info, Token>,
}

impl<'info> TransferWithTax<'info> {
    pub fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sender.to_account_info().clone(),
            to: self.recipient.to_account_info().clone(),
            authority: self.sender.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn into_tax_context1(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sender.to_account_info().clone(),
            to: self.tax_account1.to_account_info().clone(),
            authority: self.sender.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn into_tax_context2(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sender.to_account_info().clone(),
            to: self.tax_account2.to_account_info().clone(),
            authority: self.sender.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn into_tax_context3(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sender.to_account_info().clone(),
            to: self.tax_account3.to_account_info().clone(),
            authority: self.sender.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}


// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod personal_token {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

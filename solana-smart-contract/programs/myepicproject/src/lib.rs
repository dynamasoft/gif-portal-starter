use anchor_lang::prelude::*;

declare_id!("G98uKPuwZ2FpxBxzw45Jes8H7qs38b1TRvTrUwa94DC8");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> Result<()> {
        // Get a reference to the account.
        let base_account = &mut _ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    // Another function woo!
    pub fn add_gif(ctx: Context<AddGif>, gif_link:String) -> Result<()> {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct
        {
            gif_link:gif_link.to_string(),
            user_address: *user.to_account_info().key,            
        };

        base_account.gif_list.push(item);

        base_account.total_gifs += 1;
        Ok(())
    }
}

    #[derive(Accounts)]
    pub struct Initialize {}

    #[derive(Accounts)]
    pub struct StartStuffOff<'info> {
        #[account(init, payer = user, space = 9000)]
        pub base_account: Account<'info, BaseAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
    pub struct ItemStruct{
        pub gif_link:String,
        pub user_address: Pubkey
    }

    // Tell Solana what we want to store on this account.
    #[account]
    pub struct BaseAccount {
        pub total_gifs: u64,
        pub gif_list: Vec<ItemStruct>
    }


    #[derive(Accounts)]
    pub struct AddGif<'info> {
        #[account(mut)]
        pub base_account: Account<'info, BaseAccount>,
        #[account(mut)]
        pub user: Signer<'info>
    }


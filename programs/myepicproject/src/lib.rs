use anchor_lang::prelude::*;

declare_id!("GpaBCyhm5yA8H9cvSDm81N8FCkGzBbruNb4DppYRaeT2");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {

    // Note: We do &mut to get a "mutable reference" to base_account. When we do this it actually gives us the power to make changes to base_account. Otherwise, we'd simply be working w/ a "local copy" of base_account
    let base_account = &mut ctx.accounts.base_account;

    base_account.total_gifs = 0;
    Ok(())
  }

  // add gif function. All I do is grab the base_account which was passed in to the function via Context<AddGif>. Then, I increment the counter and that's it!!
  // the function now accepts a gif_link param from the user. We also reference the user from the Context.
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
      // Get a reference to the account and increment total_gifs.
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;

      // Build the struct.
      let item = ItemStruct {
          gif_link: gif_link.to_string(),
          user_address: *user.to_account_info().key,
      };

      // add it to the gif_list vector.
      base_account.gif_list.push(item);
      base_account.total_gifs += 1;
      Ok(())
  }
}



// Attach certain variables to the StartStuffOff context. First we've got [account(init, payer = user, space = 9000)]. All we're doing here is telling Solana how we want to initialize BaseAccount.
// We then have pub user: Signer<'info> which is data passed into the program that proves to the program that the user calling this program actually owns their wallet account.
// Finally, we have pub system_program: Program which is actually pretty freaking cool. It's basically a reference to the SystemProgram. The SystemProgram is the program that basically runs Solana. It is responsible for a lot of stuff, but one of the main things it does is create accounts on Solana. The SystemProgram is a program the creators of Solana deployed that other programs like ours talk to haha — it has an id of 11111111111111111111111111111111.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space=9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.  create a Context named AddGif that has access to a mutable reference to base_account. That's why I do #[account(mut)]. Basically it means I can actually change the total_gifs value stored on BaseAccount.
// add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with. tells Anchor how to serialize/deserialize the struct. Remember, data is being stored in an "account" right? That account is basically a file and we serialize our data into binary format before storing it. Then, when we want to retrieve it we'll actually deserialize it.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}


// tell solana what we want to store on this account. Basically, it tells our program what kinda of account it can make and what to hold inside of it. So, here, BaseAccount holds one thing and it's an integer named total_gifs.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}

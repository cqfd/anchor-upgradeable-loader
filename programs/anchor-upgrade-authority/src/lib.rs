use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("A6ASDXsvLnewMsTo3H1pXR1SMg3sZLLu1MrgLJVVpFtm");

#[program]
pub mod anchor_upgrade_authority {

    use anchor_lang::solana_program::bpf_loader_upgradeable;

    use super::*;
    pub fn read_upgrade_authority(ctx: Context<ReadUpgradeAuthority>) -> ProgramResult {
        let upgradeable_loader_state = ctx
            .accounts
            .program_account
            .deserialize_data::<UpgradeableLoaderState>();
        msg!("upgradeable_loader_state = {:?}", upgradeable_loader_state);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadUpgradeAuthority<'info> {
    #[account(
        constraint = {
            let (expected_address, _) = Pubkey::find_program_address(&[ID.as_ref()], &Pubkey::from_str("BPFLoaderUpgradeab1e11111111111111111111111").unwrap());
            program_account.key == &expected_address
        }
    )]
    pub program_account: AccountInfo<'info>,
}

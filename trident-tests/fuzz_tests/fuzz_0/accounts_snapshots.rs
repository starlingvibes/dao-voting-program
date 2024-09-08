use anchor_lang::prelude::*;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct CreateTeamSnapshot<'info> {
    pub team_account: Option<Account<'info, dao_voting_program::team::TeamAccount>>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct AddMemberSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct RemoveMemberSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct TransferCaptainSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct LeaveTeamSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct InitTournamentSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct VoteForTournamentSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct ViewVoteResultSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct LeaveTournamentSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct InitPercentageProposalSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct DistributionProposalHandlerSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct CanJoinTournamentSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct ClaimRewardSnapshot<'info> {
    pub team_account: Account<'info, dao_voting_program::team::TeamAccount>,
    pub from: &'info AccountInfo<'info>,
    pub to: &'info AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> CreateTeamSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: Option<
            anchor_lang::accounts::account::Account<dao_voting_program::team::TeamAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("team_account".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "team_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> AddMemberSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> RemoveMemberSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> TransferCaptainSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> LeaveTeamSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> InitTournamentSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> VoteForTournamentSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> ViewVoteResultSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> LeaveTournamentSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> InitPercentageProposalSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> DistributionProposalHandlerSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> CanJoinTournamentSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            signer,
            system_program,
        })
    }
}
impl<'info> ClaimRewardSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let team_account: anchor_lang::accounts::account::Account<
            dao_voting_program::team::TeamAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("team_account".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("team_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("team_account".to_string()))?;
        let from = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("from".to_string()))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("from".to_string()))?;
        let to = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("to".to_string()))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("to".to_string()))?;
        let user: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("user".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            team_account,
            from,
            to,
            user,
            system_program,
        })
    }
}

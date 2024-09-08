pub mod dao_voting_program_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        CreateTeam(CreateTeam),
        AddMember(AddMember),
        RemoveMember(RemoveMember),
        TransferCaptain(TransferCaptain),
        LeaveTeam(LeaveTeam),
        InitTournament(InitTournament),
        VoteForTournament(VoteForTournament),
        ViewVoteResult(ViewVoteResult),
        LeaveTournament(LeaveTournament),
        InitPercentageProposal(InitPercentageProposal),
        DistributionProposalHandler(DistributionProposalHandler),
        CanJoinTournament(CanJoinTournament),
        ClaimReward(ClaimReward),
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateTeam {
        pub accounts: CreateTeamAccounts,
        pub data: CreateTeamData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateTeamAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateTeamData {
        pub team_name: String,
        pub team_id: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct AddMember {
        pub accounts: AddMemberAccounts,
        pub data: AddMemberData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct AddMemberAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct AddMemberData {
        pub _team_name: String,
        pub _team_id: u64,
        pub member: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RemoveMember {
        pub accounts: RemoveMemberAccounts,
        pub data: RemoveMemberData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RemoveMemberAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RemoveMemberData {
        pub _team_name: String,
        pub _team_id: u64,
        pub member: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferCaptain {
        pub accounts: TransferCaptainAccounts,
        pub data: TransferCaptainData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferCaptainAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferCaptainData {
        pub _team_name: String,
        pub _team_id: u64,
        pub member: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct LeaveTeam {
        pub accounts: LeaveTeamAccounts,
        pub data: LeaveTeamData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct LeaveTeamAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct LeaveTeamData {
        pub _team_name: String,
        pub _team_id: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitTournament {
        pub accounts: InitTournamentAccounts,
        pub data: InitTournamentData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitTournamentAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitTournamentData {
        pub _team_name: String,
        pub _team_id: u64,
        pub tournament_address: AccountId,
        pub tournament_prize: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VoteForTournament {
        pub accounts: VoteForTournamentAccounts,
        pub data: VoteForTournamentData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VoteForTournamentAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VoteForTournamentData {
        pub _team_name: String,
        pub _team_id: u64,
        pub vote_type: dao_voting_program::team::VoteType,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ViewVoteResult {
        pub accounts: ViewVoteResultAccounts,
        pub data: ViewVoteResultData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ViewVoteResultAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ViewVoteResultData {
        pub _team_name: String,
        pub _team_id: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct LeaveTournament {
        pub accounts: LeaveTournamentAccounts,
        pub data: LeaveTournamentData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct LeaveTournamentAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct LeaveTournamentData {
        pub _team_name: String,
        pub _team_id: u64,
        pub vote_type: dao_voting_program::team::VoteType,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitPercentageProposal {
        pub accounts: InitPercentageProposalAccounts,
        pub data: InitPercentageProposalData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitPercentageProposalAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitPercentageProposalData {
        pub _team_name: String,
        pub _team_id: u64,
        pub percentages: Vec<u8>,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DistributionProposalHandler {
        pub accounts: DistributionProposalHandlerAccounts,
        pub data: DistributionProposalHandlerData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DistributionProposalHandlerAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DistributionProposalHandlerData {
        pub _team_name: String,
        pub _team_id: u64,
        pub vote_type: dao_voting_program::team::VoteType,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CanJoinTournament {
        pub accounts: CanJoinTournamentAccounts,
        pub data: CanJoinTournamentData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CanJoinTournamentAccounts {
        pub team_account: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CanJoinTournamentData {
        pub _team_name: String,
        pub _team_id: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ClaimReward {
        pub accounts: ClaimRewardAccounts,
        pub data: ClaimRewardData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ClaimRewardAccounts {
        pub team_account: AccountId,
        pub from: AccountId,
        pub to: AccountId,
        pub user: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ClaimRewardData {
        pub _team_name: String,
        pub _team_id: u64,
        pub reward: u64,
    }
    impl<'info> IxOps<'info> for CreateTeam {
        type IxData = dao_voting_program::instruction::CreateTeam;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CreateTeamSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::CreateTeam {
                team_name: todo!(),
                team_id: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::CreateTeam {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for AddMember {
        type IxData = dao_voting_program::instruction::AddMember;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = AddMemberSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::AddMember {
                _team_name: todo!(),
                _team_id: todo!(),
                member: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::AddMember {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for RemoveMember {
        type IxData = dao_voting_program::instruction::RemoveMember;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = RemoveMemberSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::RemoveMember {
                _team_name: todo!(),
                _team_id: todo!(),
                member: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::RemoveMember {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for TransferCaptain {
        type IxData = dao_voting_program::instruction::TransferCaptain;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = TransferCaptainSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::TransferCaptain {
                _team_name: todo!(),
                _team_id: todo!(),
                member: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::TransferCaptain {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for LeaveTeam {
        type IxData = dao_voting_program::instruction::LeaveTeam;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = LeaveTeamSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::LeaveTeam {
                _team_name: todo!(),
                _team_id: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::LeaveTeam {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitTournament {
        type IxData = dao_voting_program::instruction::InitTournament;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitTournamentSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::InitTournament {
                _team_name: todo!(),
                _team_id: todo!(),
                tournament_address: todo!(),
                tournament_prize: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::InitTournament {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for VoteForTournament {
        type IxData = dao_voting_program::instruction::VoteForTournament;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = VoteForTournamentSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::VoteForTournament {
                _team_name: todo!(),
                _team_id: todo!(),
                vote_type: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::VoteForTournament {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ViewVoteResult {
        type IxData = dao_voting_program::instruction::ViewVoteResult;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ViewVoteResultSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::ViewVoteResult {
                _team_name: todo!(),
                _team_id: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::ViewVoteResults {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for LeaveTournament {
        type IxData = dao_voting_program::instruction::LeaveTournament;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = LeaveTournamentSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::LeaveTournament {
                _team_name: todo!(),
                _team_id: todo!(),
                vote_type: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::LeaveTournament {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitPercentageProposal {
        type IxData = dao_voting_program::instruction::InitPercentageProposal;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitPercentageProposalSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::InitPercentageProposal {
                _team_name: todo!(),
                _team_id: todo!(),
                percentages: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::InitPercentageProposal {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for DistributionProposalHandler {
        type IxData = dao_voting_program::instruction::DistributionProposalHandler;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = DistributionProposalHandlerSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::DistributionProposalHandler {
                _team_name: todo!(),
                _team_id: todo!(),
                vote_type: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::DistributionProposalHandler {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CanJoinTournament {
        type IxData = dao_voting_program::instruction::CanJoinTournament;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CanJoinTournamentSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::CanJoinTournament {
                _team_name: todo!(),
                _team_id: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::CanJoinTournament {
                team_account: todo!(),
                signer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ClaimReward {
        type IxData = dao_voting_program::instruction::ClaimReward;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ClaimRewardSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dao_voting_program::instruction::ClaimReward {
                _team_name: todo!(),
                _team_id: todo!(),
                reward: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = dao_voting_program::accounts::ClaimReward {
                team_account: todo!(),
                from: todo!(),
                to: todo!(),
                user: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        from: AccountsStorage<todo!()>,
        signer: AccountsStorage<todo!()>,
        system_program: AccountsStorage<todo!()>,
        team_account: AccountsStorage<todo!()>,
        to: AccountsStorage<todo!()>,
        user: AccountsStorage<todo!()>,
    }
}

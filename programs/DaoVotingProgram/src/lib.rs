use anchor_lang::prelude::*;
use team::*;

pub mod team;

declare_id!("EyVtWh28nawY7NDLCBoFWNC5S56s198tftxDxGEgFrFM");

#[program]
pub mod dao_voting_program {

    use super::*;

    use team::adding_member::{adding_member, AddMember};
    use team::can_join::{can_join, CanJoinTournament};
    use team::claiming_reward::{claiming_reward, ClaimReward};
    use team::creating_team::{creating_team, CreateTeam};
    use team::handle_distribute_proposal::{
        handle_distribute_proposal, DistributionProposalHandler,
    };
    use team::initiating_percentage_proposal::{
        initiating_percentage_proposal, InitPercentageProposal,
    };
    use team::initiating_tournament::{initiating_tournament, InitTournament};
    use team::leaving_team::{leaving_team, LeaveTeam};
    use team::leaving_tournament::{leaving_tournament, LeaveTournament};
    use team::removing_member::{removing_member, RemoveMember};
    use team::transfering_captain::{transfering_captain, TransferCaptain};
    use team::view_vote_results::{view_vote_results, ViewVoteResults};
    use team::voting_for_tournament::{voting_for_tournament, VoteForTournament};
    use team::VoteType;

    // ----------------------------------------------
    // ADMINISTRATIVE instructions
    // instructions that can be called by the captain

    pub fn create_team(ctx: Context<CreateTeam>, team_name: String, team_id: u64) -> Result<()> {
        return creating_team(ctx, team_name, team_id);
    }

    pub fn add_member(
        ctx: Context<AddMember>,
        _team_name: String,
        _team_id: u64,
        member: Pubkey,
    ) -> Result<()> {
        return adding_member(ctx, _team_name, _team_id, member);
    }

    pub fn remove_member(
        ctx: Context<RemoveMember>,
        _team_name: String,
        _team_id: u64,
        member: Pubkey,
    ) -> Result<()> {
        return removing_member(ctx, _team_name, _team_id, member);
    }

    pub fn transfer_captain(
        ctx: Context<TransferCaptain>,
        _team_name: String,
        _team_id: u64,
        member: Pubkey,
    ) -> Result<()> {
        return transfering_captain(ctx, _team_name, _team_id, member);
    }

    pub fn leave_team(ctx: Context<LeaveTeam>, _team_name: String, _team_id: u64) -> Result<()> {
        return leaving_team(ctx, _team_name, _team_id);
    }

    pub fn init_tournament(
        ctx: Context<InitTournament>,
        _team_name: String,
        _team_id: u64,
        tournament_address: Pubkey,
        tournament_prize: u64,
    ) -> Result<()> {
        return initiating_tournament(
            ctx,
            _team_name,
            _team_id,
            tournament_address,
            tournament_prize,
        );
    }

    // ----------------------------------------------
    // VOTING instructions

    pub fn vote_for_tournament(
        ctx: Context<VoteForTournament>,
        _team_name: String,
        _team_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        return voting_for_tournament(ctx, _team_name, _team_id, vote_type);
    }

    pub fn view_vote_result(
        ctx: Context<ViewVoteResults>,
        _team_name: String,
        _team_id: u64,
    ) -> Result<()> {
        return view_vote_results(ctx, _team_name, _team_id);
    }

    pub fn leave_tournament(
        ctx: Context<LeaveTournament>,
        _team_name: String,
        _team_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        return leaving_tournament(ctx, _team_name, _team_id, vote_type);
    }

    pub fn init_percentage_proposal(
        ctx: Context<InitPercentageProposal>,
        _team_name: String,
        _team_id: u64,
        percentages: Vec<u8>,
    ) -> Result<()> {
        return initiating_percentage_proposal(ctx, _team_name, _team_id, percentages);
    }

    pub fn distribution_proposal_handler(
        ctx: Context<DistributionProposalHandler>,
        _team_name: String,
        _team_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        return handle_distribute_proposal(ctx, _team_name, _team_id, vote_type);
    }
    // two functions above will basically be used to vote for the distribution of the rewards
    // the function below will use the logic to decide if a team can join the tournament or not

    pub fn can_join_tournament(
        ctx: Context<CanJoinTournament>,
        _team_name: String,
        _team_id: u64,
    ) -> Result<()> {
        return can_join(ctx, _team_name, _team_id);
    }

    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        _team_name: String,
        _team_id: u64,
        reward: u64,
    ) -> Result<()> {
        return claiming_reward(ctx, _team_name, _team_id, reward);
    }
}

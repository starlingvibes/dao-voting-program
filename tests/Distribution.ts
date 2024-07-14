import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { DaoVotingProgram } from "../target/types/dao_voting_program";

describe("Distribution tests", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const user = provider.wallet;

	const alice = anchor.web3.Keypair.generate();
	const bob = anchor.web3.Keypair.generate();
	const carol = anchor.web3.Keypair.generate();
	const dan = anchor.web3.Keypair.generate();
	let team = [alice, bob, carol, dan];

	let tournament = anchor.web3.Keypair.generate();

	const program = anchor.workspace.DaoVotingProgram as Program<DaoVotingProgram>;

	let teamName = "Superteam VN";
	let uid = new anchor.BN(5549239993);
	let teamAccountAddr;

	let tournamentPrize = anchor.web3.LAMPORTS_PER_SOL * 100;

	// the team addresses array

	let teamPda, teamBump;

	before(async () => {
		// create accounts to be used in tests
		const ix = await program.methods.createTeam(teamName, uid);
		teamAccountAddr = (await ix.pubkeys()).teamAccount;
		const tx = await ix.rpc();

		[teamPda, teamBump] = await anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from(teamName), Buffer.from(`${uid}`)],
			program.programId
		);

		for (let i = 0; i < team.length; i++) {
			await program.methods.addMember(teamName, uid, team[i].publicKey).rpc();
		}

		await program.methods
			.initTournament(
				teamName,
				uid,
				tournament.publicKey,
				new anchor.BN(tournamentPrize)
			)
			.rpc();

		for (let i = 0; i < 4; i++) {
			await program.methods
				.voteForTournament(teamName, uid, { yes: {} })
				.accounts({
					teamAccount: teamAccountAddr,
					signer: team[i].publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([team[i]])
				.rpc();
		}

		// await program.provider.connection.confirmTransaction(
		// 	await program.provider.connection.requestAirdrop(
		// 		alice.publicKey,
		// 		anchor.web3.LAMPORTS_PER_SOL * 101
		// 	)
		// );
	});

	it("should initialize percentage proposal successfully", async () => {
		let proposalPercentages = [30, 10, 20, 15, 25];
		await program.methods
			.initPercentageProposal(teamName, uid, Buffer.from(proposalPercentages))
			.rpc();

		let { distributionPercentages: distPerc } =
			await program.account.teamAccount.fetch(teamAccountAddr);

		// checking if the send array and the fetched array has the same values
		let isArrayEqual = proposalPercentages.every(
			(element, index) => element === distPerc[index]
		);

		assert.equal(isArrayEqual, true);
	});

	it("should let players vote for percentages successfully", async () => {
		// voting for 3 members
		for (let i = 0; i < 4; i++) {
			await program.methods
				.distributionProposalHandler(teamName, uid, { yes: {} })
				.accounts({
					teamAccount: teamAccountAddr,
					signer: team[i].publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([team[i]])
				.rpc();
		}

		let { distributionVotingResult: distResult } =
			await program.account.teamAccount.fetch(teamAccountAddr);

		assert.equal(distResult, true);
	});

	it("should be able to set canJoinTournament successfully", async () => {
		await program.methods.canJoinTournament(teamName, uid).rpc();

		let { canJoinTournament } = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(canJoinTournament, true);
	});
});

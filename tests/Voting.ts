import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { DaoVotingProgram } from "../target/types/dao_voting_program";

describe("Voting tests", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const user = provider.wallet;

	const alice = anchor.web3.Keypair.generate();
	const bob = anchor.web3.Keypair.generate();
	const carol = anchor.web3.Keypair.generate();
	const dan = anchor.web3.Keypair.generate();

	let tournament = anchor.web3.Keypair.generate();

	const program = anchor.workspace.DaoVotingProgram as Program<DaoVotingProgram>;

	let teamName = "Superteam UK";
	let uid = new anchor.BN(5098695);
	let teamAccountAddr;

	let team = [alice, bob, carol, dan];

	let teamPda, teamBump;

	before(async () => {
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
			.initTournament(teamName, uid, tournament.publicKey, new anchor.BN(100))
			.rpc();
	});

	it("should vote yes successfully", async () => {
		await program.methods.voteForTournament(teamName, uid, { yes: {} }).rpc();

		const teamDetails = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(
			teamDetails.votedPlayers[0].toString(),
			user.publicKey.toString()
		);
		assert.equal(teamDetails.yesVotes, 1);
	});

	it("should display vote result successfully", async () => {
		// await program.methods.voteForTournament(teamName, uid, { yes: {} }).rpc();
		await program.methods.viewVoteResult(teamName, uid).rpc();

		const teamDetails = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		console.log("Vote results");
		console.log("Yes votes: ", teamDetails.yesVotes);

		assert.equal(teamDetails.yesVotes, 1);
	});

	it("should set tournament address successfully", async () => {
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

		const teamDetails = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(
			teamDetails.activeTournament.toBase58(),
			tournament.publicKey.toBase58()
		);
		assert.equal(teamDetails.votingResult, true);
	});

	it("should not initialize another tournament if there is still an active one", async () => {
		let anotherTournament = anchor.web3.Keypair.generate();
		try {
			await program.methods
				.initTournament(
					teamName,
					uid,
					anotherTournament.publicKey,
					new anchor.BN(101)
				)
				.rpc();
		} catch (err) {
			assert.equal(
				err.error.errorMessage,
				"The team has an active tournament and cannot vote for another tournament, leave the current one first"
			);
			assert.equal(err.error.errorCode.code, "AlreadyActiveTournamentError");
		}
	});
});

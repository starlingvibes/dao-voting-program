import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { DaoVotingProgram } from "../target/types/dao_voting_program";

describe("Error tests", () => {
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

	let teamName = "Superteam IN";
	let uid = new anchor.BN(477783854);
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

		for (let i = 0; i < team.length - 1; i++) {
			await program.methods.addMember(teamName, uid, team[i].publicKey).rpc();
		}

		await program.methods
			.initTournament(teamName, uid, tournament.publicKey, new anchor.BN(100))
			.rpc();
	});

	it("should not let a player vote twice", async () => {
		try {
			await program.methods.voteForTournament(teamName, uid, { yes: {} }).rpc();
		} catch (err) {
			assert.equal(
				err.error.errorMessage,
				"Member has already voted for the tournament"
			);
			assert.equal(err.error.errorCode.code, "AlreadyVotedError");
		}
	});

	it("should not let anybody that is not in the team to vote", async () => {
		let anotherUser = anchor.web3.Keypair.generate();
		try {
			await program.methods
				.voteForTournament(teamName, uid, { yes: {} })
				.accounts({
					teamAccount: teamAccountAddr,
					signer: anotherUser.publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([anotherUser])
				.rpc();
		} catch (err) {
			assert.equal(err.error.errorMessage, "Member is not in the team");
			assert.equal(err.error.errorCode.code, "MemberNotInTeamError");
		}
	});

	it("should not let anyone else other than the captain add member", async () => {
		try {
			await program.methods
				.addMember(teamName, uid, dan.publicKey)
				.accounts({
					teamAccount: teamAccountAddr,
					signer: alice.publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([alice])
				.rpc();
		} catch (err) {
			assert.equal(
				err.error.errorMessage,
				"Only captain can call this function"
			);
			assert.equal(err.error.errorCode.code, "NotCaptainError");
		}
	});
});

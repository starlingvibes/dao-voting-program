import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { DaoVotingProgram } from "../target/types/dao_voting_program";

describe("Team CRUD tests", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const bob = anchor.web3.Keypair.generate();

	const program = anchor.workspace.DaoVotingProgram as Program<DaoVotingProgram>;

	let teamName = "Superteam NG";
	let uid = new anchor.BN(282589922);

	let teamAccountAddr;

	let teamPda, teamBump;

	before(async () => {
		[teamPda, teamBump] = await anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from(teamName), Buffer.from(`${uid}`)],
			program.programId
		);

		const ix = await program.methods.createTeam(teamName, uid);
		teamAccountAddr = (await ix.pubkeys()).teamAccount;

		const tx = await ix.rpc();
	});

	it("should create a team successfully", async () => {
		const teamAccount = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(teamAccount.name, teamName);
	});

	it("should add a member to the team", async () => {
		let teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		let teamLength = teamAccount.members.length;

		const ix = await program.methods.addMember(teamName, uid, bob.publicKey);
		const tx = await ix.rpc();

		teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		assert.equal(teamAccount.members.length, teamLength + 1);
		assert.equal(
			teamAccount.members[teamAccount.members.length - 1].toBase58(),
			bob.publicKey.toBase58()
		);
	});

	it("should remove a member from the team", async () => {
		let teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		let teamLength = teamAccount.members.length;

		const ix = await program.methods.removeMember(teamName, uid, bob.publicKey);
		const tx = await ix.rpc();

		teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		assert.equal(teamAccount.members.length, teamLength - 1);
	});

	it("should transfer the captain role of the team", async () => {
		let teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		let newMember = anchor.web3.Keypair.generate();

		const ix = await program.methods.addMember(
			teamName,
			uid,
			newMember.publicKey
		);
		const tx = await ix.rpc();

		const ix2 = await program.methods.transferCaptain(
			teamName,
			uid,
			newMember.publicKey
		);
		const tx2 = await ix2.rpc();

		teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		assert.equal(
			teamAccount.captain.toBase58(),
			newMember.publicKey.toBase58()
		);
	});

	it("should let a member to leave team successfully", async () => {
		let teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		let teamLength = teamAccount.members.length;

		const ix = await program.methods.leaveTeam(teamName, uid);
		const tx = await ix.rpc();

		teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		assert.equal(teamAccount.members.length, teamLength - 1);
	});
});

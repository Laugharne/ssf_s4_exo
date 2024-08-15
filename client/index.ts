import { Connection, clusterApiUrl, SystemProgram, Keypair, PublicKey, LAMPORTS_PER_SOL, TransactionInstruction, Transaction, sendAndConfirmTransaction } from  "@solana/web3.js";
import { Buffer } from 'node:buffer';

(async () => {
	// Connect to cluster
	//const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
	const connection = new Connection("http://localhost:8899");

	const TAG_SSF_PDA = 'SSF_PDA';

	  // Generate a new wallet keypair and airdrop SOL
	  const payer = Keypair.generate();
	  const payerAirdropSignature = await connection.requestAirdrop(
		payer.publicKey,
		2*LAMPORTS_PER_SOL
	  );
	  console.log("payer", payer.publicKey.toBase58());

	  // Wait for airdrop confirmation
	  await connection.confirmTransaction(payerAirdropSignature);
	  console.log("Airdrop done !");

	// Generate a new wallet keypair and airdrop SOL
	const user = Keypair.generate();
	const userAirdropSignature = await connection.requestAirdrop(
		user.publicKey,
		2*LAMPORTS_PER_SOL
	);
	console.log("payer", user.publicKey.toBase58());

	// Wait for airdrop confirmation
	await connection.confirmTransaction(userAirdropSignature);
	console.log("Airdrop done !");

    // Generate keypair for the new account
    //const newAccountKp = new web3.Keypair();
	const newAccountKp = Keypair.generate();
	const PROGRAM_ID = PublicKey.unique();

	// instruction index
	const ixIdxInitialize = 0;
	/*
	pub struct Vault {
		pub owner  : Pubkey,
	}
	*/
	const sizeVault = 32; // (Pubkey size)
	// https://book.anchor-lang.com/anchor_references/space.html


	// Create instruction data buffer
    const ixDataInitialize = Buffer.alloc(sizeVault + 8);
    ixDataInitialize.writeUInt8(ixIdxInitialize, 0);
    //-ixDataInitialize.writeBigUInt64LE(BigInt(data), 1);
/*
	let user: &AccountInfo           = next_account_info(accounts_iter)?;
	let vault: &AccountInfo          = next_account_info(accounts_iter)?;
	let system_program: &AccountInfo = next_account_info(accounts_iter)?;

*/
	const ixInitialize = new TransactionInstruction({
		keys:[
			{// user: &AccountInfo
				pubkey    : payer.publicKey,
				isSigner  : true,
				isWritable: true,
			},
			{// vault: &AccountInfo
				pubkey    : newAccountKp.publicKey,
				isSigner  : true,
				isWritable: true,
			},
			{// system_program: &AccountInfo
				pubkey    : SystemProgram.programId,
				isSigner  : false,
				isWritable: false,
			  },

		],
		programId: PROGRAM_ID,
		data: ixDataInitialize,
	});

    const transactionInitialize = new Transaction().add(ixInitialize);
    const txHashInitialize = await sendAndConfirmTransaction(
		connection,
		transactionInitialize,
		[payer, newAccountKp],
	);
	console.log(`Use 'solana confirm -v ${txHashInitialize}' to see the logs`);

	// instruction index
	const ixIdxDeposit = 1;

	/*
	pub struct Pda {
		pub signer      : Pubkey,
		pub balance     : u64,
		pub deposit_time: i64,      // Unix timestamp
		pub done        : bool,     // withdraw already done
	}
	*/
	const sizePda = 32 + 8 + 8 + 2; // (Pubkey + u64 + i64 + bool)
	// https://book.anchor-lang.com/anchor_references/space.html

	const amountToDepose = 1;

	// Create instruction data buffer
    const ixDataDeposit = Buffer.alloc(sizePda + 8);
    ixDataDeposit.writeUInt8(ixIdxDeposit, 0);
    ixDataDeposit.writeBigUInt64LE(BigInt(amountToDepose), 1);


	const [depositPda, depositBump] = PublicKey.findProgramAddressSync(
		[
			Buffer.from(TAG_SSF_PDA),
			user.publicKey.toBuffer()
		],
		PROGRAM_ID
	);

	/*
	let user: &AccountInfo           = next_account_info(accounts_iter)?;
	let user_pda: &AccountInfo       = next_account_info(accounts_iter)?;
	let system_program: &AccountInfo = next_account_info(accounts_iter)?;
	*/
	const ixDeposit = new TransactionInstruction({
		keys:[
			{// user: &AccountInfo
				pubkey    : user.publicKey,
				isSigner  : true,
				isWritable: true,
			},
			{// user_pda: &AccountInfo
				pubkey    : depositPda,
				isSigner  : true,
				isWritable: true,
			},
			{// system_program: &AccountInfo
				pubkey    : SystemProgram.programId,
				isSigner  : false,
				isWritable: false,
			},

		],
		programId: PROGRAM_ID,
		data: ixDataDeposit,
	});

	const transactionDeposit = new Transaction().add(ixDeposit);
	const txHashDeposit = await sendAndConfirmTransaction(
		connection,
		transactionDeposit,
		[user],
	);
	console.log(`Use 'solana confirm -v ${txHashDeposit}' to see the logs`);

	// instruction index
	const ixIdxWithdraw = 2;

	// Create instruction data buffer
    const ixDataWithdraw = Buffer.alloc(sizePda + 8);
    ixDataWithdraw.writeUInt8(ixIdxWithdraw, 0);

	const [withdrawPda, withdrawBump] = PublicKey.findProgramAddressSync(
		[
			Buffer.from(TAG_SSF_PDA),
			user.publicKey.toBuffer()
		],
		PROGRAM_ID
	);

	/*
	let user: &AccountInfo           = next_account_info(accounts_iter)?;
	let vault: &AccountInfo          = next_account_info(accounts_iter)?;
	let system_program: &AccountInfo = next_account_info(accounts_iter)?;
	*/
	const ixWithdraw = new TransactionInstruction({
		keys:[
			{// user: &AccountInfo
				pubkey    : user.publicKey,
				isSigner  : true,
				isWritable: true,
			},
			{// vault: &AccountInfo
				pubkey    : withdrawPda,
				isSigner  : true,
				isWritable: true,
			},
			{// system_program: &AccountInfo
				pubkey    : SystemProgram.programId,
				isSigner  : false,
				isWritable: false,
			},

		],
		programId: PROGRAM_ID,
		data: ixDataWithdraw,
	});

	const transactionWithdraw = new Transaction().add(ixWithdraw);
	const txHashWithdraw = await sendAndConfirmTransaction(
		connection,
		transactionWithdraw,
		[user],
	);
	console.log(`Use 'solana confirm -v ${txHashWithdraw}' to see the logs`);

})();
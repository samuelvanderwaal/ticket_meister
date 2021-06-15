import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Account, Connection, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction, TransactionInstruction } from "@solana/web3.js";
import BN from "bn.js";
import { EVENT_ACCOUNT_DATA_LAYOUT } from "./layout";
const connection = new Connection("http://localhost:8899", "singleGossip");
export const createEvent = async (privateKeyByteArray, eventName, maxTickets, eventProgramIdString) => {
    const privateKeyDecoded = privateKeyByteArray
        .split(",")
        .map(s => parseInt(s));
    const initializerAccount = new Account(privateKeyDecoded);
    const eventAccount = new Account();
    // const mintAccount = new Account();
    const eventProgramId = new PublicKey(eventProgramIdString);
    // Create an instruction object for creating a new event account, owned by the event program.
    const createEventAccountIx = SystemProgram.createAccount({
        space: EVENT_ACCOUNT_DATA_LAYOUT.span,
        lamports: await connection.getMinimumBalanceForRentExemption(EVENT_ACCOUNT_DATA_LAYOUT.span, "singleGossip"),
        fromPubkey: initializerAccount.publicKey,
        newAccountPubkey: eventAccount.publicKey,
        programId: eventProgramId
    });
    // const createMintAccountIx = SystemProgram.createAccount({
    //   space: MINT_ACCOUNT_DATA_LAYOUT.span,
    //   lamports: await connection.getMinimumBalanceForRentExemption(
    //     MINT_ACCOUNT_DATA_LAYOUT.span,
    //     "singleGossip"
    //   ),
    //   fromPubkey: initializerAccount.publicKey,
    //   newAccountPubkey: mintAccount.publicKey,
    //   programId: TOKEN_PROGRAM_ID
    // });
    const token = await Token.createMint(connection, initializerAccount, initializerAccount.publicKey, null, 0, TOKEN_PROGRAM_ID);
    console.log("Mint Account: ");
    console.log(token.publicKey.toBase58());
    // const createTokenMintIx = Token.createInitMintInstruction(
    //   TOKEN_PROGRAM_ID,
    //   mintAccount.publicKey,
    //   0,
    //   initializerAccount.publicKey,
    //   null
    // );
    let eventNameArray = Uint8Array.from(new Array(32).fill(0));
    // Truncate strings longer than 32 chars
    new TextEncoder().encodeInto(eventName.slice(0, 32), eventNameArray);
    console.log(eventNameArray);
    const createEventIx = new TransactionInstruction({
        programId: eventProgramId,
        keys: [
            {
                pubkey: initializerAccount.publicKey,
                isSigner: true,
                isWritable: false
            },
            { pubkey: eventAccount.publicKey, isSigner: false, isWritable: true },
            { pubkey: token.publicKey, isSigner: false, isWritable: true },
            { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }
        ],
        data: Buffer.from(Uint8Array.of(0, ...new BN(maxTickets).toArray("le", 8), ...eventNameArray))
    });
    const tx = new Transaction().add(
    // createMintAccountIx,
    // createTokenMintIx,
    createEventAccountIx, createEventIx);
    try {
        let res = await connection.sendTransaction(tx, [initializerAccount, eventAccount], {
            skipPreflight: false,
            preflightCommitment: "singleGossip"
        });
        console.log(res);
    }
    catch (err) {
        console.log(`Error in sending transaction: ${err}`);
    }
    await new Promise(resolve => setTimeout(resolve, 1000));
    const encodedEventState = (await connection.getAccountInfo(eventAccount.publicKey, "singleGossip")).data;
    console.log(encodedEventState);
    const decodedEventState = EVENT_ACCOUNT_DATA_LAYOUT.decode(encodedEventState);
    console.log(decodedEventState);
    /// Remove trailing zeros.
    let decodedEventName = decodedEventState.eventName.filter(e => e !== 0);
    console.log(decodedEventName);
    return {
        eventAccountPubkey: eventAccount.publicKey.toBase58(),
        isInitialized: !!decodedEventState.isInitialized,
        initializerAccountPubkey: new PublicKey(decodedEventState.initializerPubkey).toBase58(),
        ticketsPurchased: new BN(decodedEventState.ticketsPurchased, 10, "le").toNumber(),
        maxTickets: new BN(decodedEventState.maxTickets, 10, "le").toNumber(),
        eventName: new TextDecoder().decode(decodedEventName),
        mintAccount: new PublicKey(decodedEventState.mintAccount).toBase58()
    };
};
//# sourceMappingURL=createEvent.js.map
import { AccountLayout, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Account,
  Connection,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction
} from "@solana/web3.js";
import BN from "bn.js";
import { EVENT_ACCOUNT_DATA_LAYOUT, EventLayout } from "./layout";

const connection = new Connection("http://localhost:8899", "singleGossip");

export const createEvent = async (
  privateKeyByteArray: string,
  eventName: string,
  maxTickets: number,
  eventProgramIdString: string
) => {
  console.log(eventName);
  const privateKeyDecoded = privateKeyByteArray
    .split(",")
    .map(s => parseInt(s));
  const initializerAccount = new Account(privateKeyDecoded);

  const eventAccount = new Account();
  const eventProgramId = new PublicKey(eventProgramIdString);

  // Create an instruction object for creating a new event account, owned by the event program.
  const createEventAccountIx = SystemProgram.createAccount({
    space: EVENT_ACCOUNT_DATA_LAYOUT.span,
    lamports: await connection.getMinimumBalanceForRentExemption(
      EVENT_ACCOUNT_DATA_LAYOUT.span,
      "singleGossip"
    ),
    fromPubkey: initializerAccount.publicKey,
    newAccountPubkey: eventAccount.publicKey,
    programId: eventProgramId
  });

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
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }
    ],
    data: Buffer.from(
      Uint8Array.of(
        0,
        ...eventNameArray,
        ...new BN(maxTickets).toArray("le", 8)
      )
    )
  });

  const tx = new Transaction().add(createEventAccountIx, createEventIx);

  let res = await connection.sendTransaction(
    tx,
    [initializerAccount, eventAccount],
    {
      skipPreflight: false,
      preflightCommitment: "singleGossip"
    }
  );

  console.log(res);

  await new Promise(resolve => setTimeout(resolve, 1000));

  const encodedEventState = (await connection.getAccountInfo(
    eventAccount.publicKey,
    "singleGossip"
  ))!.data;
  console.log(encodedEventState);

  const decodedEventState = EVENT_ACCOUNT_DATA_LAYOUT.decode(
    encodedEventState
  ) as EventLayout;

  console.log(decodedEventState);

  /// Remove trailing zeros.
  let decodedEventName = decodedEventState.eventName.filter(e => e !== 0);
  console.log(decodedEventName);

  return {
    eventAccountPubkey: eventAccount.publicKey.toBase58(),
    isInitialized: !!decodedEventState.isInitialized,
    initializerAccountPubkey: new PublicKey(
      decodedEventState.initializerPubkey
    ).toBase58(),
    eventName: new TextDecoder().decode(decodedEventName),
    maxTickets: new BN(decodedEventState.maxTickets, 10, "le").toNumber()
  };
};

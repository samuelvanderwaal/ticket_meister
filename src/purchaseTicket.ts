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

export const purchaseTicket = async (
  privateKeyByteArray: string,
  eventAccountString: string,
  eventProgramIdString: string
) => {
  const privateKeyDecoded = privateKeyByteArray
    .split(",")
    .map((s) => parseInt(s));
  const initializerAccount = new Account(privateKeyDecoded);
  const eventProgramId = new PublicKey(eventProgramIdString);

  const eventAccountPubKey = new PublicKey(eventAccountString);

  const purchaseTicketIx = new TransactionInstruction({
    programId: eventProgramId,
    keys: [
      {
        pubkey: initializerAccount.publicKey,
        isSigner: true,
        isWritable: false
      },
      { pubkey: eventAccountPubKey, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }
    ],
    data: Buffer.from(Uint8Array.of(1, 11))
  });

  const tx = new Transaction().add(purchaseTicketIx);

  let res = await connection.sendTransaction(tx, [initializerAccount], {
    skipPreflight: false,
    preflightCommitment: "singleGossip"
  });

  console.log(res);

  await new Promise((resolve) => setTimeout(resolve, 1000));

  const encodedEventState = (await connection.getAccountInfo(
    eventAccountPubKey,
    "singleGossip"
  ))!.data;
  console.log("encoded event state");
  console.log(encodedEventState);

  const decodedEventState = EVENT_ACCOUNT_DATA_LAYOUT.decode(
    encodedEventState
  ) as EventLayout;

  console.log("decoded event state");
  console.log(decodedEventState);

  /// Remove trailing zeros.
  let decodedEventName = decodedEventState.eventName.filter((e) => e !== 0);
  console.log("decoded event name");
  console.log(decodedEventName);

  return {
    eventAccountPubkey: eventAccountPubKey.toBase58(),
    isInitialized: !!decodedEventState.isInitialized,
    initializerAccountPubkey: new PublicKey(
      decodedEventState.initializerPubkey
    ).toBase58(),
    eventName: new TextDecoder().decode(decodedEventName),
    ticketsPurchased: new BN(
      decodedEventState.ticketsPurchased,
      10,
      "le"
    ).toNumber(),
    maxTickets: new BN(decodedEventState.maxTickets, 10, "le").toNumber()
  };
};

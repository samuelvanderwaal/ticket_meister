import * as BufferLayout from "buffer-layout";
/**
 * Layout for a public key
 */
const publicKey = (property = "publicKey") => {
    return BufferLayout.blob(32, property);
};
/**
 * Layout for a eventName string
 */
const string = (property = "string") => {
    return BufferLayout.blob(32, property);
};
/**
 * Layout for a 64bit unsigned value
 */
const uint64 = (property = "uint64") => {
    return BufferLayout.blob(8, property);
};
export const EVENT_ACCOUNT_DATA_LAYOUT = BufferLayout.struct([
    BufferLayout.u8("isInitialized"),
    publicKey("initializerPubkey"),
    string("eventName"),
    uint64("maxTickets")
]);
//# sourceMappingURL=layout.js.map
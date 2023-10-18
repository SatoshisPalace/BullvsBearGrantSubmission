export interface TestSnip20InitMsg {
    name: string;
    symbol: string;
    decimals: number;
    prng_seed: string;
    config: {
        public_total_supply: boolean;
        enable_deposit: boolean;
        enable_redeem: boolean;
        enable_mint: boolean;
        enable_burn: boolean;
    };
}

export interface MintMsg {
    mint: {
        recipient: string;
        amount: string;
    };
}
export interface BalanceMsg {
    balance: {
        address: string,
        key: string,
    }
}
export interface CreateViewingKey {
    create_viewing_key: {
        entropy: string,
        padding: string | undefined,
    },
}

// Define the message interface
export interface SetViewingKeyMsg {
    set_viewing_key: {
        key: string,
        padding?: string,  // Using TypeScript's optional property syntax
    }
}

export interface SetViewingKeyResponse {
    status: ResponseStatus,  // Assuming ResponseStatus is already defined
}

export enum ResponseStatus {
    Success,
    Failure,
}

export interface SendMsg {
    send: {
        recipient: string;
        recipient_code_hash: string;
        amount: string;
        msg: string;
    };
}

export interface CreateViewingKeyResponse {
    key: string;
}

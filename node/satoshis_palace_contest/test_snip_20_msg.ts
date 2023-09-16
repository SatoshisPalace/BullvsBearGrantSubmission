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

export interface SendMsg {
    send: {
        recipient: string;
        recipient_code_hash: string;
        amount: string;
        msg: string;
    };
}

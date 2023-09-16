export interface RegisterMsg {
    register: {
        reg_addr: string;
        reg_hash: string;
    };
}

export interface GetContestMsg {
    get_contest: {
        contest_id: number;
    };
}

export interface GetSnip20sMsg {
    getSnip20s: {};
}

export interface GetContestCreationMsgBinary {
    get_contest_creation_msg_binary: {
        contest_info: ContestInfo;
        contest_info_signature_hex: string;
        outcome_id: number;
    };
}

export interface GetBetContestMsgBinary {
    get_bet_contest_msg_binary: {
        contest_id: number;
        outcome_id: number;
    };
}


export interface ContestInitMsg {
    oracle_contract: string;
    satoshis_palace: string;
}
export interface ContestInfo {
    id: number;
    options: ContestOutcome[];
    time_of_close: number;
    time_of_resolve: number;
}

export interface ContestOutcome {
    id: number;
    name: string;
}

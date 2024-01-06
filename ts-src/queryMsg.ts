import { TxResponse } from "secretjs";
import { ContestInfo, UserContest } from "./datatypes";

export type GetContestMsg = {
    getContest: {
        contestId: number;
    };
};

export type GetContestsMsg = {
    getContests: {
        contestIds: number[];
    };
};

export type GetUserBetMsg = {
    getUserBet: {
        userContest: UserContest;  // Assuming UserContest can be represented, adjust as necessary
        key: string;
    };
};

export type GetContestResultMsg = {
    getContestResult: {
        contestId: number;
    };
};

export type GetSnip20sMsg = {
    getSnip20s: {};  // Empty object as no properties are defined
};

export type GetContestCreationMsgBinaryMsg = {
    getContestCreationMsgBinary: {
        contestInfo: ContestInfo;  // Assuming ContestInfo can be represented, adjust as necessary
        contestInfoSignatureHex: string;
        outcomeId: number;
    };
};

export type GetBetContestMsgBinaryMsg = {
    getBetContestMsgBinary: {
        contestId: number;
        outcomeId: number;
    };
};

// Combined Type for all QueryMsg variants
export type QueryMsg =
    | GetContestMsg
    | GetContestsMsg
    | GetUserBetMsg
    | GetContestResultMsg
    | GetSnip20sMsg
    | GetContestCreationMsgBinaryMsg
    | GetBetContestMsgBinaryMsg;


///////////////////

// TypeScript types for Rust enum variants
export interface ContractIntegrationTemplate1Answer extends TxResponse {
    ContractIntegrationTemplate: {
    };
};

export interface ContractIntegrationTemplate1Answer extends TxResponse {
    ContractIntegrationTemplate: {
    };
};


// Combined Type for all QueryAnswer variants
export type QueryAnswer =
    | ContractIntegrationTemplate1Answer
    | ContractIntegrationTemplate1Answer
    ;
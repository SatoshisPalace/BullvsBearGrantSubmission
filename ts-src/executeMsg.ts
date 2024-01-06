import { TxResponse } from "secretjs";
import { ContestInfo } from "./datatypes";

export type CreateContestMsg = {
    createContest: {
        contestInfo: ContestInfo;  // Assuming ContestInfo can be represented as string
        contestInfoSignatureHex: string;
        outcomeId: number;
        sender?: string;  // Assuming Addr can be represented as string
        amount?: string;  // Assuming Uint128 can be represented as string
    };
};

export type BetContestMsg = {
    betContest: {
        contestId: number;
        outcomeId: number;
        sender?: string;  // Assuming Addr can be represented as string
        amount?: string;  // Assuming Uint128 can be represented as string
    };
};

export type ClaimMsg = {
    claim: {
        contestId: number;
    };
};

export type RegisterMsg = {
    register: {
        regAddr: string;  // Assuming Addr can be represented as string
        regHash: string;
    };
};

export type ReceiveMsg = {
    receive: {
        sender: string;  // Assuming Addr can be represented as string
        from: string;  // Assuming Addr can be represented as string
        amount: string;  // Assuming Uint128 can be represented as string
        memo?: string;
        msg: string;  // Assuming Binary can be represented as string
    };
};

export type RedeemMsg = {
    redeem: {
        addr: string;
        hash: string;
        to: string;  // Assuming Addr can be represented as string
        amount: string;  // Assuming Uint128 can be represented as string
        denom?: string;
    };
};

export type CreateViewingKeyMsg = {
    createViewingKey: {
        entropy: string;
        padding?: string;
    };
};

export type SetViewingKeyMsg = {
    setViewingKey: {
        key: string;
        padding?: string;
    };
};

// Combined Type for all ExecuteMsg variants
export type ExecuteMsg =
    | CreateContestMsg
    | BetContestMsg
    | ClaimMsg
    | RegisterMsg
    | ReceiveMsg
    | RedeemMsg
    | CreateViewingKeyMsg
    | SetViewingKeyMsg;


///////////////////////////////////


export interface ContractIntegrationTemplate1Response extends TxResponse {
    ContractIntegrationTemplate: {
    };
};

export interface ContractIntegrationTemplate2Response extends TxResponse {
    ContractIntegrationTemplate: {
    };
};

export type ExecuteAnswer  =
    | ContractIntegrationTemplate1Response
    | ContractIntegrationTemplate2Response
    ;
   

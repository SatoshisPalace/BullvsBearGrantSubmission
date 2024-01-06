import { SecretNetworkClient, TxResponse } from "secretjs";
import { Contract, ContractFactory } from "secretcontract"
import { ExecuteMsg } from "./executeMsg";
import { QueryAnswer, QueryMsg } from "./queryMsg";
import { InstantiateMsg } from "./initMsg";


export class Contest extends Contract {
    constructor(address: string, code_hash: string, secretjs: SecretNetworkClient) {
        super(address, code_hash, secretjs);
    }

    async execute(executeMsg: ExecuteMsg, gasLimit: number = 100_000): Promise<TxResponse> {
        return (await super.execute(executeMsg, gasLimit));
    }

    async query(queryMsg: QueryMsg): Promise<TxResponse| QueryAnswer> {
        const txResponse: TxResponse = await super.query(queryMsg);
        return txResponse
    }

}

export class ContesFactory extends ContractFactory {

    // New method for creating CounterContract instances
    static async createCounterContract(secretjs: SecretNetworkClient, initMsg: InstantiateMsg, contractWasm: Buffer): Promise<Contest> {
        const codeInfo = await Contract.upload_contract(secretjs, contractWasm);
        const contractAddress = await Contract.instantiate_contract(secretjs, codeInfo.codeId, codeInfo.contractCodeHash, initMsg);

        return new Contest(contractAddress, codeInfo.contractCodeHash, secretjs);
    }
}
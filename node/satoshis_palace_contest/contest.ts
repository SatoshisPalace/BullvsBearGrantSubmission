import { Contract } from "../sercet_network/Contract";
import { ClaimMsg, ContestInitMsg, GetBetContestMsgBinary, GetContestCreationMsgBinary, GetContestMsg, RegisterMsg } from "./contest_msg";


export class Contest extends Contract {

    constructor(contractCode: Buffer) {
        super(contractCode); // Call the constructor of the parent class
    }

    async instantiate(initMsg: ContestInitMsg): Promise<Contest> {
        // Call the parent class's instantiate method
        await super.instantiate(initMsg);
        return this;
    }

    async register(msg: RegisterMsg): Promise<any> {
        return await this.execute(msg);
    }

    async getContest(msg: GetContestMsg): Promise<any> {
        return await this.query(msg);
    }

    async getSnip20s(): Promise<any> {
        return await this.query({ get_snip20s: {} });
    }

    async getContestCreationMsgBinary(msg: GetContestCreationMsgBinary): Promise<any> {
        return await this.query(msg);
    }

    async getBetContestMsgBinary(msg: GetBetContestMsgBinary): Promise<any> {
        return await this.query(msg);
    }

    async claimReward(msg: ClaimMsg): Promise<any> {
        return await this.execute(msg);
    }
}
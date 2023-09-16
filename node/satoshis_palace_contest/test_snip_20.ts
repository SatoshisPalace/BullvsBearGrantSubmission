import { Contract } from "../sercet_network/Contract";
import { MintMsg, SendMsg, TestSnip20InitMsg } from "./test_snip_20_msg";

export class TestSnip20 extends Contract {

    constructor(contractCode: Buffer) {
        super(contractCode);
    }

    async instantiate(initMsg: TestSnip20InitMsg): Promise<TestSnip20> {
        await super.instantiate(initMsg);
        return this;
    }

	async mint(msg: MintMsg): Promise<any> {
        return await this.execute(msg);
    }
	
	async send(msg: SendMsg): Promise<any> {
        return await this.execute(msg);
    }

}

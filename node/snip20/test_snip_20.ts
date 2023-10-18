import { Contract } from "../sercet_network/Contract";
import { BalanceMsg, CreateViewingKey, CreateViewingKeyResponse, MintMsg, SendMsg, SetViewingKeyMsg, SetViewingKeyResponse, TestSnip20InitMsg } from "./test_snip_20_msg";

export class TestSnip20 extends Contract {

    constructor(contractCode: Buffer) {
        super(contractCode);
    }

    public static from(address: string, code_hash: string): TestSnip20 {
        const instance = new this(new Buffer([])); // Create an instance with an empty Buffer
        instance.from(address, code_hash); // Call the parent class's from() method
        return instance;
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

    async send_msg(
        contract: Contract,
        msg: any,
        amount: string
    ): Promise<any> {
        // Base64 encode the msg parameter
        // const encodedMsg = encodeMessage(msg);
    
        const encodedMsg = Buffer.from(JSON.stringify(msg)).toString('base64');
    
        // Create the SendMsg object
        const sendMsg: SendMsg = {
            send: {
                recipient: contract.getContractAddress(),
                recipient_code_hash: contract.getCodeHash(),
                amount: amount,
                msg: encodedMsg
            }
        };
    
        // Call the send method of TestSnip20
        return await this.send(sendMsg);
    }
    
    

    async getBalance(msg: BalanceMsg): Promise<any> {
        return await this.query(msg);
    }

    /** Deprecated: Decoding doesn't work properly but could be fixed in future */
    async createViewingKey(msg: CreateViewingKey): Promise<CreateViewingKeyResponse> {
        const response = await this.execute(msg);

        // Assuming the binary data is in response.data and it's a Buffer containing a UTF-8 encoded JSON string
        const binaryData = response.data;

        // Convert the Buffer to a UTF-8 string and then parse it as JSON
        let jsonString = binaryData.toString('base64');
        // Remove non-ASCII characters at the beginning
        // let jsonStringGood = jsonString.substring(1);
        const json = JSON.parse(jsonString);

        // Extract the key from the JSON object
        const createViewingKeyResponse: CreateViewingKeyResponse = json.create_viewing_key;

        return createViewingKeyResponse;
    }

    async setViewingKey(msg: SetViewingKeyMsg) {
        const response = await this.execute(msg);

        // Assuming the binary data is in response.data and it's a Buffer containing a UTF-8 encoded JSON string
        // const binaryData = response.data;

        // // Convert the Buffer to a UTF-8 string and then parse it as JSON
        // let jsonString = binaryData.toString('utf-8');
        // const json = JSON.parse(jsonString);

        // // Extract the status from the JSON object
        // const setViewingKeyResponse: SetViewingKeyResponse = json.set_viewing_key;

        // return setViewingKeyResponse;
    }
}

import fs from 'fs';
import { TestSnip20 } from './test_snip_20';

import path from 'path';
import { Contract } from '../sercet_network/Contract';
import { SendMsg } from './test_snip_20_msg';
const TEST_SNIP_20_CODE = path.resolve(__dirname, './test_snip_20_contract.wasm.gz');




export async function mintTestSnip20(testSnip20: TestSnip20 , address: string) {
	const mintMsg = {
		mint: {
			recipient: address,
			amount: '10000000000'
		}
	};
	return await testSnip20.mint(mintMsg);
}

export async function instantiateTestSnip20(): Promise<TestSnip20> {
	const testSnip20Code = fs.readFileSync(TEST_SNIP_20_CODE);
	var testSnip20 = new TestSnip20(testSnip20Code);
	await testSnip20.deploy();
	const testSnip20InitMsg = {
		name: "USDC",
		symbol: "USDC",
		decimals: 18,
		prng_seed: "VGhpcyBpcyBhIGJhc2UgNjQgZW5jb2RlZCBzdHJpbmcK",
		config: {
			public_total_supply: true,
			enable_deposit: true,
			enable_redeem: true,
			enable_mint: true,
			enable_burn: true
		}
	};
	await testSnip20.instantiate(testSnip20InitMsg);
	return testSnip20
}

function encodeMessage<T>(msg: T): string {
    return btoa(JSON.stringify(msg));
}

export async function send(
    testSnip20: TestSnip20,
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
    return await testSnip20.send(sendMsg);
}


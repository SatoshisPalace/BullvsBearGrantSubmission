import fs from 'fs';
import { TestSnip20 } from './satoshis_palace_contest/test_snip_20';
import { Contest } from './satoshis_palace_contest/contest';
import { SecretNetwork } from './sercet_network/SecretNetwork';
import { GetBetContestMsgBinary, GetContestCreationMsgBinary, GetContestMsg } from './satoshis_palace_contest/contest_msg';
import { SendMsg } from './satoshis_palace_contest/test_snip_20_msg';
import crypto from 'crypto';

const CONTEST_CONTRACT_CODE = "../contract.wasm.gz";
const TEST_SNIP_20_CODE = "../test_snip_20_contract.wasm.gz";

class MainExecutor {
    private sercet_network = SecretNetwork.getInstance();
    private testSnip20!: TestSnip20;
    private contest!: Contest;
    private viewingKey!: string;  // Declare a class-level variable to store the viewing key

    async instantiateTestSnip20() {
        const testSnip20Code = fs.readFileSync(TEST_SNIP_20_CODE);
        this.testSnip20 = new TestSnip20(testSnip20Code);
        await this.testSnip20.deploy();
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
        await this.testSnip20.instantiate(testSnip20InitMsg);
    }

    async mintTestSnip20() {
        const mintMsg = {
            mint: {
                recipient: this.sercet_network.getWallet().address,
                amount: '10000000000'
            }
        };
        return await this.testSnip20.mint(mintMsg);
    }

    async instantiateContest() {
        const contestCode = fs.readFileSync(CONTEST_CONTRACT_CODE);
        this.contest = new Contest(contestCode);
        await this.contest.deploy();
        const contestInitMsg = {
            oracle_contract: "ABCDEFGH",
            satoshis_palace: "04eec6a876668ffb7031f9b9ade7c0c4bc47681ac27fec532bfd5c94fb3dd71d675a363d7036ba8d831a499b12e4f04c8741b90e3c4f3c6b64dd1104132d49498c"
        };
        await this.contest.instantiate(contestInitMsg);
    }

    async registerSnip20WithContest() {
        const registerMsg = {
            register: {
                reg_addr: this.testSnip20.getContractAddress(),
                reg_hash: this.testSnip20.getCodeInfo().contractCodeHash
            }
        };
        return await this.contest.register(registerMsg);
    }

    async printRegisteredSnip20s() {
        const snip20s = await this.contest.getSnip20s();
        console.log("Registered Snip20s:");
        console.log(snip20s);
    }
    
    async fetchContestCreationMsgBinary(): Promise<any> {
        const msg: GetContestCreationMsgBinary = {
            get_contest_creation_msg_binary: {
                contest_info: {
                    id: 0,
                    options: [
                        {
                            id: 0,
                            name: "option1"
                        },
                        {
                            id: 1,
                            name: "option2"
                        }
                    ],
                    time_of_close: 0,
                    time_of_resolve: 0
                },
                contest_info_signature_hex: "ccf5c5b987455453eaddc62ce5b8e64877ea4f14500a7bdcce594e4b79303ceb29c5c9038e70177005b61cb6fbb486e7b22b76831da46c34e42f77909f0310f5",
                outcome_id: 0
            }
        };

        const result = await this.contest.getContestCreationMsgBinary(msg);
        return result.send.msg;
    }

    async sendContestMsgBinary(msg_binary: string): Promise<any> {
        const sendMsg: SendMsg = {
            send: {
                recipient: this.contest.getContractAddress(),
                recipient_code_hash: this.contest.getCodeInfo().contractCodeHash,
                amount: "1000",  // Adjust this value as needed
                msg: msg_binary
            }
        };

        const result = await this.testSnip20.send(sendMsg);
        return result;
    }

    async fetchContestDetails(contestId: number): Promise<any> {
        const getContestMsg: GetContestMsg = {
            get_contest: {
                contest_id: contestId
            }
        };

        const contestDetails = await this.contest.getContest(getContestMsg);
        return contestDetails;
    }

    async fetchBetContestMsgBinary(contestId: number, outcomeId: number): Promise<any> {
        const getBetContestMsgBinary: GetBetContestMsgBinary = {
            get_bet_contest_msg_binary: {
                contest_id: contestId,
                outcome_id: outcomeId
            }
        };

        const betContestMsgBinary = await this.contest.getBetContestMsgBinary(getBetContestMsgBinary);
        return betContestMsgBinary.send.msg;
    }

    async claimReward(contestId: number): Promise<any> {
        const claimMsg = {
            claim: {
                contest_id: contestId
            }
        };
        return await this.contest.claimReward(claimMsg);
    }

    async setAndSaveViewingKey(): Promise<void> {
        const key = this.generateRandomKey();  // Generate a random key
        const setViewingKeyMsg = {
            set_viewing_key: {
                key: key,
                padding: undefined  // Optional, you can also set some padding here
            }
        };

        const response = await this.testSnip20.setViewingKey(setViewingKeyMsg);
        this.viewingKey = key;  // Save the viewing key
    }

    generateRandomKey(): string {
        const random = crypto.randomBytes(32);
        return random.toString('hex');
    }


    async checkBalance(): Promise<any> {
        const balanceMsg = {
            balance: {
                address: this.sercet_network.getWallet().address,
                key: this.viewingKey  // Use the saved viewing key
            }
        };
        const balance = await this.testSnip20.getBalance(balanceMsg);
        console.log("----------CheckBalance-------------");
        console.log(balance);
    }

    async execute() {
        console.log("----------TestSnip20-------------")
        await this.instantiateTestSnip20();
        console.log("----------MintSnip20-------------")
        await this.mintTestSnip20();
        console.log("----------ContestContract-------------")
        await this.instantiateContest();
        const snip_20_register_result = await this.registerSnip20WithContest();
        // console.log(snip_20_register_result)
        await this.printRegisteredSnip20s();

        // Create and save the viewing key
        console.log("----------SetViewingKey-------------")
        await this.setAndSaveViewingKey();

        await this.checkBalance();  // Check balance after making a contest

        console.log("----------ContestCreate-------------")
        const contest_create_msg_binary = await this.fetchContestCreationMsgBinary()
        const create_contest_response = await this.sendContestMsgBinary(contest_create_msg_binary)
        // console.log(create_contest_response);

        await this.checkBalance();  // Check balance after making a contest

        console.log("----------ContestBet-------------")
        const betContestMsgBinary = await this.fetchBetContestMsgBinary(0, 0);  // Assuming contest ID 0 and outcome ID 0 for this example
        const betResponse = await this.sendContestMsgBinary(betContestMsgBinary);
        // console.log(betResponse);

        await this.checkBalance();  // Check balance after making a contest

        console.log("----------GetContest-------------")
        const contest_bet_details = await this.fetchContestDetails(0)
        // console.log(JSON.stringify(contest_bet_details, null, 2));

        console.log("----------ClaimReward-------------");
        const claimResult = await this.claimReward(0); // Replace 0 with the actual contest ID
        // console.log(claimResult);

        await this.checkBalance();  // Check balance after making a contest

    }
}

// Execute the main method
const executor = new MainExecutor();
executor.execute().catch(err => {
    console.error('Error in main:', err);
});

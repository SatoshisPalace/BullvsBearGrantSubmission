import fs from 'fs';
import { TestSnip20 } from './satoshis_palace_contest/test_snip_20';
import { Contest } from './satoshis_palace_contest/contest';
import { SecretNetwork } from './sercet_network/SecretNetwork';
import { GetBetContestMsgBinary, GetContestCreationMsgBinary, GetContestMsg } from './satoshis_palace_contest/contest_msg';
import { SendMsg } from './satoshis_palace_contest/test_snip_20_msg';

const CONTEST_CONTRACT_CODE = "../contract.wasm.gz";
const TEST_SNIP_20_CODE = "../test_snip_20_contract.wasm.gz";

class MainExecutor {
    private sercet_network = SecretNetwork.getInstance();
    private testSnip20!: TestSnip20;
    private contest!: Contest;

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
                amount: "1",  // Adjust this value as needed
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

    async execute() {
        await this.instantiateTestSnip20();
        await this.mintTestSnip20();
        await this.instantiateContest();
        await this.registerSnip20WithContest();
        await this.printRegisteredSnip20s();

        const contest_create_msg_binary = await this.fetchContestCreationMsgBinary()
        console.log("----------ContestCreate-------------")
        console.log(await this.sendContestMsgBinary(contest_create_msg_binary));

        const betContestMsgBinary = await this.fetchBetContestMsgBinary(0, 0);  // Assuming contest ID 0 and outcome ID 0 for this example
        console.log("----------ContestBet-------------")
        console.log(await this.sendContestMsgBinary(betContestMsgBinary));

        console.log("----------GetContest-------------")
        const contest_bet_details = await this.fetchContestDetails(0)
        console.log(JSON.stringify(contest_bet_details, null, 2));
    }
}

// Execute the main method
const executor = new MainExecutor();
executor.execute().catch(err => {
    console.error('Error in main:', err);
});

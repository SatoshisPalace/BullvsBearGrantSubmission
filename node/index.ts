import fs from 'fs';
import { Contest } from './satoshis_palace_contest/contest';
import { SecretNetwork } from './sercet_network/SecretNetwork';
import { GetBetContestMsgBinary, GetContestCreationMsgBinary, GetContestMsg } from './satoshis_palace_contest/contest_msg';
import crypto from 'crypto';
import { TestSnip20 } from './snip20/test_snip_20';
import { SendMsg } from './snip20/test_snip_20_msg';

const CONTEST_CONTRACT_CODE = "../contract.wasm.gz";
const SNIP_20_CONTRACT_ADDRESS = "secret18rh39utg2vyyymgevqfksquea8ve9yp8vauzyy"
const SNIP_20_CONTRACT_CODE_HASH = "18c26bde92662902d3d9b090edff53642e9da4f46d0a76ec921c9dea0c64dcda"



class MainExecutor {
    private sercet_network = SecretNetwork.getInstance();
    private testSnip20!: TestSnip20;
    private contest!: Contest;
    private viewingKey!: string;  // Declare a class-level variable to store the viewing key


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
                    id: 1,
                    options: [
                        {
                            id: 0,
                            name: "Arizona Cardinals"
                        },
                        {
                            id: 1,
                            name: "Atlanta Falcons"
                        }
                    ],
                    time_of_close: 1384759,
                    time_of_resolve: 1385509,
                    event_details: "NFL game 1",
                },
                contest_info_signature_hex: "bb8e4c6b9bacce9be10c404111da06a40518c5a1965eb19e1d431050de3218f707b72fa3a05767bb53967f387eb6693c3631cec1dbf2fee7cdcb00d712e30397",
                outcome_id: 0
            }
        };
        console.log(msg)
        const result = await this.contest.getContestCreationMsgBinary(msg);
        console.log(result)
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
        console.log("--------Conontest Creation MSG--------")
        console.log(result)
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
        console.log(response)
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
        this.testSnip20 = TestSnip20.from(SNIP_20_CONTRACT_ADDRESS, SNIP_20_CONTRACT_CODE_HASH)
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

        await this.checkBalance();  // Check balance after making a contest

        console.log("----------ContestBet-------------")
        const betContestMsgBinary = await this.fetchBetContestMsgBinary(1, 0);  // Assuming contest ID 0 and outcome ID 0 for this example
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

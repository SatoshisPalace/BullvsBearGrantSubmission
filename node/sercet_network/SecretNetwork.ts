import { SecretNetworkClient, TxResponse, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

interface SecretNetworkOptions {
	chainId: string;
	url: string;
	wallet: any; // You can specify a more specific type if you have one
	walletAddress: string;
}

export interface CodeInfo {
	codeId?: string,
	contractCodeHash: string
}

export class SecretNetwork {
	private static instance: SecretNetwork;
	private secretjs: SecretNetworkClient; // You can specify a more specific type if you have one
	private wallet: Wallet;

	// Private constructor
	private constructor(options: SecretNetworkOptions) {
		this.secretjs = new SecretNetworkClient(options);
		this.wallet = options.wallet;
	}

	// Public static method to get the instance of the class
	public static getInstance(): SecretNetwork {
		if (!this.instance) {
			this.instance = new SecretNetwork(SecretNetwork.getdefaultOptions());
		}
		return this.instance;
	}

	private static getdefaultOptions() {
		if (!process.env.CHAIN_ID) {
			throw new Error("CHAIN_ID environment variable is missing!");
		}
		if (!process.env.RPC_URL) {
			throw new Error("RPC_URL environment variable is missing!");
		}
		if (!process.env.MNEMONIC) {
			throw new Error("MNEMONIC environment variable is missing!");
		}
		const wallet = new Wallet(process.env.MNEMONIC);
		console.log(`Using \nChain ID: ${process.env.CHAIN_ID}\nEndpoint${process.env.RPC_URL}\nAddress:${wallet.address}`)
		const defaultOptions: SecretNetworkOptions = {
			chainId: process.env.CHAIN_ID,
			url: process.env.RPC_URL,
			wallet: wallet,
			walletAddress: wallet.address
		};
		return defaultOptions
	}

	public async upload_contract(contract_wasm: Buffer): Promise<CodeInfo> {
		const tx: TxResponse = await this.secretjs.tx.compute.storeCode(
			{
				sender: this.wallet.address,
				wasm_byte_code: contract_wasm,
				source: "",
				builder: "",
			},
			{
				gasLimit: 4_000_000,
			}
		);
		const codeId = tx.arrayLog?.find(
			(log) => log.type === "message" && log.key === "code_id"
		)?.value;

		if (!codeId) {
			throw new Error(`Failed to upload code`)
		}

		const contractCodeHash = (
			await this.secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
		).code_hash;

		if (!contractCodeHash) {
			throw new Error("Failed to upload code")
		}

		const code_info: CodeInfo = {
			codeId: codeId,
			contractCodeHash: contractCodeHash
		}
		console.log(`Uploaded Code Info: ${JSON.stringify(code_info, null, 2)}`);
		return code_info;
	}

	public async instantiate_contract(codeId: number, contractCodeHash: string, initMsg: any): Promise<string> {
		const tx: TxResponse = await this.secretjs.tx.compute.instantiateContract(
			{
				code_id: codeId,
				sender: this.wallet.address,
				code_hash: contractCodeHash,
				init_msg: initMsg,
				label: "SP_Contract_" + Math.ceil(Math.random() * 10000),
			},
			{
				gasLimit: 400_000,
			}
		);
		// Find the contract_address in the logs
		const contractAddress = tx.arrayLog?.find(
			(log: { type: string; key: string; }) => log.type === "message" && log.key === "contract_address"
		)?.value ?? null;

		if (!contractAddress) {
			throw new Error(`Contract Failed to instantiate:${tx}`)
		}
		console.log(`Contract Address: ${contractAddress}`)
		return contractAddress;
	}

	public async execute(contractAddress: string, msg: any, codeHash: string): Promise<any> {
		const tx: TxResponse = await this.secretjs.tx.compute.executeContract(
			{
				sender: this.wallet.address,
				contract_address: contractAddress,
				msg: msg,
				code_hash: codeHash,
			},
			{ gasLimit: 100_000 }
		);

		return tx;
	}
	public async query(contractAddress: string, codeHash: string, query: any): Promise<any> {
		const tx: TxResponse = await this.secretjs.query.compute.queryContract({
			contract_address: contractAddress,
			code_hash: codeHash,
			query: query
		});

		return tx;
	}
	public getWallet(): Wallet {
		return this.wallet
	}
}
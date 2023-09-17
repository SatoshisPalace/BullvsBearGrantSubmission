import { CodeInfo, SecretNetwork } from "./SecretNetwork";

export class Contract {

    protected secretNetwork: SecretNetwork;
    protected contractCode: Buffer;
    protected codeInfo: CodeInfo | undefined;
    protected contractAddress?: string;

    constructor(contractCode: Buffer) {
        this.secretNetwork = SecretNetwork.getInstance();
        this.contractCode = contractCode;
    }

    async deploy(): Promise<Contract> {
        const result = await this.secretNetwork.upload_contract(this.contractCode);

        this.codeInfo = {
            codeId: result.codeId,
            contractCodeHash: result.contractCodeHash
        };
        return this
    }

    // Method to instantiate the contract
    async instantiate(initMsg: any): Promise<Contract> {
        if (!this.codeInfo) {
            throw new Error("Contract has not been uploaded yet.");
        }

        // Use this.secretNetwork to instantiate the contract
        this.contractAddress = await this.secretNetwork.instantiate_contract(
            Number(this.codeInfo.codeId),
            this.codeInfo.contractCodeHash,
            initMsg
        );
        return this
    }

    // Method to execute a message on the contract
    async execute(msg: any): Promise<any> {
        if (!this.contractAddress || !this.codeInfo) {
            throw new Error("Contract has not been instantiated or uploaded.");
        }

        const tx = await this.secretNetwork.execute(
            this.contractAddress,
            msg,
            this.codeInfo.contractCodeHash
        );
        return tx;
    }

    // Method to query the contract
    async query(queryObj: any): Promise<any> {
        if (!this.contractAddress || !this.codeInfo) {
            throw new Error("Contract has not been instantiated or uploaded.");
        }

        const result = await this.secretNetwork.query(
            this.contractAddress,
            this.codeInfo.contractCodeHash,
            queryObj
        );
        return result;
    }

    public getContractAddress(): string {
        if (!this.contractAddress) {
            throw new Error("Contract has not been instantiated yet.");
        }
        return this.contractAddress;
    }

    public getCodeInfo(): CodeInfo {
        if (!this.codeInfo) {
            throw new Error("Contract has not been uploaded yet.");
        }
        return this.codeInfo;
    }
}

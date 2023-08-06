#! /bin/bash
echo "----Cleaning Build----"
make clean
echo "----------------------"
echo "----Building----"
make build-mainnet
echo "----------------"
echo "----Setting Up Secret CLI----"
echo "Using Wallet Name: $WALLET_NAME"
echo "Using Wallet Address: $WALLET_ADDRESS"
echo "Using Secret Node: $SECRET_NODE"
echo "Using Chain ID: $CHAIN_ID"
secretcli config node $SECRET_NODE
secretcli config chain-id $CHAIN_ID
secretcli config output json
echo "--------Filling Wallet--------"
curl "http://localhost:5000/faucet?address=$WALLET_ADDRESS"
sleep 5
printf "\n---------Wallet Filled---------\n"
echo "------Deploying Contract------"
secretcli tx compute store contract.wasm.gz --gas 5000000 --from $WALLET_NAME --chain-id $CHAIN_ID -y
sleep 5
echo "--------------------------"
echo "----Instantiating Contract----"
echo "Using Satoshis Palace Address: $SATOSHIS_PALACE_PUBLIC_KEY"
secretcli tx compute instantiate $DEPLOYMENT_NUM "{\"oracle_contract\": \"$ORACLE_CONTRACT_ADDRESS\",\"satoshis_palace\": \"$SATOSHIS_PALACE_PUBLIC_KEY\"}" --from $WALLET_NAME --label contestContract$DEPLOYMENT_NUM -y
sleep 5
echo "------------------------------"
secretcli query compute list-contract-by-code $DEPLOYMENT_NUM
echo "Please set your contract address: export CONTRACT_ADDRESS="
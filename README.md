# Satoshis Palace Contest


# Generate Schemas:
JSON schemas for contract messages are placed in `/schema` and can be generated with the following command:
```
cargo schema
```
Schemas can be used to create actual json [here](https://www.liquid-technologies.com/online-schema-to-json-converter)

Some examples are also provided within `/schema/examples` 

# Use
## Jump Start
Build, Deploy, and Instantiate the contract with 1 command
```
chmod +x ./jump-start.sh;chmod +x ./env.sh;. ./env.sh;./jump-start.sh  
```
## Build Project:
```
make build-mainnet
```
## Deploy
```
secretcli tx compute store contract.wasm.gz --gas 5000000 --from myWallet --chain-id secretdev-1 -y
```
## Instantiate
```
secretcli tx compute instantiate 1 '{           
        "oracle_contract": "ABCDEFGH",
        "satoshis_palace": "02087602e71a82777a7a9c234b668a1dc942c9a29bf31c931154eb331c21b6f6fd"
}' --from myWallet --label contestContract -y
```
## Execute
Create A Contest
```
secretcli tx compute execute $CONTRACT_ADDRESS '{
	"create_contest": {
		"contest_info": {
			"id": 0,
			"options": [
				{
					"id": 0,
					"name": "option1"
				},
				{
					"id": 0,
					"name": "option2"
				}
			],
			"time_of_close": 0,
			"time_of_resolve": 0
		},
		"contest_info_signature_hex": "f05eeb907cfa5b82742995fd471c73fdd320e7086af5b8bc3e2818675a7f5c19307661956b7d01fea44ddb32dbc8b352a465622bdd6f68171904cd00a4886889",
		"users_bet": {
			"option": 0,
			"value": 0
		}
	}
}' --from myWallet
```
## Query
### Query A Contest
```
secretcli query compute query $CONTRACT_ADDRESS '{
	"get_contest": {
	  "contest_id": 0
	}
}'
```
### Get Contest Creation Msg Binary
```
secretcli query compute query $CONTRACT_ADDRESS '{
	"get_contest_creation_msg_binary": {
		"contest_info": {
			"id": 0,
			"options": [
				{
					"id": 0,
					"name": "option1"
				},
				{
					"id": 1,
					"name": "option2"
				}
			],
			"time_of_close": 0,
			"time_of_resolve": 0
		},
		"contest_info_signature_hex": "ccf5c5b987455453eaddc62ce5b8e64877ea4f14500a7bdcce594e4b79303ceb29c5c9038e70177005b61cb6fbb486e7b22b76831da46c34e42f77909f0310f5",
		"outcome_id": 0
	}
}'
```
## Signing Contests
[Signing Repository](https://github.com/SatoshisPalace/Signer)

## Testing
```
cargo unit-test
```
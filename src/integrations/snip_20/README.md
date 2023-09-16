# SNIP-20 functions

## Register SNIP-20 Contract
```
secretcli tx compute execute $CONTRACT_ADDRESS '{
  "register": {
    "reg_addr": "$SNIP_20_CONTRACT_ADDRESS",
    "reg_hash": "$SNIP_20_CODE_HASH"
  }
}' --from myWallet -y
```
## Receive SNIP-20

## Redeem SNIP-20


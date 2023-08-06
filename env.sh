echo "----Setting Environment Variables----"
export WALLET_NAME=myWallet
export WALLET_ADDRESS=secret1nlfhwjhsx9hf6akmcep9p85w9k62vvzvaxemct
export CHAIN_ID=secretdev-1
export SECRET_NODE=http://localhost:26657
export ORACLE_CONTRACT_ADDRESS=ABCDEFGH
export SATOSHIS_PALACE_PUBLIC_KEY="04eec6a876668ffb7031f9b9ade7c0c4bc47681ac27fec532bfd5c94fb3dd71d675a363d7036ba8d831a499b12e4f04c8741b90e3c4f3c6b64dd1104132d49498c"

###### Do not edit beyond this point! ######
if [[ -z "${DEPLOYMENT_NUM}" ]]; then #DNE
  	export DEPLOYMENT_NUM=1
else # EXISTS
  	export DEPLOYMENT_NUM=$((DEPLOYMENT_NUM + 1)) 
fi
echo "------Environment Variables Set------"

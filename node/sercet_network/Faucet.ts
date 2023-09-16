const axios = require('axios');

export async function fillWallet(address: string) {
    try {
        const response = await axios.get(`http://localhost:5000/faucet?address=${address}`);
        console.log(response.data);
    } catch (error) {
        console.error('Error fetching from faucet:', error);
    }
}
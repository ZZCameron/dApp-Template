/* The code needed to interface the dApp with the components on the devnet solana chain
    The user is expected to have a compatible wallet that the wallet adapter can connect to 
    While others are available and should be accommodated, the adapter in this Template is
    for a Phantom wallet */

    
import { Connection, PublicKey } from '@solana/web3.js';

// Initialize Solana connection (Devnet for now).  The 'confirmed' is the commitment level 
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// Function to connect to a wallet (e.g., Phantom)
export async function connectWallet() { // This is a reusable function that is imported by another file
    try {
        const provider = window.solana; // Assumes Phantom wallet
        if (!provider) throw new Error('No wallet found');
        await provider.connect();
        const walletPublicKey = new PublicKey(provider.publicKey.toString());
        return walletPublicKey;
    } catch (error) {
        console.error('Wallet connection failed:', error);
        throw error;
    }
}

// Function to verify wallets (payer and receiver) on-chain
export async function verifyWallets(payerPublicKey, receiverPublicKey) {
    try {
        // Placeholder: Add your on-chain Rust program call here
        // Example: Check if both wallets are valid and funded
        const payerAccount = await connection.getAccountInfo(new PublicKey(payerPublicKey)); // An ordinary Solana RPC API using connection.getAccountInfo()
        const receiverAccount = await connection.getAccountInfo(new PublicKey(receiverPublicKey));
        if (!payerAccount || !receiverAccount) throw new Error('Invalid wallet');
        return true; // Wallets are valid
    } catch (error) {
        console.error('Wallet verification failed:', error);
        throw error;
    }
}

// Update wallet status on the page
export function updateWalletStatus(status) {
    const statusElement = document.getElementById('wallet-status');
    if (statusElement) statusElement.textContent = status;
}
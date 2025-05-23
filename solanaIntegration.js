/* The code needed to interface the dApp with the components on the devnet solana chain
    The user is expected to have a compatible wallet that the wallet adapter can connect to 
    While others are available and should be accommodated, the adapter in this Template is
    for a Phantom wallet */

    
import { Connection, Keypair, PublicKey } from 'https://cdn.jsdelivr.net/npm/@solana/web3.js@1.91.8/+esm';;

// Define wallet and program ID
let wallet = { publicKey: null, keypair: null }; // Define wallet globally
const PROGRAM_ID = new PublicKey("52UBAneHVYsa3C2kQiitp6SE4PZJ3nW6jdzcdSvm2yrw");

// Initialize Solana connection (Devnet for now).  The 'confirmed' is the commitment level 
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// Connect to a provider wallet (e.g., Phantom)
export async function connectWallet() { // This is a reusable function that is imported by another file
    try {
        const provider = window.solana; // Assumes Phantom wallet
        if (!provider) throw new Error('No wallet found');
        await provider.connect();
        const walletPublicKey = new PublicKey(provider.publicKey.toString());
        wallet.publicKey = walletPublicKey; //Store in global wallet object
        wallet.keypair = Keypair.generate(); // Optional:  If your app needs a keypair
        return wallet.publicKey;
    } catch (error) {
        console.error('Wallet connection failed:', error);
        throw error;
    }
}

// Verify the provider wallet is on-chain
export async function verifyProviderWallet() {
    try {
        if (!wallet.publicKey) throw new Error("Wallet not connected");
        // Call the on-chain program to verify the provider wallet exists
        // This is a placeholdet-replace with actual Solana program call
        const providerAccount = await connection.getAccountInfo(wallet.publicKey); // An ordinary Solana RPC API using connection.getAccountInfo()
        if (!providerAccount) throw new Error("Provider wallet not found on-chain");
        return true; // Wallet exists
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
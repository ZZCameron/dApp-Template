/* The code needed to start the dApp by verifying connection with the wallet */

import { connectWallet, verifyWallets, updateWalletStatus } from './solanaIntegration.js';

async function init() {
    try {
        // Connect to wallet
        const payerPublicKey = await connectWallet();
        updateWalletStatus(`Connected: ${payerPublicKey.toString()}`);

        // Verify wallets (assuming a receiver wallet is known)
        const receiverPublicKey = 'RECEIVER_PUBLIC_KEY_HERE'; // Replace with actual receiver
        const isValid = await verifyWallets(payerPublicKey, receiverPublicKey);
        if (isValid) updateWalletStatus('Wallets verified successfully');
    } catch (error) {
        updateWalletStatus('Wallet setup failed');
    }
}

// Run on page load
window.addEventListener('load', init);
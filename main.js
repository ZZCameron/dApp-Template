/* The code needed to start the dApp by verifying connection with the wallet */

import { connectWallet, verifyProviderWallet, updateWalletStatus } from './solanaIntegration.js';

async function init() {
    try {
        // Connect to provider wallet
        const providerPublicKey = await connectWallet();
        updateWalletStatus(`Connected: ${providerPublicKey.toString()}`);

        // Verify the provider wallet on-chain
        const isValid = await verifyProviderWallet();
        if (isValid) updateWalletStatus('Provider wallet verified on-chain');
    } catch (error) {
        updateWalletStatus('Wallet setup failed');
    }
}

// Run on page load
window.addEventListener('load', init);
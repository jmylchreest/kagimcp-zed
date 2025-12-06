#!/usr/bin/env node

/**
 * Postinstall script for Kagi MCP Server NPX wrapper
 *
 * This script attempts to download the appropriate binary during npm install,
 * so that the first run is faster. If download fails, it will be retried
 * on first use.
 */

const KagiMcpServerRunner = require('../bin/kagi-mcp-server.js');

async function main() {
    const runner = new KagiMcpServerRunner();

    // Skip if binary already exists
    if (runner.binaryExists()) {
        console.log('✅ Kagi MCP Server binary already cached');
        return;
    }

    console.log('📦 Pre-downloading Kagi MCP Server binary...');

    try {
        await runner.installBinary();
        console.log('✅ Binary installed successfully');
    } catch (err) {
        // Don't fail installation if download fails - it will be retried on first run
        console.warn('⚠️  Could not pre-download binary (will retry on first run)');
        console.warn(`   Reason: ${err.message}`);
    }
}

main().catch((err) => {
    // Don't fail npm install
    console.warn('⚠️  Postinstall warning:', err.message);
});

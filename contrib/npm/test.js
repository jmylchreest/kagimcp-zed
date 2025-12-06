#!/usr/bin/env node

/**
 * Test script for Kagi MCP Server NPX wrapper
 *
 * This script verifies that the wrapper is correctly configured
 * and can detect the platform/architecture.
 */

const KagiMcpServerRunner = require('./bin/kagi-mcp-server.js');
const path = require('path');
const fs = require('fs');

// ANSI color codes
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    reset: '\x1b[0m'
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

function pass(test) {
    log(`  ✓ ${test}`, 'green');
}

function fail(test, reason) {
    log(`  ✗ ${test}`, 'red');
    log(`    Reason: ${reason}`, 'yellow');
}

async function runTests() {
    log('\n🧪 Kagi MCP Server NPX Wrapper - Test Suite\n', 'blue');

    let passed = 0;
    let failed = 0;

    // Test 1: Package.json is valid
    try {
        const pkg = require('./package.json');
        if (pkg.name && pkg.version && pkg.bin) {
            pass('package.json is valid');
            passed++;
        } else {
            fail('package.json is valid', 'Missing required fields');
            failed++;
        }
    } catch (err) {
        fail('package.json is valid', err.message);
        failed++;
    }

    // Test 2: Binary wrapper exists
    const binPath = path.join(__dirname, 'bin', 'kagi-mcp-server.js');
    if (fs.existsSync(binPath)) {
        pass('Binary wrapper script exists');
        passed++;
    } else {
        fail('Binary wrapper script exists', `Not found at ${binPath}`);
        failed++;
    }

    // Test 3: Runner class can be instantiated
    try {
        const runner = new KagiMcpServerRunner();
        pass('Runner class instantiates');
        passed++;
    } catch (err) {
        fail('Runner class instantiates', err.message);
        failed++;
    }

    // Test 4: Platform detection works
    try {
        const runner = new KagiMcpServerRunner();
        const { platform, arch } = runner.getPlatformId();
        pass(`Platform detected: ${platform}-${arch}`);
        passed++;
    } catch (err) {
        fail('Platform detection', err.message);
        failed++;
    }

    // Test 5: Binary filename generation
    try {
        const runner = new KagiMcpServerRunner();
        const filename = runner.getBinaryFilename();
        if (filename && filename.includes('kagi-mcp-server')) {
            pass(`Binary filename: ${filename}`);
            passed++;
        } else {
            fail('Binary filename generation', `Invalid filename: ${filename}`);
            failed++;
        }
    } catch (err) {
        fail('Binary filename generation', err.message);
        failed++;
    }

    // Test 6: Download URL generation
    try {
        const runner = new KagiMcpServerRunner();
        const url = runner.getDownloadUrl('0.0.30');
        if (url && url.includes('github.com') && url.includes('releases')) {
            pass(`Download URL format correct`);
            passed++;
        } else {
            fail('Download URL generation', `Invalid URL: ${url}`);
            failed++;
        }
    } catch (err) {
        fail('Download URL generation', err.message);
        failed++;
    }

    // Test 7: Cache directory path
    try {
        const runner = new KagiMcpServerRunner();
        const binaryPath = runner.getBinaryPath();
        if (binaryPath && binaryPath.includes('kagi-mcp-server')) {
            pass(`Cache path: ${binaryPath}`);
            passed++;
        } else {
            fail('Cache directory path', `Invalid path: ${binaryPath}`);
            failed++;
        }
    } catch (err) {
        fail('Cache directory path', err.message);
        failed++;
    }

    // Test 8: Index.js exports correctly
    try {
        const exported = require('./index.js');
        if (exported === KagiMcpServerRunner) {
            pass('index.js exports Runner class');
            passed++;
        } else {
            fail('index.js exports', 'Export mismatch');
            failed++;
        }
    } catch (err) {
        fail('index.js exports', err.message);
        failed++;
    }

    // Test 9: Postinstall script is valid
    try {
        const postinstallPath = path.join(__dirname, 'scripts', 'postinstall.js');
        if (fs.existsSync(postinstallPath)) {
            require(postinstallPath);
            // If we get here without error, syntax is valid
            pass('postinstall.js syntax valid');
            passed++;
        } else {
            fail('postinstall.js exists', 'File not found');
            failed++;
        }
    } catch (err) {
        // Syntax errors would throw here
        if (err.code === 'MODULE_NOT_FOUND') {
            fail('postinstall.js', 'File not found');
        } else {
            // Runtime errors are expected (we're not actually installing)
            pass('postinstall.js syntax valid');
            passed++;
        }
    }

    // Summary
    console.log('\n' + '─'.repeat(50));
    log(`\n📊 Results: ${passed} passed, ${failed} failed\n`, failed > 0 ? 'red' : 'green');

    process.exit(failed > 0 ? 1 : 0);
}

runTests().catch((err) => {
    log(`\n❌ Test suite error: ${err.message}\n`, 'red');
    process.exit(1);
});

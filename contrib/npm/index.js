/**
 * Kagi MCP Server - NPX Wrapper
 *
 * This module provides an NPX-compatible wrapper for the Kagi MCP Server,
 * enabling easy installation and execution via npx.
 *
 * The wrapper automatically downloads the correct platform-specific binary
 * from GitHub releases and caches it for subsequent runs.
 */

const KagiMcpServerRunner = require("./bin/kagi-mcp-server.js");

/**
 * Export the main runner class for programmatic usage
 */
module.exports = KagiMcpServerRunner;

/**
 * CLI entry point when run directly
 */
if (require.main === module) {
  console.log("ℹ️  Note: For NPX usage, use: npx kagi-mcp-server");
  console.log("   Running directly from index.js...");
  console.log("");

  const runner = new KagiMcpServerRunner();
  runner.run();
}

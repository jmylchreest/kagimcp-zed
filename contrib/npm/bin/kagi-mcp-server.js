#!/usr/bin/env node

/**
 * Kagi MCP Server - NPX Binary Wrapper
 *
 * This script downloads and runs the appropriate platform-specific binary
 * for the Kagi MCP Server, enabling easy installation via npx.
 */

const { spawn, execSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const os = require("os");
const https = require("https");
const { createWriteStream, createReadStream } = require("fs");
const { pipeline } = require("stream/promises");
const zlib = require("zlib");

// Configuration
const GITHUB_REPO = "jmylchreest/kagimcp-zed";
const BINARY_NAME = "kagi-mcp-server";

class KagiMcpServerRunner {
  constructor() {
    this.platform = os.platform();
    this.arch = os.arch();
    this.cacheDir = path.join(os.homedir(), ".cache", "kagi-mcp-server");
    this.packageVersion = require("../package.json").version;
  }

  /**
   * Get the platform identifier for the binary name
   */
  getPlatformId() {
    // Platform names must match the release workflow artifact naming
    const platformMap = {
      darwin: "darwin",
      linux: "linux",
      win32: "windows-msvc", // Windows builds use MSVC by default
    };

    const archMap = {
      x64: "x86_64",
      arm64: "arm64",
    };

    const platform = platformMap[this.platform];
    const arch = archMap[this.arch];

    if (!platform || !arch) {
      throw new Error(`Unsupported platform: ${this.platform}-${this.arch}`);
    }

    return { platform, arch };
  }

  /**
   * Get the expected binary filename based on platform
   */
  getBinaryFilename() {
    const { platform, arch } = this.getPlatformId();
    const ext = this.platform === "win32" ? ".exe" : "";
    return `${BINARY_NAME}_${platform}_${arch}${ext}`;
  }

  /**
   * Get the download URL for the binary
   */
  getDownloadUrl(version) {
    const filename = this.getBinaryFilename();
    // Release workflow uses .tgz for unix, .zip for windows
    const ext = this.platform === "win32" ? ".zip" : ".tgz";
    return `https://github.com/${GITHUB_REPO}/releases/download/v${version}/${filename}${ext}`;
  }

  /**
   * Get the path where the binary should be cached
   */
  getBinaryPath() {
    const ext = this.platform === "win32" ? ".exe" : "";
    return path.join(
      this.cacheDir,
      this.packageVersion,
      `${BINARY_NAME}${ext}`,
    );
  }

  /**
   * Check if binary exists and is executable
   */
  binaryExists() {
    const binaryPath = this.getBinaryPath();
    try {
      fs.accessSync(binaryPath, fs.constants.X_OK);
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Follow redirects and download file
   */
  async downloadFile(url, destPath) {
    return new Promise((resolve, reject) => {
      const download = (url, redirectCount = 0) => {
        if (redirectCount > 5) {
          reject(new Error("Too many redirects"));
          return;
        }

        https
          .get(url, (response) => {
            // Handle redirects
            if (
              response.statusCode >= 300 &&
              response.statusCode < 400 &&
              response.headers.location
            ) {
              download(response.headers.location, redirectCount + 1);
              return;
            }

            if (response.statusCode !== 200) {
              reject(
                new Error(`Failed to download: HTTP ${response.statusCode}`),
              );
              return;
            }

            const file = createWriteStream(destPath);
            response.pipe(file);

            file.on("finish", () => {
              file.close();
              resolve();
            });

            file.on("error", (err) => {
              fs.unlink(destPath, () => {});
              reject(err);
            });
          })
          .on("error", reject);
      };

      download(url);
    });
  }

  /**
   * Extract tar.gz archive
   */
  async extractTarGz(archivePath, destDir) {
    return new Promise((resolve, reject) => {
      // Use tar command for extraction (available on macOS and Linux)
      try {
        execSync(`tar -xzf "${archivePath}" -C "${destDir}"`, {
          stdio: "pipe",
        });
        resolve();
      } catch (err) {
        reject(new Error(`Failed to extract archive: ${err.message}`));
      }
    });
  }

  /**
   * Extract zip archive (Windows)
   */
  async extractZip(archivePath, destDir) {
    return new Promise((resolve, reject) => {
      try {
        // Use PowerShell for extraction on Windows
        execSync(
          `powershell -command "Expand-Archive -Path '${archivePath}' -DestinationPath '${destDir}' -Force"`,
          { stdio: "pipe" },
        );
        resolve();
      } catch (err) {
        reject(new Error(`Failed to extract archive: ${err.message}`));
      }
    });
  }

  /**
   * Download and install the binary
   */
  async installBinary() {
    const version = this.packageVersion;
    const versionDir = path.join(this.cacheDir, version);
    const binaryPath = this.getBinaryPath();

    // Create cache directory
    fs.mkdirSync(versionDir, { recursive: true });

    const downloadUrl = this.getDownloadUrl(version);
    const isZip = this.platform === "win32";
    const archiveExt = isZip ? ".zip" : ".tgz";
    const archivePath = path.join(versionDir, `download${archiveExt}`);

    console.error(`📥 Downloading Kagi MCP Server v${version}...`);
    console.error(`   URL: ${downloadUrl}`);

    try {
      await this.downloadFile(downloadUrl, archivePath);
    } catch (err) {
      throw new Error(
        `Failed to download binary: ${err.message}\n   URL: ${downloadUrl}`,
      );
    }

    console.error("📦 Extracting...");

    try {
      if (isZip) {
        await this.extractZip(archivePath, versionDir);
      } else {
        await this.extractTarGz(archivePath, versionDir);
      }
    } catch (err) {
      throw new Error(`Failed to extract archive: ${err.message}`);
    }

    // Find and rename the extracted binary
    const extractedBinaryName = this.getBinaryFilename();
    const extractedPath = path.join(versionDir, extractedBinaryName);

    if (fs.existsSync(extractedPath)) {
      fs.renameSync(extractedPath, binaryPath);
    } else {
      // Binary might be directly extracted without platform suffix
      const simpleName =
        this.platform === "win32" ? `${BINARY_NAME}.exe` : BINARY_NAME;
      const simplePath = path.join(versionDir, simpleName);
      if (fs.existsSync(simplePath) && simplePath !== binaryPath) {
        fs.renameSync(simplePath, binaryPath);
      }
    }

    // Make binary executable (Unix)
    if (this.platform !== "win32") {
      fs.chmodSync(binaryPath, 0o755);
    }

    // Clean up archive
    try {
      fs.unlinkSync(archivePath);
    } catch {
      // Ignore cleanup errors
    }

    console.error("✅ Installation complete!");
    console.error("");
  }

  /**
   * Run the binary with the given arguments
   */
  runBinary(args) {
    const binaryPath = this.getBinaryPath();

    const child = spawn(binaryPath, args, {
      stdio: "inherit",
      env: process.env,
    });

    child.on("error", (err) => {
      console.error(`❌ Failed to start Kagi MCP Server: ${err.message}`);
      process.exit(1);
    });

    child.on("exit", (code, signal) => {
      if (signal) {
        process.kill(process.pid, signal);
      } else {
        process.exit(code || 0);
      }
    });

    // Forward signals to child process
    process.on("SIGINT", () => child.kill("SIGINT"));
    process.on("SIGTERM", () => child.kill("SIGTERM"));
  }

  /**
   * Show help message
   */
  showHelp() {
    console.log(`
Kagi MCP Server - NPX Wrapper
=============================

This is an NPX wrapper that downloads and runs the Kagi MCP Server binary.

Usage:
  npx kagi-mcp-server [options]

The wrapper will automatically download the correct binary for your platform
on first run. Subsequent runs use the cached binary.

Options are passed through to the Kagi MCP Server binary:
  --api-key KEY           Kagi API key (or set KAGI_API_KEY env var)
  --summarizer-engine EN  Summarizer engine (cecil, agnes, daphne, muriel)
  --help                  Show this help message

Environment Variables:
  KAGI_API_KEY            Your Kagi API key

Example Claude Desktop Configuration:
{
  "mcpServers": {
    "kagi": {
      "command": "npx",
      "args": ["-y", "kagi-mcp-server"],
      "env": {
        "KAGI_API_KEY": "your-api-key-here"
      }
    }
  }
}

For more information, visit:
  https://github.com/jmylchreest/kagimcp-zed
`);
  }

  /**
   * Main entry point
   */
  async run() {
    const args = process.argv.slice(2);

    // Check for wrapper-specific help
    if (args.includes("--wrapper-help")) {
      this.showHelp();
      return;
    }

    try {
      // Download binary if needed
      if (!this.binaryExists()) {
        await this.installBinary();
      }

      // Run the binary
      this.runBinary(args);
    } catch (err) {
      console.error(`❌ Error: ${err.message}`);
      console.error("");
      console.error("💡 Troubleshooting:");
      console.error("   1. Check your internet connection");
      console.error("   2. Verify the release exists at:");
      console.error(`      https://github.com/${GITHUB_REPO}/releases`);
      console.error("   3. Try clearing the cache:");
      console.error(`      rm -rf ${this.cacheDir}`);
      process.exit(1);
    }
  }
}

// Run the wrapper
if (require.main === module) {
  const runner = new KagiMcpServerRunner();
  runner.run();
}

module.exports = KagiMcpServerRunner;

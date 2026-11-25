#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const os = require('os');
const { spawnSync } = require('child_process');
const https = require('https');

const VERSION = '0.1.1';
const REPO = 'Ankara-Software/context-squeeze';

function getTarget() {
    const platform = os.platform();
    const arch = os.arch();

    if (platform === 'linux') {
        if (arch === 'x64') return { asset: 'context-squeeze-linux-x64', bin: 'context-squeeze' };
        if (arch === 'arm64') return { asset: 'context-squeeze-linux-arm64', bin: 'context-squeeze' };
    } else if (platform === 'darwin') {
        if (arch === 'x64') return { asset: 'context-squeeze-darwin-x64', bin: 'context-squeeze' };
        if (arch === 'arm64') return { asset: 'context-squeeze-darwin-arm64', bin: 'context-squeeze' };
    } else if (platform === 'win32') {
        if (arch === 'x64') return { asset: 'context-squeeze-win32-x64.exe', bin: 'context-squeeze.exe' };
    }
    return null;
}

function downloadFile(url, destPath) {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(destPath);

        const request = (targetUrl) => {
            https.get(targetUrl, (response) => {
                if (response.statusCode === 301 || response.statusCode === 302) {
                    request(response.headers.location);
                    return;
                }

                if (response.statusCode !== 200) {
                    file.close(() => {
                        fs.unlink(destPath, () => {});
                        reject(new Error(`Failed to download binary: HTTP ${response.statusCode}`));
                    });
                    return;
                }

                response.pipe(file);

                file.on('finish', () => {
                    file.close(resolve);
                });
            }).on('error', (err) => {
                file.close(() => {
                    fs.unlink(destPath, () => {});
                    reject(err);
                });
            });
        };

        request(url);
    });
}

function main() {
    const target = getTarget();
    if (!target) {
        console.error(`Error: Unsupported platform/architecture (${os.platform()}/${os.arch()}).`);
        console.error("context-squeeze-mcp supports 64-bit Linux, macOS, and Windows.");
        process.exit(1);
    }

    const homeDir = os.homedir();
    const installDir = path.join(homeDir, '.context-squeeze', `v${VERSION}`);
    const binPath = path.join(installDir, target.bin);

    if (fs.existsSync(binPath)) {
        runBinary(binPath);
        return;
    }

    console.error(`context-squeeze binary not found locally. Downloading v${VERSION} (${target.asset})...`);

    if (!fs.existsSync(installDir)) {
        fs.mkdirSync(installDir, { recursive: true });
    }

    const downloadUrl = `https://github.com/${REPO}/releases/download/v${VERSION}/${target.asset}`;

    downloadFile(downloadUrl, binPath)
        .then(() => {
            if (os.platform() !== 'win32') {
                try {
                    fs.chmodSync(binPath, 0o755);
                } catch (e) {}
            }

            console.error("Download complete.\n");
            runBinary(binPath);
        })
        .catch((err) => {
            console.error(`Error: Failed to install binary: ${err.message}`);
            process.exit(1);
        });
}

function runBinary(binaryPath) {
    const args = process.argv.slice(2);
    const result = spawnSync(binaryPath, args, { stdio: 'inherit' });
    process.exit(result.status ?? 0);
}

main();

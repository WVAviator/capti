#!/usr/bin/env node

const os = require("node:os");
const path = require("node:path");
const { spawn } = require("node:child_process");

const DIST_DIR = path.resolve(__dirname, "..", "dist");

const extension = os.platform() === "win32" ? ".exe" : "";
const binaryPath = path.resolve(DIST_DIR, `capti${extension}`);

spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
});

#!/usr/bin/env node

import os from "os";
import path from 'path';
import { spawn } from "child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const DIST_DIR = path.resolve(__dirname, "..", "dist");

const extension = os.platform() === "win32" ? ".exe" : "";
const binaryPath = path.resolve(DIST_DIR, `capti${extension}`);

spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
});

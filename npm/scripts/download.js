import { version } from '../package.json';
import os from 'os';
import fs from 'fs';

const BINARY_PATH = './bin/capti';
const SUPPORTED_PLATFORMS = ['linux', 'darwin', 'win32'];
const SUPPORTED_ARCHS = ['x64', 'arm64'];

const createLogger = () => {
  const date = new Date().toISOString();
  const filename = `./logs/${date}.log`;

  return (message) => {
    fs.appendFileSync(filename, `${message}\n`);
  }
}

const log = createLogger();

const download = async () => {
  const platform = os.platform();
  const arch = os.arch();

  log(`Detected platform: ${platform}, arch: ${arch}`);

  if (!SUPPORTED_PLATFORMS.includes(platform) || !SUPPORTED_ARCHS.includes(arch)) {
    log(`Unsupported platform or architecture: ${platform}, ${arch}`);
    console.error("Unsupported platform or architecture:", platform, arch);
    process.exit(1);
  }

  const binary = `capti-${platform}-${arch}`;

  log(`Downloading ${binary}`);

  const url = `https://github.com/wvaviator/capti/releases/download/${version}/${binary}`;

  log(`Downloading from ${url}`);

  try {

    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(response.statusText);
    }

    log("Downloaded successfully. Writing to binary file.");

    const buffer = await response.buffer();

    fs.writeFile(BINARY_PATH, buffer);
    fs.chmod(BINARY_PATH, 0o755);

    log(`Successfully downloaded to ${BINARY_PATH}`);
  } catch (error) {
    log(`Failed to download: ${error.message}`);
    console.error("Failed to download:", error.message);
    process.exit(1);
  }

}

download();

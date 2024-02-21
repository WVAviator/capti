import os from "os";
import fs from "fs";
import fsPromises from "fs/promises";
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const DIST_PATH = path.resolve(__dirname, "..", "dist");
const PACKAGE_JSON_PATH = path.resolve(__dirname, "..", "package.json");
const LOGS_PATH = path.resolve(__dirname, "..", "logs");

const SUPPORTED_ARCHITECTURE = {
  linux: ["x64", "arm64"],
  darwin: ["x64", "arm64"],
  win32: ["x64"],
};

const createLogger = () => {
  const date = new Date().toISOString().replace(/:/g, '-').replace(/\./g, '_');
  const filename = `${date}.log`;

  if (!fs.existsSync(LOGS_PATH)) {
    fs.mkdirSync(LOGS_PATH, { recursive: true });
  }

  const logPath = path.resolve(LOGS_PATH, filename);

  return (message) => {
    fs.appendFileSync(logPath, `${message}\n`);
  };
};

const log = createLogger();

const loadPackageJson = () => {
  const data = fs.readFileSync(PACKAGE_JSON_PATH, { encoding: "utf8" });
  return JSON.parse(data);
};

const { capti_version } = await loadPackageJson();
log(`Loaded version ${capti_version} from package.json`);

const download = async () => {
  const platform = os.platform();
  const arch = os.arch();

  log(`Detected platform: ${platform}, arch: ${arch}`);

  if (
    !SUPPORTED_ARCHITECTURE[platform].includes(arch)
  ) {
    log(`Unsupported platform and architecture: ${platform} ${arch}`);
    console.error("Unsupported platform and architecture:", platform, arch);
    process.exit(1);
  }

  const extension = platform === "win32" ? ".exe" : "";
  const binary = `capti-${platform}-${arch}${extension}`;

  log(`Downloading ${binary}`);

  const url = `https://github.com/WVAviator/capti/releases/download/${capti_version}/${binary}`;

  log(`Downloading from ${url}`);

  try {
    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(response.statusText);
    }

    log("Downloaded successfully. Extracting binary...");

    const arrayBuffer = await response.arrayBuffer();
    log("Array buffer loaded, converting to buffer...");

    const buffer = Buffer.from(arrayBuffer);
    log("Buffer created, writing to file...");

    if (!fs.existsSync(DIST_PATH)) {
      fs.mkdirSync(DIST_PATH, { recursive: true });
    }

    const binaryPath = path.resolve(DIST_PATH, `capti${extension}`);

    await fsPromises.writeFile(binaryPath, buffer);
    log("Buffer written to file, setting permissions...");

    await fsPromises.chmod(binaryPath, 0o755);
    log("Permissions set.");

    log(`Successfully downloaded to ${binaryPath}`);
  } catch (error) {
    log(`Failed to download: ${error.message}`);
    console.error("Failed to download/install:", error.message);
    process.exit(1);
  }
};

download();

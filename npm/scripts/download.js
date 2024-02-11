import os from "os";
import fs from "fs";
import fsPromises from "fs/promises";

const BINARY_PATH = "./bin/capti";
const SUPPORTED_PLATFORMS = ["linux", "darwin", "win32"];
const SUPPORTED_ARCHS = ["x64", "arm64"];

const createLogger = () => {
  const date = new Date().toISOString();
  const filename = `./logs/${date}.log`;

  return (message) => {
    fs.appendFileSync(filename, `${message}\n`);
  };
};

const log = createLogger();

const loadPackageJson = () => {
  const data = fs.readFileSync("package.json", { encoding: "utf8" });
  return JSON.parse(data);
};

const { version } = await loadPackageJson();
log(`Loaded version ${version} from package.json`);

const download = async () => {
  const platform = os.platform();
  const arch = os.arch();

  log(`Detected platform: ${platform}, arch: ${arch}`);

  if (
    !SUPPORTED_PLATFORMS.includes(platform) ||
    !SUPPORTED_ARCHS.includes(arch)
  ) {
    log(`Unsupported platform or architecture: ${platform}, ${arch}`);
    console.error("Unsupported platform or architecture:", platform, arch);
    process.exit(1);
  }

  const binary = `capti-${platform}-${arch}`;

  log(`Downloading ${binary}`);

  const url = `https://github.com/WVAviator/capti/releases/download/${version}/${binary}`;

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

    // fs.createWriteStream(BINARY_PATH).write(buffer);
    await fsPromises.writeFile(BINARY_PATH, buffer);
    log("Buffer written to file, setting permissions...");

    await fsPromises.chmod(BINARY_PATH, 0o755);
    log("Permissions set.");

    log(`Successfully downloaded to ${BINARY_PATH}`);
  } catch (error) {
    log(`Failed to download: ${error.message}`);
    console.error("Failed to download/install:", error.message);
    process.exit(1);
  }
};

download();

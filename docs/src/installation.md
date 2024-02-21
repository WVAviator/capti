# Installation

There are currently a few different ways you can install Capti.
- [Globally with NPM](#global-npm-install)
- [Locally with NPM for NodeJS Projects](#local-npm-install)
- [Anywhere else by downloading the binary executable](#binary-executable)

## Global NPM Install

Installing globally is the easiest way to get started with Capti if you already have Node/NPM installed.

```bash
$ npm install -g capti
```

This will make Capti available for you to use in any project at the command line. To verify that your installation succeeded, you can run:

```bash
$ capti --version
```

> Note: If your installation did not succeed, please [report the issue](reporting_issues.md#npm-installation) so that any necessary fixes can be made.

## Local NPM Install

Installing Capti locally in an NPM project is useful when you share your project with other developers and don't want them to have to globally install anything. It's also useful if you plan to use Capti for continuous integration in your project.

To install locally in an NPM project, cd into your project directory and run:

```bash
$ npm install --save-dev capti
```

> Note: If your installation did not succeed, please [report the issue](reporting_issues.md#npm-installation) so that any necessary fixes can be made.

This will save Capti as a development dependency (meaning that it won't bundle into your final build). To use Capti in your project, first create a new folder in your project directory `tests/` (you can name it whatever you want). Then open your package.json and add the following script:

```json
{
    "scripts": {
        "test:capti": "capti --path ./tests"
    }
}
```

The to run the tests your project, all you need to do is run:

```bash
$ npm run test:capti
```

## Binary Executable

If you want to install Capti on the command line but you do not want to use NPM or don't have Node installed, you can download the binary executable for your platform/architecture and manually add it to your `PATH`.

To access the binary executable downloads, head over to the project's [GitHub Releases](https://github.com/WVAviator/capti/releases) and download the latest version.

> Note: If you don't see your platform/architecture as a download option, and you think it should be available, feel free to put in an [enhancement issue](reporting_issues.md#enhancements) and we can look at possibly adding support for your architecture.

Instructions for adding the binary to your PATH environment variable varies system-to-system. Please expand the section below for your platform.

<details>

<summary>Unix (Linux / MacOS)</summary>

1. Start by moving the downloaded binary to your `usr/local/bin` directory.

```bash
$ mv capti /usr/local/bin/capti
```

2. Open your shell config file `.bash-profile` if you use bash, or `.zshrc` if you use zsh (default on MacOS) using your favorite text editor.

3. Add the following line to the end of the file:

```
export PATH="/usr/local/bin/capti:$PATH"
```

4. Restart your shell, and you should be good to go. You can verify by running:

```bash
$ capti --version
```

</details>


<details>

<summary>Windows</summary>

1. Move the binary to a convenient location. I recommend `C:/Program Files/Capti/capti.exe`.

2. Right-click on the Start button > System > About > Advanced system settings > Environment Variables

3. Edit the PATH Variable:

- In the Environment Variables window, under "System variables" (for all users) or "User variables" (for the current user only), find and select the PATH variable, then click Edit.
- In the Edit Environment Variable window, click New and add the path to the folder that contains your binary. For example, C:\Program Files\MyBinary.
- Click OK to close each window.

4. Restart any open command prompts or applications.

</details>


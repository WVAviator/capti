# Config File

Capti config files enable you to set some settings that apply to all your test suites and infuence how test suites are processed. Many of the same configuration options are available on a per-suite basis as well. One useful component of the configuration is specifying scripts that should run before and after your tests - for example, starting your server.

## Setup

To create a config file, simply include a file named `capti-config.yaml` in your tests folder. This will automatically be parsed as a configuration for Capti.

### Custom Config

If you would instead prefer to name your config differently, or include the config in a location separate from your tests, you can specify the `--config` or `-c` argument when running Capti. For example, say you want to keep your config file in a separate directory in your project:

```
.
├── src/
│   └── index.ts
├── tests/
│   └── hello.yaml
├── config/
│   └── capti.yaml
└── .gitignore
```

You can configure your script to run 

```bash
$ capti --path ./tests --config ./config/capti.yaml
```
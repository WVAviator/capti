# Setup Scripts

If you would like Capti to run commands or scripts before executing tests, whether for continuous integration workflows or just for convenience, you can specify scripts to run and optional `wait_until` parameter to determine when to continue with executing your tests or additional scripts.

## Adding Scripts

Setup scripts should be listed in sequential order under `before_all` or `after_all` in your config file. 

```yaml
# tests/capti-config.yaml

setup:
  before_all:
    - script: npm start
      description: start server
      wait_until: output 'Server running on port 3000'
```

You can also additionally include `before_each` and `after_each` scripts in your individual test suites. Keep in mind that config-level scripts will all execute first.

```yaml
# tests/hello.yaml
suite: /hello endpoint tests
description: tests the various HTTP methods of the /hello endpoint
setup:
  before_all:
    - script: echo 'starting hello suite'
  before_each:
    - script: ./scripts/reset-test-db.sh
      desription: reset test db
      wait_until: finished
```

## Wait Until Options

There are a few different options to choose from when deciding how to wait for your scripts to finish. By default, if `wait_until` is not included, execution will immediately continue with your script running in the background. This is not always what you want - for example when starting a server, you need to give it time to fully spin up before you start testing its endpoints.

- `wait_until: finished` - This executes the command/script/program and waits synchronously for it to finish before proceeding.
- `wait_until: 5 seconds` - This executes the script and then waits for the specified number of seconds before continuing.
- `wait_until: port 3000` - This executes the script and waits for the specified port to open. If the port already has an open connection, the script will not execute.
- `wait_until: output 'Server listening on port 3000` - This executes the script and then waits for the specified console output from your server. This is useful in some cases where the port may be open but the server is still not quite ready to take requests.

## Examples

Here is a simple cross-platform script to start a server and check that the port connection is open before proceeding.

```yaml
setup:
  before_all:
    - description: start app server
      script: NODE_ENV=test && npm start
      wait_until: port 3000
```

Here is an example from a project that uses Docker Compose to spin up both a database and a server. This Unix script checks if Docker Compose is already running, and if not - starts it. If it is already started, the `wait_until` output is still detected because of the call to `echo` the same output text. Note - make sure you update the output text to match your server's log message when it becomes ready.

```yaml
setup:
  before_all:
    - description: "Start db and server"
      wait_until: output "Listening on 3000"
      script: >
        if ! docker-compose ps | grep -q " Up "; then
            docker-compose up
        else
            echo "Listening on 3000"
        fi
```

## Considerations

Running setup scripts is entirely optional and merely provides a convenience for development. It is not necessary and you may instead choose to start your server manually first and then run Capti.

> Why doesn't Capti just integrate directly with the server?
> The goal of Capti is to provide the convenience of platform-agnostic test suites to run with your project, without directly coupling with your server (and behaving more like a user). If you want a framework that more tightly integrates with your server, you can look into a tool like supertest for NodeJS or MockMvc with Java/Spring.
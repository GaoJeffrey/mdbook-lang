# firejail Configuration

## Adding Whitelists

If your deployed Python programming environment is not in the system path but under `${HOME}`, in addition to adding it to the `PATH` environment variable in `$HOME/.bash`, you also need to install the required packages:

- If you use the system Python environment and install packages as a non-root user, the extra packages will be installed at: `${HOME}/.local/lib/python3.10/site-packages`

```bash
$ python -m pip install numpy
```

You need to add the following statement at the end of your firejail configuration file (note the Python version number):
```bash
whitelist ${HOME}/.local/lib/python3.10/site-packages
```

## Adding Compiler Whitelists

If you have installed other compilers, add the following to your firejail configuration file:
```bash
private-bin your-compiler-file-name
```
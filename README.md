```text
bitburner-oxide 0.1.0
Chris Dawkins

Bitburner-oxide will automatically push modified or created script files to a running Bitburner game server.
If ran from the same directory as the scripts the --directory flag is not needed.
All managed files must exists in the top level directory; bitburner-oxide does not manage nested folders.
Bitburner-oxide does not currently support deleting, or pulling files from the game server.
Source for bitburner-oxide can be found at https://www.gitlab.com/xsiph/bitburner-oxide

USAGE:
    bitburner-oxide [OPTIONS] --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --directory <directory>    base directory to look for files
    -t, --token <token>            auth token from game context menu
```

# Bitburner-oxide

Bitburner-oxide is an editor-agnostic file manager for the game [Bitburner](https://github.com/danielyxie/bitburner).
Bitburner-oxide allows you to edit scripts externally from the game.  

The purpose of Bitburner-oxide is to fill the void for other editors (vim, emacs, sublime text) that [bitburner-vscode](https://github.com/bitburner-official/bitburner-vscode) fills for vscode; although bitburner-oxide also works with vscode.  

```text
Bitburner-oxide will automatically push modified or created script files to a running Bitburner game server.

If ran from the same directory as the scripts the --directory flag is not needed.
All managed files must exist in the top level directory; bitburner-oxide does not manage nested folders.

Authentication is done by passing in the bearer-token via --token. 
Alternatively, the bearer-token can be placed in a file named 'token' in the chosen directory.

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
## Build Instructions
### Linux
```bash
$ git clone 'https://gitlab.com/xsiph/bitburner-oxide.git'
```
```bash
# To add to PATH and call directly from the command line.
$ cargo build --release && cp target/release/bitburner-oxide ~/.local/bin/
```

### Mac / Windows
I have no idea if this works on Mac or Windows. There is nothing platform specific in the code, so I assume it should?  
If not, try docker.

# Docker
## Build
```bash
$ git clone 'https://gitlab.com/xsiph/bitburner-oxide.git'
```
```bash
$ docker build -t xsiph/bitburner-oxide .
```
## Run
```bash
# run from directory with script files
$ docker run --network host --rm -v "$PWD:$PWD" -w "$PWD" xsiph/bitburner-oxide -t '<bearer-token>'
```

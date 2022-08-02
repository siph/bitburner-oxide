# Bitburner-oxide
Bitburner-oxide is an editor-agnostic file manager for the game [Bitburner](https://github.com/danielyxie/bitburner).
Bitburner-oxide allows you to synchronize the scripts edited on your computer with the scripts inside the game.
Bitburner-oxide will watch for the creation, modification, or deletion of files within the chosen directory and its
child directories. Upon detection of these events, Bitburner-oxide will update the Bitburner game files to reflect
the changes made to the files and directories within the chosen directory.
The purpose of Bitburner-oxide is to fill the void for other editors (vim, emacs, sublime text, etc.) that
[bitburner-vscode](https://github.com/bitburner-official/bitburner-vscode) fills for vscode, although bitburner-oxide
also works with vscode.

## How To Use
Bitburner-oxide is ran using the command line.

### Running Bitburner-oxide With Arguments
Bitburner-oxide can be ran from any location by supplying the optional arguments.
```bash
bitburner-oxide --bearer-token <token-from-bitburner> --directory <path/to/directory>
```

### Running Bitburner-oxide Without Arguments
To run bitburner-oxide without arguments you must choose a directory that will contain your managed scripts. Inside of
that folder you can take the bitburner token and place it within a file named 'token'.
```bash
# From inside of the directory with the 'token' file
bitburner-oxide
```

## What Bitburner-oxide Synchronizes
Bitburner-oxide takes files from a chosen directory and one-way synchronizes those files with the files inside of the
game Bitburner. Bitburner-oxide does not work in the opposite direction of game files -> chosen directory files.
Bitburner-oxide will only synchronize files that end with a relevant extension: [.js, .ns, .script, etc].

### Will Be Synchronized
 - Files that are created, modified, or deleted within the chosen directory.
 - Files that are moved or renamed within the chosen directory.
 - The above actions performed within nested folders under the chosen directory.

### Won't Be Synchronized
 - Files that are modified or created using the in-game editor.
 - Files that exist above the chosen directory.
 - Files that do not contain relevant extensions.

## Build Instructions
### Linux
```bash
$ git clone 'https://gitlab.com/xsiph/bitburner-oxide.git'
```
```bash
# To add to PATH and call directly from the command line.
$ cargo build --release && cp target/release/bitburner-oxide ~/.local/bin/
```

### Nix with Flakes
```bash
# From repository
$ nix run -- -t '<bearer-token>'
```
```bash
# From anywhere
$ nix run gitlab:xsiph/bitburner-oxide -- -t '<bearer-token>'
```

### Mac / Windows
I have no idea if this works on Mac or Windows. There is nothing platform specific in the code, so I assume it should?
If not, try docker.

## Docker
### Build
```bash
$ git clone 'https://gitlab.com/xsiph/bitburner-oxide.git'
```
```bash
$ docker build -t xsiph/bitburner-oxide .
```

### Run
```bash
# run from directory with script files
$ docker run --network host --rm -v "$PWD:$PWD" -w "$PWD" xsiph/bitburner-oxide -t '<bearer-token>'
```

# Recursive-git
by Franklin Blanco

# English-US
## Why?
I was tired of having a bunch of shell scripts and always thought git should've had an option to run commands recursively. At least git pull.

## How to install
`cargo install recursive-git`

## Requirements 
- Git

## Usage
- Get inside the workspace/parent directory you want. 
- Run any of the commands (`recursive-git <COMMAND>`)
- Understand this only goes one layer deep in the tree of subdirectories.

# SPANISH-DO

## Como instalar
`cargo install recursive-git`

## Requerimientos 
- Git

## Como lo uso?
- Colocate dentro de un workspace o un folder donde tengas todos tus repositorios.
- Corre el commando que quieres (`recursive-git pull` | `recursive-git push` | `recursive-git commit <"MENSAJE">`)
- Esto solo irá haciendo los comandos a 1 nivel, solo los directorios que esten dentro del directorio que corras este comando.
# Recursive-git
by Franklin Blanco

## Como instalar?
- Descargate este repositorio.
- Coloca tu terminal dentro de el `cd recursive-git`
- Corre `cargo install --path .`

Listo!

## Como lo uso?
- Colocate dentro de un workspace o un folder donde tengas todos tus repositorios.
- Corre el commando que quieres (`recursive-git pull` | `recursive-git push` | `recursive-git commit <"MENSAJE">`)
- Esto solo irá haciendo los comandos a 1 nivel, solo los directorios que esten dentro del directorio que corras este comando.
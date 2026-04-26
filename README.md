# dict-rae

Consulta el diccionario de la Real Academia de la Lengua Española desde tu terminal 📚.

## Uso

Pasa como argumento la palabra que quieres consultar:

```
dict-rae amar

Palabra: amar

1. Tener amor a alguien o algo
     Sin: querer, adorar, idolatrar, estimar, apreciar, camelar
     Ant: odiar, abominar, aborrecer, detestar, desamar
2. desear

```
## Instalación

Clona el repositorio y compila en modo release:

```bash
git clone https://github.com/tu-usuario/dict-rae.git
cd dict-rae
cargo build --release
mv target/release/dict-rae ~/.local/bin/
```

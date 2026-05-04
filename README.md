# dict-rae

Consulta el diccionario de la Real Academia de la Lengua Española desde tu terminal 📚.

![demo](./assets/demo.gif)

dict-rae es una herramienta de línea de comandos que te permite consultar el diccionario de la Real Academia Española directamente desde tu terminal. Proporciona definiciones, significados y ejemplos de uso en una interfaz rápida y sin distracciones, con soporte opcional para colores ANSI y un modo minimalista pensado para scripting o flujos de trabajo más eficientes.

Ideal para:
- Estudiantes y escritores de español — quienes necesitan definiciones rápidas y fiables mientras trabajan en la terminal.
- Desarrolladores y entusiastas de la CLI — usuarios que prefieren flujos de trabajo basados en teclado y no quieren salir de su entorno.

## Características

- Acceso directo a la RAE — Consulta el diccionario oficial de la Real Academia Española al instante, sin usar el navegador.
- Salida amigable para scripts — El modo minimalista produce resultados limpios y fáciles de procesar, ideales para scripts.
- Visualización flexible — Colores ANSI configurables para una salida legible en cualquier entorno.

## Uso

Pasa como argumento la palabra que quieres consultar:

```
dict-rae amar

Significado 1
    Del lat. amāre.
1. Tener amor a alguien o algo
Sin querer1, adorar, idolatrar, estimar, apreciar, camelar
Ant odiar, abominar, aborrecer, detestar, desamar
2. desear

```
## Instalación

Usando `Cargo`:

```bash
cargo install --git https://github.com/cyberSapoPerro/dict-rae.git
```

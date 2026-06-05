# Sintaxis de alto nivel (MVP)

Este documento define la **sintaxis mínima** para iniciar el desarrollo del core.

- Se ejecuta **línea por línea** (estilo REPL).
- Cada comando se procesa de inmediato y actualiza el estado.

Todo lo que no sea necesario para el MVP está listado en `docs/timeline.md`.

## Comandos (MVP)

### JUGADOR
```
JUGADOR "Nombre"
```
Agrega un jugador.

### EMPAREJAR
```
EMPAREJAR
```
Genera enfrentamientos **TODOS_CONTRA_TODOS** (round-robin simple):
- Cada par de jugadores se enfrenta una vez.
- El orden de los partidos puede ser aleatorio.

### VER
```
VER <opción>
```
Muestra información basada en la opción:
- `JUGADORES` — muestra la lista de todos los jugadores agregados.
- `GRUPOS` — muestra los grupos/enfrentamientos generados con `EMPAREJAR`.

### RESULTADO
```
RESULTADO "JugadorA" vs "JugadorB" A-B
```
Registra el resultado de un partido.

Reglas MVP:
- `A` y `B` son enteros >= 0.
- No se permite empate (si `A == B`, es error) para simplificar el core.

### TABLA
```
TABLA
```
Calcula y muestra la tabla.

Regla MVP de puntos:
- Victoria = 1 punto
- Derrota = 0 puntos

### GANADOR
```
GANADOR
```
Muestra el/los líder(es) actual(es) según la tabla:
- Si hay un líder único, se imprime ese jugador.
- Si hay empate en el primer lugar, se imprime la lista de líderes (sin desempates en MVP).

## Ejemplo completo (MVP)

```
JUGADOR "Ana"
JUGADOR "Luis"
JUGADOR "Marta"
JUGADOR "Carlos"

EMPAREJAR

VER JUGADORES
VER GRUPOS

RESULTADO "Ana" vs "Luis" 3-1
RESULTADO "Marta" vs "Carlos" 2-3

TABLA
GANADOR
```

## Gramática (EBNF simple, MVP)

```
linea      := comando ;
comando    := jugador | emparejar | ver | resultado | tabla | ganador ;

jugador    := "JUGADOR" cadena ;
emparejar  := "EMPAREJAR" ;
ver        := "VER" opcion ;
resultado  := "RESULTADO" cadena "vs" cadena int "-" int ;
tabla      := "TABLA" ;
ganador    := "GANADOR" ;

opcion     := "JUGADORES" | "GRUPOS" ;
cadena     := string_entre_comillas ;
int        := numero_entero ;
```

# Bytecode y VM (MVP)

Este documento describe el **bytecode mínimo** y la VM (máquina de pila) necesarios para el MVP.

La sintaxis de comandos está en `docs/sintaxis.md`. La lista de mejoras post‑MVP está en `docs/timeline.md`.

## Modelo de ejecución

- Cada línea se tokeniza/parsea y se compila a bytecode.
- La VM ejecuta ese bytecode inmediatamente y actualiza el estado.
- En CLI, `TABLA` y `GANADOR` imprimen salida.

## Estado mínimo en memoria (MVP)

- `jugadores`: lista de strings
- `partidos`: lista de pares (A, B)
- `resultados`: mapa (A,B) -> (a,b)

Derivados:
- `tabla`: se calcula bajo demanda desde `resultados`.

## Emparejamiento (MVP)

`EMPAREJAR` genera un round-robin simple:

- Para N jugadores, genera N*(N-1)/2 partidos.
- Orden: puede barajarse (RNG) para variar el calendario.

Nota: no hay semilla en MVP; la reproducibilidad queda para post‑MVP.

## Instrucciones de bytecode (stack VM) — MVP

- `PushNum <n>` — push número a stack
- `PushStr <idx>` — push índice string a stack (string lookup en VM.strings)
- `AddPlayer` — pop nombre de stack, agregar a estado torneo
- `Pop` — descartar tope de stack
- `Print` — imprimir tope de stack
- `Eoc` — fin comando

**Pendiente MVP** (post-MVP):
- `EMPAREJAR_GEN` (genera `partidos`)
- `RESULTADO_ADD` (pop: scoreB, scoreA, jugadorB, jugadorA)
- `TABLA_CALC`, `TABLA_PRINT`
- `GANADOR_CALC`, `GANADOR_PRINT`

## Mapeo comando -> bytecode (MVP)

**JUGADOR**
```
JUGADOR "Ana"
=> PushStr "Ana"; AddPlayer
```

**EMPAREJAR**
```
EMPAREJAR
=> EMPAREJAR_GEN
```

**RESULTADO**
```
RESULTADO "Ana" vs "Luis" 3-1
=> PushStr "Ana"; PushStr "Luis"; PushInt 3; PushInt 1; AddResult
```

**TABLA**
```
TABLA
=> TABLA_CALC; TABLA_PRINT
```

**GANADOR**
```
GANADOR
=> GANADOR_CALC; GANADOR_PRINT
```

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

## Instrucciones de bytecode (stack VM)

- `PUSH_STR <s>`
- `PUSH_INT <n>`

- `JUGADOR_ADD` (pop: name)

- `EMPAREJAR_GEN` (genera `partidos`)

- `RESULTADO_ADD` (pop: scoreB, scoreA, jugadorB, jugadorA)

- `TABLA_CALC` (calcula tabla y la deja en un registro interno)
- `TABLA_PRINT` (imprime tabla calculada)

- `GANADOR_CALC` (calcula líder/es)
- `GANADOR_PRINT` (imprime líder/es)

## Mapeo comando -> bytecode (MVP)

**JUGADOR**
```
JUGADOR "Ana"
=> PUSH_STR "Ana"; JUGADOR_ADD
```

**EMPAREJAR**
```
EMPAREJAR
=> EMPAREJAR_GEN
```

**RESULTADO**
```
RESULTADO "Ana" vs "Luis" 3-1
=> PUSH_STR "Ana"; PUSH_STR "Luis"; PUSH_INT 3; PUSH_INT 1; RESULTADO_ADD
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

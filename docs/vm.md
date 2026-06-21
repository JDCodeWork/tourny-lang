# Bytecode y VM (MVP)

Este documento describe el **bytecode mínimo** y la VM (sin stack) necesarios para el MVP.

La sintaxis de comandos está en `docs/sintaxis.md`. La lista de mejoras post‑MVP está en `docs/timeline.md`.

## Modelo de ejecución

- Cada línea se tokeniza/parsea y se compila a bytecode.
- La VM ejecuta ese bytecode inmediatamente sin usar pila (stack) — lee directamente del bytecode.
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

## Instrucciones de bytecode (sin stack) — MVP

- `AddPlayer` — lee índice string del bytecode, agrega jugador a estado torneo
- `MakeGroups` — genera enfrentamientos (round-robin)
- `Show` — lee opción del bytecode y muestra información:
  - `1` — muestra lista de jugadores
  - `2` — muestra grupos
  - `3` - muestra enfrentamientos
- `Eoc` — fin comando

**Pendiente MVP**:
- `SetResult` (lee: matchId, scoreA, scoreB)
- `CalcTable`, `ShowTable`
- `CalcWinner`, `ShowWinner`

## Mapeo comando -> bytecode (MVP)

**JUGADOR**
```
JUGADOR "Ana"
=> AddPlayer <string_index>
```

**EMPAREJAR**
```
EMPAREJAR
=> MakeGroups
```

**VER JUGADORES**
```
VER JUGADORES
=> Show 1
```

**VER GRUPOS**
```
VER GRUPOS
=> Show 2
```

**VER ENFRENTAMIENTOS**
```
VER ENFRENTAMIENTOS
=> Show 3
```

**RESULTADO**
```
ENFRENTAMIENTO 1 RESULTADO 3-1
=> SetResult <match_id> <scoreA> <scoreB>
```

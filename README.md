# Torneos VM (individuales)

Lenguaje en español para administrar torneos de deportes individuales (tenis de mesa, tenis de campo, etc.), ejecutado **línea por línea** en un CLI y compilado a bytecode simple.

## MVP (prioridad para iniciar)

El MVP cubre únicamente:

- Agregar jugadores
- Generar enfrentamientos (round-robin simple)
- Registrar resultados
- Ver tabla y obtener ganador

## Ejemplo rápido (MVP)

```
TORNEO "Liga Tenis de Mesa"
JUGADOR "Ana"
JUGADOR "Luis"
JUGADOR "Marta"
JUGADOR "Carlos"

EMPAREJAR

RESULTADO "Ana" vs "Luis" 3-1
RESULTADO "Marta" vs "Carlos" 2-3

TABLA
GANADOR
```

## Documentación

- **Sintaxis (MVP):** `docs/sintaxis.md`
- **Bytecode y VM (MVP):** `docs/vm.md`
- **Timeline / Post-MVP:** `docs/timeline.md`

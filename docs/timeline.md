# Timeline (post‑MVP)

Esta lista es lo ideal para agregar **después** de tener el core (jugadores, emparejar, resultados, tabla/ganador).

## Fase 1: Configuración básica

- `PUNTOS GANA=X PIERDE=Y` (y eventualmente `EMPATA=Z`)
- `FORMATO ELIMINACION_SIMPLE`
- Validaciones más estrictas (no permitir resultados para jugadores inexistentes, etc.)

## Fase 2: Mejoras de emparejamiento

- `SEMILLA N` para reproducibilidad
- `NIVEL "Jugador" N`
- `EMPAREJAMIENTO AZAR | NIVELADO P=0.7`

## Fase 3: Desempates y ranking

- `DESEMPATE ...` (criterios y orden)
- Soportar empates en resultados (si el deporte lo permite)

## Fase 4: Presets

- `CARGAR "@presets/..."`
- Presets como archivos **YAML** (recomendado para autoría humana)
- Normalización/validación del preset

## Fase 5: Formato de partido

- `PARTIDO AL_MEJOR_DE N SETS A PUNTOS`
- Reglas específicas (tiebreak, ventaja, etc.)

## Fase 6: Calidad de vida CLI

- Comando `AYUDA`
- `LISTAR JUGADORES`, `LISTAR PARTIDOS`
- Exportar/importar estado (JSON)
- Guardado en disco
- `DESHACER`/`REHACER`

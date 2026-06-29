README.md para SDK
markdown
Copiar

# Level 2 - Configuración Contextual de Cádiz 1812

## Estructura



config/
├── game/              # Configuración global
├── world/             # Mundo (tiempo, actos, visibilidad)
├── content/          # Contenido del juego
│   ├── factions.json  # Facciones políticas
│   ├── spaces.json    # Espacios de Cádiz
│   ├── medidores.json # Medidores del juego
│   ├── characters/    # Personajes
│   │   ├── protagonists.json
│   │   └── npcs.json
│   └── events/        # Eventos
│       └── templates/
│           ├── pivote/      # Eventos históricos fijos
│           └── narrativo/   # Eventos narrativos
└── README.md          # Este archivo
text
Copiar

## Reglas de compatibilidad con el motor (Nivel 1)

### M1 (Estado del Mundo)
- **Facciones**: `id` debe coincidir con los IDs en `factions.json`
- **Espacios**: `id` debe coincidir con los IDs en `spaces.json`
- **Visibilidad**: Usa los niveles definidos en `visibility.json` (0=oculto, 1=parcial, 2=completo)

### M2 (Estado del Protagonista)
- **Atributos**: Usa los valores definidos en `protagonists.json`
- **Medidores**: Los IDs deben coincidir con `medidores.json`
- **Relaciones**: Las claves deben ser IDs de facciones de `factions.json`

### M3 (Medidores)
- **IDs**: Deben coincidir con los definidos en `medidores.json`
- **Rangos**: `min`=0, `max`=100 (como en el motor)
- **Umbrales**: `umbral_bajo` y `umbral_alto` son opcionales

### M4 (Generador de Eventos)
- **Plantillas**: Cada evento debe tener:
  - `id`: único
  - `event_type`: "pivote" o "narrativo"
  - `preconditions`: Objeto con `world` y/o `protagonist`
  - `consequences`: Objeto con efectos en medidores, relaciones, mundo
- **Familias**: Agrupan eventos temáticamente

### M5 (Jornada Director)
- **Actos**: Definidos en `acts.json`
- **Tramos**: Siempre ["manana", "tarde", "noche"] (como en el motor)

### M6 (Memoria)
- **Cooldowns**: Configurables por evento/familia

### M7 (Narrative Generator)
- **Triggers**: Condiciones basadas en:
  - Medidores (ej: `influencia > 80`)
  - Eventos activos (ej: `trafalgar_1805` ocurrió)
  - Decisiones tomadas (ej: `apoyar_resistencia` elegido)

## Convenciones de nombrado
- **IDs**: `snake_case` (ej: `liberales_progresistas`, `trafalgar_1805`)
- **Nombres**: Título propio (ej: "Liberales Progresistas", "La Batalla de Trafalgar")
- **Descripciones**: Texto narrativo para el jugador

## Ejemplo de flujo de trabajo con SDK
1. **Generar evento histórico**:
   - Usar plantilla base de `pivote/`
   - Completar campos: `id`, `name`, `date`, `description`
   - Definir `preconditions` basadas en documentación histórica
   - Definir `consequences` según impacto histórico

2. **Generar PNJ**:
   - Usar estructura de `npcs.json`
   - Asignar `faction` de las 5 definidas
   - Asignar `clase_social` y `oficio` de los disponibles
   - Definir `medidores` iniciales (0-100)

3. **Validar**:
   - Todos los IDs referenciados deben existir
   - Los rangos de medidores deben ser 0-100
   - Las precondiciones deben ser evaluables por el motor

## Documentación de referencia
- **Documento 7**: Cronología 1805-1815 (70+ eventos)
- **Documento 8**: Diario de Sesiones de las Cortes 1809-1814 (50+ eventos)
- **Documento 9**: Lista consolidada (78 eventos + anécdotas)

## Contacto
Para dudas sobre la configuración, consulte los documentos de la biblioteca CADIZ12 o pregunte al equipo de desarrollo.




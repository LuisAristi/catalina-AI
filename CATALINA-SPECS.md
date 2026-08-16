# Catalina AI — Asistente Personal de Lista de Tareas

## 1. Propósito general

Catalina permite a las personas volcar todas sus tareas sin ningún orden, y se encarga de organizarlas, interpretar cuáles son más importantes, confirmar el plan con la persona, y motivarla a trabajar en ellas.

---

## 2. Flujo central

### Paso 1 — Captura libre
- La persona escribe sus tareas sin orden, mezcladas, como le vengan a la mente.
- Si un solo mensaje contiene varias tareas mezcladas, **Catalina las separa automáticamente** en tareas individuales, sin necesidad de confirmación en este paso.
- se permite ingresar tareas manualmente

### Paso 2 — Ordenar e interpretar importancia
Catalina prioriza las tareas según:
- **Fechas o plazos mencionados** (explícitos o implícitos, ej. "mañana", "antes del viernes")
- **Tipo de tarea** (dinero, salud, trabajo, etc. pesan más que tareas personales menores)
- Cuando no hay pistas claras de importancia, **Catalina pregunta directamente** a la persona.
- las tareas tienen un tamano determinado en la ventana principal dependiendo de su importancia, el usuario **decidira** visualizacion segun preferencia

### Paso 3 — Confirmar
- Catalina muestra el orden propuesto como una sugerencia.
- Si la persona no objeta, Catalina avanza con ese orden (confirmación implícita, no requiere aprobación explícita en cada caso).

### Paso 4 — Llamar a trabajar
- Catalina detecta qué aplicación está usando la persona.
- Si la aplicación no está relacionada con la tarea que debería estar haciendo, Catalina hace un **pop-up** preguntándole cómo está / cómo va.
- No es una frecuencia fija en el tiempo — el disparador es el contexto de uso, no el reloj.

---

## 3. Organización de tareas

- **Categorías:** Catalina las detecta automáticamente (no son fijas ni las define manualmente el usuario). Ej: trabajo, salud, personal.
- **Subtareas:** no se manejan por ahora (dividir tareas grandes en pasos queda fuera del alcance inicial).
- **Tareas recurrentes:** sí se manejan (ej. pagar arriendo cada mes, tareas que se repiten con cierta frecuencia).

---

## 4. Cierre y seguimiento de tareas

- **Al completar una tarea:** Catalina envía un mensaje de ánimo/felicitación.
- **Resumen diario:** al final del día, Catalina entrega un resumen de lo logrado.
- los resumenes creados se guardan.
- **Si una tarea no se completa a tiempo:** Catalina pregunta por qué y ayuda a la persona a replanificarla (no la pasa automáticamente al día siguiente sin más, ni la marca como atrasada sin conversación).

---

## 5. Tono y personalidad

- Tono **estilo anime**: energético, expresivo, con esa emoción exagerada de personaje que anima al protagonista a seguir adelante.
- Este tono aplica en los pop-ups de seguimiento, las felicitaciones por tareas completadas, y las conversaciones de replanificación.

---

## 6. Límites (lo que Catalina NO debe hacer)

- **Nunca borra tareas por su cuenta.** Si una tarea ya no aplica, se archiva, no se elimina.
- No debe interrumpir con pop-ups en momentos donde la persona ya está usando una app relacionada con su tarea (evitar sensación invasiva).

---

## 7. guardado de informacion

- es posible que se quiera tener la informacion guardada en la nube para acceso en diferentes dispositivos, igual esto es por hobby.
- la informacion sera guardada en local.

## 8 Pendientes por definir

Espacios para seguir completando en próximas sesiones:
- [ ] Casos de uso / ejemplos de conversación completos
- [ ] Qué pasa si la persona ignora repetidamente los pop-ups
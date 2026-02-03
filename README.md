# 🎰 Poker Solver - Double or Nothing

¡Bienvenido al **Poker Solver**! Una herramienta potente escrita en **Rust** diseñada específicamente para estrategias de **Doble o Nada**, permitiéndote analizar bloqueos (blockers) de múltiples cuentas en tiempo real.

## 🚀 Características
- **Análisis de Bloqueos:** Agrega hasta 3 aliados (blockers) para reducir el rango del oponente.
- **Cálculo de Equity Real:** Analiza todo el mazo restante para darte tu % de victoria exacto.
- **Modo Duelo 1vs1:** Ingresa las cartas del villano para ver quién gana la mano actual.
- **Matriz Interactiva:** Visualiza qué manos específicas te ganan (Rojo) y cuáles dominas (Verde).
- **Interfaz Gráfica Fluida:** Construido con `egui` para máxima velocidad.

## 📦 Instalación (Windows)
1. Descarga el archivo `poker_solver_windows.zip`.
2. Descomprímelo.
3. Ejecuta `poker_solver.exe`. ¡Y listo! No requiere instalación.

## 🛠 Cómo usar
1. **Hero:** Selecciona tus 2 cartas.
2. **Mesa:** Agrega el Flop, Turn y River a medida que salen.
3. **Aliados:** Despliega la pestaña de aliados y pon las cartas de tus otras cuentas.
4. **Matriz:** Haz click en las celdas para excluir manos que sabes que el rival no jugaría.

---
*Desarrollado con ❤️ en Rust por Pablo.*
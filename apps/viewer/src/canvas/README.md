# Canvas

Canvas owns drawing and interaction math only.

Expected files:

- `renderer.ts`: draw scene shapes
- `layers.ts`: logical and physical canvas layers
- `hit-test.ts`: node/edge/label/badge hit testing
- `camera.ts`: world/screen coordinate conversion
- `text.ts`: text measurement and cache

Canvas code should not validate IR, compose FSMs, or decide projection meaning.
If Canvas code needs to know that a state is terminal, guarded, covered, or
diagnostic-bearing, move that decision into the ViewModel and expose it as a
badge, label, style token, or scene decoration.

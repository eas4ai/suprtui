# suprtui — glossary

Status: Agreed 2026-09-07.

- **Cell.** One position on the grid: a character plus its foreground
  color, background color, and text attributes (bold, underline, and so
  on). The unit the renderer compares and the terminal displays.
- **Buffer (cell grid).** The rectangular array of cells for one
  screen, `buffer.zig`'s `OptimizedBuffer`. Not a text buffer.
- **Text buffer.** Rope-backed storage for editable text (a document),
  independent of screen size. Painting a text buffer onto a cell grid
  is called drawing, never rendering.
- **Frame.** One complete cell-grid state handed to the renderer. The
  renderer never edits frames; it only compares them.
- **Renderer.** The component that compares the new frame with the
  previous one and writes the smallest correct update to the terminal.
- **Backend.** Where renderer output goes: the real terminal
  (`stdout`), an in-memory capture (`buffered`, used by tests), or a
  caller-supplied sink (`feed`).
- **Handle.** An opaque integer that names an engine object (a
  renderer, a buffer, a terminal) across the API. Handles are issued
  and checked by the handle table; a wrong-kind handle is rejected.
- **Grapheme cluster.** What the user sees as one character on screen,
  which may be several Unicode code points (for example a letter plus
  an accent). Cutting text between code points of one cluster breaks
  display.
- **Cell width.** How many grid columns a grapheme cluster occupies:
  one or two. The `uni` module is the only place that computes it.
- **Viewport.** The visible window over a text buffer or scrollback:
  which lines are on screen and where the cursor sits.
- **Scrollback.** Lines that scrolled off the top of the viewport and
  are kept for review. They are storage, not cells, until drawn.
- **Renderable.** A layout node bound to measured content (a text view
  or editor view). Layout arranges renderables; the renderer never
  sees them.
- **Capability.** A terminal feature the engine detected (for example
  Kitty graphics or Sixel). Output degrades gracefully when a
  capability is missing; it never fails.
- **Feed.** A caller-supplied byte sink that receives renderer output
  instead of the terminal, used for recording, testing, and embedding.

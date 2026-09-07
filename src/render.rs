//! Frame renderer core, ported from the reference `renderer.zig`
//! (REN-001 … REN-005, REN-011).
//!
//! The caller draws each frame into the next buffer (`next_buffer`);
//! `render` diffs it against the current buffer and publishes the
//! difference to the backend. Unchanged frames report `Skipped` and emit
//! nothing; failed frames publish nothing and roll back the hit grid,
//! staged image state, and cursor cache, forcing a full repaint next.
//!
//! Deliberately out of scope: threading parity (REN-006), terminal
//! lifecycle sequences (REN-007), hit testing with scissor (REN-008),
//! render offsets (REN-009), and image materialization (REN-010). Image
//! cells render through the buffer's quadrant fallback here. Style-run
//! coalescing is omitted on purpose: every changed cell carries its own
//! move plus `cell_ansi` bytes, which is correct output at the cost of
//! redundant sequences; the reference's run tracking is an optimization,
//! not a requirement.

use crate::ansi::{self, Rgba};
use crate::buffer::{BufferError, InitOptions, OptimizedBuffer};
use crate::uni::pool::GraphemePool;
use std::cell::RefCell;
use std::rc::Rc;

// ---- frame envelope sequences (reference `ansi.ANSI`) ----

const SYNC_SET: &str = "\x1b[?2026h";
const SYNC_RESET: &str = "\x1b[?2026l";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";
const RESET: &str = "\x1b[0m";
const DEFAULT_CURSOR_STYLE: &str = "\x1b[0 q";

/// Per-frame write outcome. Reference `output.WriteStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteStatus {
    Ok,
    Skipped,
    Failed,
}

/// Per-frame render outcome. Reference `RenderStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderStatus {
    Rendered,
    Skipped,
    Failed,
}

/// Byte sink behind one frame. The renderer owns the diff and cursor
/// logic; the backend only accumulates, commits, or drops bytes.
pub trait Backend {
    /// Backpressure gate. Anything but `Ok` skips the frame before any
    /// byte is produced. The memory backend always reports `Ok`.
    fn prepare_frame(&mut self) -> WriteStatus;
    /// Reset the frame accumulator.
    fn begin_frame(&mut self);
    /// Accumulate output bytes for the current frame.
    fn write_bytes(&mut self, data: &[u8]);
    /// Mark the frame failed; `end_frame` must then drop it.
    fn fail_frame(&mut self);
    /// Commit the frame (`Ok`) or report failure (`Failed`) after
    /// dropping the partial bytes. Empty frames commit observably
    /// nothing: the memory backend records no frame for them.
    fn end_frame(&mut self) -> WriteStatus;
}

/// In-memory backend. Captures exactly the bytes the renderer wrote, so
/// tests observe the same stream a terminal would receive (REN-005).
/// `fail_next` turns the next `end_frame` into a failure that publishes
/// nothing, which is how the rollback tests fail a frame (REN-004).
#[derive(Clone, Debug, Default)]
pub struct MemoryBackend {
    frames: Vec<Vec<u8>>,
    current: Vec<u8>,
    failed: bool,
    fail_next: bool,
}

impl MemoryBackend {
    pub fn new() -> Self {
        MemoryBackend {
            frames: Vec::new(),
            current: Vec::new(),
            failed: false,
            fail_next: false,
        }
    }

    /// Committed frames in order; empty frames are never recorded.
    pub fn frames(&self) -> &[Vec<u8>] {
        &self.frames
    }

    /// Fail the next frame at `end_frame`, publishing nothing.
    pub fn set_fail_next(&mut self, fail: bool) {
        self.fail_next = fail;
    }
}

impl Backend for MemoryBackend {
    fn prepare_frame(&mut self) -> WriteStatus {
        WriteStatus::Ok
    }

    fn begin_frame(&mut self) {
        self.current.clear();
        self.failed = false;
    }

    fn write_bytes(&mut self, data: &[u8]) {
        self.current.extend_from_slice(data);
    }

    fn fail_frame(&mut self) {
        self.failed = true;
    }

    fn end_frame(&mut self) -> WriteStatus {
        let failed = self.failed || self.fail_next;
        self.fail_next = false;
        self.failed = false;
        if failed {
            // Partial bytes are dropped, never published (REN-004).
            self.current.clear();
            return WriteStatus::Failed;
        }
        if !self.current.is_empty() {
            self.frames.push(std::mem::take(&mut self.current));
        }
        WriteStatus::Ok
    }
}

/// Cursor shape. Reference `Terminal.CursorStyle` without the terminal
/// domain, which owns mode toggles (spec mapping).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorStyle {
    Default,
    Block,
    Line,
    Underline,
}

/// Pending cursor state, set by the caller. Coordinates are 0-based grid
/// cells; emission adds one, matching the cell convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorState {
    pub x: u32,
    pub y: u32,
    pub visible: bool,
    pub style: CursorStyle,
    pub blinking: bool,
    pub color: Rgba,
}

impl Default for CursorState {
    fn default() -> Self {
        CursorState {
            x: 0,
            y: 0,
            visible: false,
            style: CursorStyle::Default,
            blinking: false,
            color: ansi::rgb_color(255, 255, 255, 255),
        }
    }
}

/// Render statistics. `frame_count` grows by one per rendered frame only:
/// skipped and failed frames never touch it (REN-011).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderStats {
    pub frame_count: u64,
    pub cells_updated: u32,
}

/// Double-buffered frame renderer over a caller-owned backend.
/// Reference `CliRenderer`, render-core slice.
pub struct Renderer<'a, B: Backend> {
    backend: B,
    current: OptimizedBuffer<'a>,
    next: OptimizedBuffer<'a>,
    width: u32,
    height: u32,
    background: Rgba,
    cursor: CursorState,
    last_style_tag: Option<u8>,
    last_blinking: Option<bool>,
    last_color: Option<(u8, u8, u8)>,
    last_x: Option<u32>,
    last_y: Option<u32>,
    last_visible: Option<bool>,
    force_full_repaint: bool,
    current_hit: Vec<u32>,
    next_hit: Vec<u32>,
    committed_images: Vec<u32>,
    pending_images: Vec<u32>,
    stats: RenderStats,
    emitted: usize,
}

impl<'a, B: Backend> Renderer<'a, B> {
    /// Both buffers start cleared to the background, hence identical, so
    /// a first frame with no drawing skips and emits nothing.
    pub fn new(
        width: u32,
        height: u32,
        pool: Rc<RefCell<GraphemePool<'a>>>,
        backend: B,
    ) -> Result<Self, BufferError> {
        let background = ansi::rgb_color(0, 0, 0, 255);
        let mut current = OptimizedBuffer::new(width, height, InitOptions::new(Rc::clone(&pool)))?;
        let mut next = OptimizedBuffer::new(width, height, InitOptions::new(pool))?;
        current.clear(background, None);
        next.clear(background, None);
        let size = width as usize * height as usize;
        Ok(Renderer {
            backend,
            current,
            next,
            width,
            height,
            background,
            cursor: CursorState::default(),
            last_style_tag: None,
            last_blinking: None,
            last_color: None,
            last_x: None,
            last_y: None,
            last_visible: None,
            force_full_repaint: false,
            current_hit: vec![0; size],
            next_hit: vec![0; size],
            committed_images: Vec::new(),
            pending_images: Vec::new(),
            stats: RenderStats::default(),
            emitted: 0,
        })
    }

    /// Draw the next frame here. Reference `getNextBuffer`.
    pub fn next_buffer(&mut self) -> &mut OptimizedBuffer<'a> {
        &mut self.next
    }

    /// Last published frame, for tests.
    pub fn current_buffer(&self) -> &OptimizedBuffer<'a> {
        &self.current
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn set_cursor(&mut self, x: u32, y: u32, visible: bool) {
        self.cursor.x = x;
        self.cursor.y = y;
        self.cursor.visible = visible;
    }

    pub fn set_cursor_style(&mut self, style: CursorStyle, blinking: bool) {
        self.cursor.style = style;
        self.cursor.blinking = blinking;
    }

    pub fn set_cursor_color(&mut self, color: Rgba) {
        self.cursor.color = color;
    }

    /// Stage one image id for this frame. Materialization is REN-010;
    /// here staging only participates in commit and rollback (REN-004).
    pub fn stage_image(&mut self, placement_id: u32) {
        self.pending_images.push(placement_id);
    }

    /// Image ids committed by the last successful frame.
    pub fn committed_images(&self) -> &[u32] {
        &self.committed_images
    }

    /// Stage one hit-grid id. Full hit testing with scissor is REN-008;
    /// this observes commit versus rollback only.
    pub fn set_next_hit(&mut self, x: u32, y: u32, id: u32) {
        if x < self.width && y < self.height {
            self.next_hit[(y * self.width + x) as usize] = id;
        }
    }

    /// Last committed hit-grid id. Test hook for REN-004; `check_hit`
    /// semantics arrive with render-terminal.
    pub fn committed_hit(&self, x: u32, y: u32) -> u32 {
        if x < self.width && y < self.height {
            self.current_hit[(y * self.width + x) as usize]
        } else {
            0
        }
    }

    pub fn stats(&self) -> RenderStats {
        self.stats
    }

    fn emit(&mut self, bytes: &[u8]) {
        if !bytes.is_empty() {
            self.emitted += bytes.len();
            self.backend.write_bytes(bytes);
        }
    }

    fn emit_str(&mut self, s: &str) {
        self.emit(s.as_bytes());
    }

    /// Open the frame envelope lazily: sync set plus cursor hide.
    /// Reference `beginRenderFrame`. Frames that never start stay
    /// byte-empty, which is the no-op suppression mechanism.
    fn start_frame(&mut self, started: &mut bool) {
        if !*started {
            self.emit_str(SYNC_SET);
            self.emit_str(HIDE_CURSOR);
            *started = true;
        }
    }

    fn cells_equal(a: &crate::buffer::Cell, b: &crate::buffer::Cell) -> bool {
        a.char == b.char && a.attributes == b.attributes && a.fg == b.fg && a.bg == b.bg
    }

    fn row_equal(&self, y: u32) -> bool {
        for x in 0..self.width {
            let current = self.current.get(x, y);
            let next = self.next.get(x, y);
            match (current, next) {
                (Some(a), Some(b)) => {
                    if !Self::cells_equal(&a, &b) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }

    /// Diff the next buffer against the current one and publish changed
    /// cells, syncing each published cell into the current buffer with
    /// `sync_cell`: no span cleanup, so continuation cells written by an
    /// earlier column of this same left-to-right pass survive (reference
    /// #723). Returns the cell count for stats.
    fn diff_buffers(&mut self, force: bool, started: &mut bool) -> u32 {
        use crate::uni::segments::is_continuation_char;
        let mut cells_updated = 0;
        // Row-equality fast path, disabled under force: a fully rewritten
        // row disables it for the rows after it (reference behavior).
        let mut use_row_equality = !force;
        for y in 0..self.height {
            if use_row_equality && self.row_equal(y) {
                continue;
            }
            let row_before = cells_updated;
            for x in 0..self.width {
                let (Some(current_cell), Some(next_cell)) =
                    (self.current.get(x, y), self.next.get(x, y))
                else {
                    continue;
                };
                if !force && Self::cells_equal(&current_cell, &next_cell) {
                    continue;
                }
                if is_continuation_char(next_cell.char) {
                    // Continuations carry no bytes; syncing alone keeps
                    // the next diff correct without starting a frame.
                    self.current.sync_cell(x, y, next_cell);
                    cells_updated += 1;
                    continue;
                }
                self.start_frame(started);
                let move_to = format!("\x1b[{};{}H", y + 1, x + 1);
                self.emit_str(&move_to);
                // Truecolor assumed; capability-gated emission arrives
                // with render-terminal. Image cells fall back to quadrant
                // glyphs here; protocol paths are REN-010.
                if let Some(ansi) = self.next.cell_ansi(x, y, true, true) {
                    self.emit_str(&ansi);
                }
                self.current.sync_cell(x, y, next_cell);
                cells_updated += 1;
            }
            if cells_updated - row_before == self.width {
                use_row_equality = false;
            }
        }
        cells_updated
    }

    fn cursor_style_code(style: CursorStyle, blinking: bool) -> &'static str {
        match (style, blinking) {
            (CursorStyle::Default, _) => DEFAULT_CURSOR_STYLE,
            (CursorStyle::Block, true) => "\x1b[1 q",
            (CursorStyle::Block, false) => "\x1b[2 q",
            (CursorStyle::Line, true) => "\x1b[5 q",
            (CursorStyle::Line, false) => "\x1b[6 q",
            (CursorStyle::Underline, true) => "\x1b[3 q",
            (CursorStyle::Underline, false) => "\x1b[4 q",
        }
    }

    fn style_tag(style: CursorStyle) -> u8 {
        match style {
            CursorStyle::Default => 0,
            CursorStyle::Block => 1,
            CursorStyle::Line => 2,
            CursorStyle::Underline => 3,
        }
    }

    /// Emit cursor moves and style changes exactly when they change
    /// between frames (REN-003). When the frame already produced visual
    /// output, position and visibility are restored unconditionally.
    fn emit_cursor(&mut self, started: &mut bool) {
        let cursor = self.cursor;
        if cursor.visible {
            let style_tag = Self::style_tag(cursor.style);
            let style_changed = self.last_style_tag != Some(style_tag)
                || self.last_blinking != Some(cursor.blinking);
            let color = (
                ansi::red(cursor.color),
                ansi::green(cursor.color),
                ansi::blue(cursor.color),
            );
            let color_changed = self.last_color != Some(color);
            let position_changed = self.last_x != Some(cursor.x) || self.last_y != Some(cursor.y);
            let visibility_changed = self.last_visible != Some(true);
            if *started || style_changed || color_changed || position_changed || visibility_changed
            {
                self.start_frame(started);
                if color_changed {
                    let seq = format!("\x1b]12;#{:02x}{:02x}{:02x}\x07", color.0, color.1, color.2);
                    self.emit_str(&seq);
                    self.last_color = Some(color);
                }
                if style_changed {
                    self.emit_str(Self::cursor_style_code(cursor.style, cursor.blinking));
                    self.last_style_tag = Some(style_tag);
                    self.last_blinking = Some(cursor.blinking);
                }
                let move_to = format!("\x1b[{};{}H", cursor.y + 1, cursor.x + 1);
                self.emit_str(&move_to);
                self.emit_str(SHOW_CURSOR);
            }
            self.last_x = Some(cursor.x);
            self.last_y = Some(cursor.y);
            self.last_visible = Some(true);
        } else {
            if !*started && self.last_visible != Some(false) {
                self.start_frame(started);
                self.emit_str(HIDE_CURSOR);
            }
            self.last_style_tag = None;
            self.last_blinking = None;
            self.last_color = None;
            self.last_x = None;
            self.last_y = None;
            self.last_visible = Some(false);
        }
    }

    fn clear_frame_state(&mut self) {
        self.next.clear(self.background, None);
        self.next_hit.fill(0);
        self.pending_images.clear();
    }

    /// Skipped frames clear the next buffer without publishing (REN-002).
    fn finish_skipped(&mut self) -> RenderStatus {
        self.clear_frame_state();
        RenderStatus::Skipped
    }

    /// Failed frames publish nothing and roll back: the next hit grid is
    /// dropped, staged images never commit, the cursor cache resets, and
    /// the next render repaints fully (REN-004).
    fn finish_failed(&mut self) -> RenderStatus {
        self.next_hit.fill(0);
        self.pending_images.clear();
        self.force_full_repaint = true;
        self.last_style_tag = None;
        self.last_blinking = None;
        self.last_color = None;
        self.last_x = None;
        self.last_y = None;
        self.last_visible = None;
        RenderStatus::Failed
    }

    fn finish_rendered(&mut self, cells_updated: u32) -> RenderStatus {
        std::mem::swap(&mut self.current_hit, &mut self.next_hit);
        self.next_hit.fill(0);
        self.committed_images.append(&mut self.pending_images);
        self.force_full_repaint = false;
        self.next.clear(self.background, None);
        self.stats.frame_count += 1;
        self.stats.cells_updated = cells_updated;
        RenderStatus::Rendered
    }

    /// Diff the next frame against the current one and publish the
    /// difference (REN-001). `force` repaints every cell.
    pub fn render(&mut self, force: bool) -> RenderStatus {
        // Backpressure: a refused backend skips before any byte exists.
        if self.backend.prepare_frame() != WriteStatus::Ok {
            return self.finish_skipped();
        }
        self.backend.begin_frame();
        self.emitted = 0;
        let should_force = force || self.force_full_repaint;
        let mut started = false;
        let cells_updated = self.diff_buffers(should_force, &mut started);
        if started {
            self.emit_str(RESET);
        }
        self.emit_cursor(&mut started);
        if started {
            self.emit_str(SYNC_RESET);
        }
        if self.emitted == 0 {
            // True no-op: the backend holds an empty frame the memory
            // backend never records, so nothing is published.
            self.backend.end_frame();
            return self.finish_skipped();
        }
        match self.backend.end_frame() {
            WriteStatus::Ok => self.finish_rendered(cells_updated),
            WriteStatus::Failed => self.finish_failed(),
            // Defensive: only the memory backend is wired here and it
            // never reports Skipped; a skipped backend frame publishes
            // nothing, so treat it as a skipped render.
            WriteStatus::Skipped => self.finish_skipped(),
        }
    }
}

#[cfg(test)]
use std::cell::RefCell as TestRefCell;
#[cfg(test)]
use std::rc::Rc as TestRc;

#[cfg(test)]
fn test_renderer(width: u32, height: u32) -> Renderer<'static, MemoryBackend> {
    let pool = TestRc::new(TestRefCell::new(GraphemePool::new()));
    Renderer::new(width, height, pool, MemoryBackend::new()).unwrap()
}

#[cfg(test)]
fn white_on_black<B: Backend>(renderer: &mut Renderer<'_, B>, text: &str, x: u32, y: u32) {
    use crate::ansi::rgb_color;
    renderer
        .next_buffer()
        .draw_text(
            text,
            x,
            y,
            rgb_color(255, 255, 255, 255),
            Some(rgb_color(0, 0, 0, 255)),
            0,
        )
        .unwrap();
}

/// REN-001 falsifier: content drawn into the next buffer never appears
/// in the output, or appears without a `render` call.
#[cfg(test)]
#[test]
fn frame_publish() {
    let mut renderer = test_renderer(6, 2);
    assert_eq!(0, renderer.backend().frames().len());
    white_on_black(&mut renderer, "AB", 0, 0);
    // Drawn but not yet rendered: nothing published.
    assert_eq!(0, renderer.backend().frames().len());
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    assert_eq!(1, renderer.backend().frames().len());
    let frame = renderer.backend().frames()[0].clone();
    assert!(frame.contains(&b'A'));
    assert!(frame.contains(&b'B'));
    assert!(frame.windows(6).any(|w| w == b"\x1b[1;1H"));
    // The published cell is now current.
    assert_eq!(
        b'A' as u32,
        renderer.current_buffer().get(0, 0).unwrap().char
    );
}

/// REN-002 falsifier: rendering an unchanged frame twice rewrites a
/// cell on the second pass.
#[cfg(test)]
#[test]
fn unchanged_skips() {
    let mut renderer = test_renderer(6, 2);
    white_on_black(&mut renderer, "AB", 0, 0);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    assert_eq!(1, renderer.backend().frames().len());
    let before = renderer.current_buffer().get(0, 0).unwrap();
    // Immediate mode: the caller redraws the identical frame; the diff
    // finds nothing, so the frame skips.
    white_on_black(&mut renderer, "AB", 0, 0);
    assert_eq!(RenderStatus::Skipped, renderer.render(false));
    // Skipped means published nothing and rewrote no cell.
    assert_eq!(1, renderer.backend().frames().len());
    assert_eq!(before, renderer.current_buffer().get(0, 0).unwrap());
}

/// REN-003 falsifier: an unchanged cursor is re-emitted, or a moved
/// cursor keeps its old position in the output.
#[cfg(test)]
#[test]
fn cursor_tracking() {
    let mut renderer = test_renderer(6, 2);
    renderer.set_cursor(2, 1, true);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    assert_eq!(1, renderer.backend().frames().len());
    let first = renderer.backend().frames()[0].clone();
    assert!(first.windows(6).any(|w| w == b"\x1b[2;3H"));
    assert!(first.windows(6).any(|w| w == b"\x1b[?25h"));
    // Unchanged cursor with unchanged cells: skipped, re-emitted nothing.
    assert_eq!(RenderStatus::Skipped, renderer.render(false));
    assert_eq!(1, renderer.backend().frames().len());
    // A move lands on the new position.
    renderer.set_cursor(0, 0, true);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    let moved = renderer.backend().frames()[1].clone();
    assert!(moved.windows(6).any(|w| w == b"\x1b[1;1H"));
    // A style change emits exactly the new style sequence.
    renderer.set_cursor_style(CursorStyle::Line, false);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    let styled = renderer.backend().frames()[2].clone();
    assert!(styled.windows(5).any(|w| w == b"\x1b[6 q"));
    // Hiding emits the hide sequence once, then goes quiet.
    renderer.set_cursor(0, 0, false);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    let hidden = renderer.backend().frames()[3].clone();
    assert!(hidden.windows(6).any(|w| w == b"\x1b[?25l"));
    assert_eq!(RenderStatus::Skipped, renderer.render(false));
    assert_eq!(4, renderer.backend().frames().len());
}

/// REN-004 falsifier: a hit-test answer or cell reflects the failed
/// frame, or the frame after a failure leaves a stale cell.
#[cfg(test)]
#[test]
fn failed_frame_rolls_back() {
    let mut renderer = test_renderer(6, 2);
    white_on_black(&mut renderer, "X", 0, 0);
    renderer.set_next_hit(0, 0, 7);
    renderer.stage_image(42);
    renderer.set_cursor(1, 1, true);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    assert_eq!(7, renderer.committed_hit(0, 0));
    assert_eq!(&[42], renderer.committed_images());
    // The failed frame stages new state, then publishes nothing.
    white_on_black(&mut renderer, "Y", 3, 1);
    renderer.set_next_hit(1, 0, 9);
    renderer.stage_image(43);
    renderer.backend_mut().set_fail_next(true);
    assert_eq!(RenderStatus::Failed, renderer.render(false));
    assert_eq!(1, renderer.backend().frames().len());
    assert_eq!(0, renderer.committed_hit(1, 0));
    assert_eq!(7, renderer.committed_hit(0, 0));
    assert_eq!(&[42], renderer.committed_images());
    // The next render repaints fully, including the cursor the failure
    // evicted from the cache. Immediate mode redraws both cells; without
    // the redraw X would correctly disappear rather than go stale.
    white_on_black(&mut renderer, "X", 0, 0);
    white_on_black(&mut renderer, "Y", 3, 1);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    let repaint = renderer.backend().frames()[1].clone();
    assert!(repaint.contains(&b'X'));
    assert!(repaint.contains(&b'Y'));
    assert!(repaint.windows(6).any(|w| w == b"\x1b[?25h"));
    assert_eq!((6 * 2) as u32, renderer.stats().cells_updated);
}

/// REN-005 falsifier: captured bytes differ from a tee'd copy of the
/// written stream for the same frame.
#[cfg(test)]
#[test]
fn memory_backend_exact() {
    struct Tee<'t> {
        a: &'t mut Vec<u8>,
        b: &'t mut Vec<u8>,
    }
    impl Backend for Tee<'_> {
        fn prepare_frame(&mut self) -> WriteStatus {
            WriteStatus::Ok
        }
        fn begin_frame(&mut self) {}
        fn write_bytes(&mut self, data: &[u8]) {
            self.a.extend_from_slice(data);
            self.b.extend_from_slice(data);
        }
        fn fail_frame(&mut self) {}
        fn end_frame(&mut self) -> WriteStatus {
            WriteStatus::Ok
        }
    }

    let mut tee_a = Vec::new();
    let mut tee_b = Vec::new();
    let pool = TestRc::new(TestRefCell::new(GraphemePool::new()));
    let mut tee_renderer = Renderer::new(
        2,
        1,
        TestRc::clone(&pool),
        Tee {
            a: &mut tee_a,
            b: &mut tee_b,
        },
    )
    .unwrap();
    let mut mem_renderer = Renderer::new(2, 1, pool, MemoryBackend::new()).unwrap();
    white_on_black(&mut tee_renderer, "Z", 0, 0);
    white_on_black(&mut mem_renderer, "Z", 0, 0);
    assert_eq!(RenderStatus::Rendered, tee_renderer.render(false));
    assert_eq!(RenderStatus::Rendered, mem_renderer.render(false));
    // Both tee copies saw the same stream, and the memory backend
    // captured exactly those bytes.
    assert_eq!(tee_a, tee_b);
    assert_eq!(1, mem_renderer.backend().frames().len());
    assert_eq!(tee_a, mem_renderer.backend().frames()[0]);
    // A 2x1 frame with one drawn cell is byte-exact by hand.
    let hand =
        "\x1b[?2026h\x1b[?25l\x1b[1;1H\x1b[38;2;255;255;255m\x1b[48;2;0;0;0mZ\x1b[0m\x1b[?2026l";
    assert_eq!(
        hand.as_bytes(),
        mem_renderer.backend().frames()[0].as_slice()
    );
}

/// REN-011 falsifier: a skipped frame increments the frame counter or
/// the cells counter.
#[cfg(test)]
#[test]
fn stats_count() {
    let mut renderer = test_renderer(6, 2);
    assert_eq!(0, renderer.stats().frame_count);
    white_on_black(&mut renderer, "AB", 0, 0);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    assert_eq!(1, renderer.stats().frame_count);
    let cells = renderer.stats().cells_updated;
    assert!(cells > 0);
    white_on_black(&mut renderer, "AB", 0, 0);
    assert_eq!(RenderStatus::Skipped, renderer.render(false));
    assert_eq!(1, renderer.stats().frame_count);
    assert_eq!(cells, renderer.stats().cells_updated);
    renderer.backend_mut().set_fail_next(true);
    white_on_black(&mut renderer, "AB", 0, 0);
    white_on_black(&mut renderer, "C", 0, 1);
    assert_eq!(RenderStatus::Failed, renderer.render(false));
    assert_eq!(1, renderer.stats().frame_count);
    white_on_black(&mut renderer, "AB", 0, 0);
    white_on_black(&mut renderer, "C", 0, 1);
    assert_eq!(RenderStatus::Rendered, renderer.render(false));
    assert_eq!(2, renderer.stats().frame_count);
}

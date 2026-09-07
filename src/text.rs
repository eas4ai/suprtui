//! Text storage, ported from the reference `rope.zig`,
//! `text-buffer.zig`, and `edit-buffer.zig` (TXT-001 … TXT-004).
//!
//! The reference entangles a balanced rope with views and memory
//! buffers; the spec observes only edit and read behavior (rope
//! balancing is an internal detail), so storage here is a plain
//! `String` addressed by byte offsets. Style spans shift and stretch
//! with edits, undo/redo restore content plus spans step by step, and
//! registered views go dirty on every content edit under an advancing
//! epoch. Wrapping, selection, cursor motion, highlights, syntax
//! styles, and bounds errors arrive with the `text-view` commitment,
//! which owns the view domain.

/// Style span over `[start, end)` byte offsets with a caller-defined
/// style id. Syntax styles attach meaning to ids in text-view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub style: u32,
}

/// Opaque handle to a registered view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ViewId(usize);

#[derive(Clone, Debug)]
struct Snapshot {
    text: String,
    spans: Vec<Span>,
}

/// Text entry-point failures. Out-of-range offsets and split points
/// inside a character return errors; nothing here panics on input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextError {
    OutOfRange,
    NotCharBoundary,
}

/// Editable text with spans, history, and view dirt tracking.
/// Reference `UnifiedTextBuffer` plus `EditBuffer` history, behavior
/// only.
pub struct TextBuffer {
    text: String,
    spans: Vec<Span>,
    history: Vec<Snapshot>,
    future: Vec<Snapshot>,
    views: Vec<bool>,
    free_views: Vec<usize>,
    epoch: u64,
}

impl TextBuffer {
    pub fn new() -> Self {
        TextBuffer {
            text: String::new(),
            spans: Vec::new(),
            history: Vec::new(),
            future: Vec::new(),
            views: Vec::new(),
            free_views: Vec::new(),
            epoch: 0,
        }
    }

    pub fn from_text(text: &str) -> Self {
        let mut buffer = TextBuffer::new();
        buffer.text.push_str(text);
        buffer
    }

    /// Full content as plain text.
    pub fn content(&self) -> &str {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    fn check_offset(&self, offset: usize) -> Result<(), TextError> {
        if offset > self.text.len() {
            return Err(TextError::OutOfRange);
        }
        if !self.text.is_char_boundary(offset) {
            return Err(TextError::NotCharBoundary);
        }
        Ok(())
    }

    fn begin_edit(&mut self) {
        self.history.push(Snapshot {
            text: self.text.clone(),
            spans: self.spans.clone(),
        });
        self.future.clear();
    }

    fn finish_edit(&mut self) {
        self.epoch += 1;
        for dirty in self.views.iter_mut() {
            *dirty = true;
        }
    }

    /// Insert text at a byte offset, shifting spans at or after it and
    /// stretching spans that cover it.
    pub fn insert(&mut self, offset: usize, text: &str) -> Result<(), TextError> {
        self.check_offset(offset)?;
        if text.is_empty() {
            return Ok(());
        }
        self.begin_edit();
        self.text.insert_str(offset, text);
        let delta = text.len();
        for span in self.spans.iter_mut() {
            if offset <= span.start {
                span.start += delta;
                span.end += delta;
            } else if offset <= span.end {
                span.end += delta;
            }
        }
        self.finish_edit();
        Ok(())
    }

    /// Delete `[offset, offset + len)`, dropping spans it fully covers
    /// and clamping spans it overlaps.
    pub fn delete(&mut self, offset: usize, len: usize) -> Result<(), TextError> {
        self.check_offset(offset)?;
        let end = offset.checked_add(len).ok_or(TextError::OutOfRange)?;
        self.check_offset(end)?;
        if len == 0 {
            return Ok(());
        }
        self.begin_edit();
        self.text.drain(offset..end);
        let mut kept = Vec::with_capacity(self.spans.len());
        for span in self.spans.drain(..) {
            if span.end <= offset {
                kept.push(span);
            } else if span.start >= end {
                kept.push(Span {
                    start: span.start - len,
                    end: span.end - len,
                    style: span.style,
                });
            } else if span.start >= offset && span.end <= end {
                // Fully covered: the span's text is gone with it.
            } else {
                let overlap = span.end.min(end) - span.start.max(offset);
                let start = span.start.min(offset);
                let end = start + (span.end - span.start) - overlap;
                kept.push(Span {
                    start,
                    end,
                    style: span.style,
                });
            }
        }
        self.spans = kept;
        self.finish_edit();
        Ok(())
    }

    /// Split the buffer at an offset, returning the tail as a new
    /// buffer. Spans crossing the split are clamped to their side;
    /// history stays with the head.
    pub fn split_off(&mut self, offset: usize) -> Result<TextBuffer, TextError> {
        self.check_offset(offset)?;
        self.begin_edit();
        let tail_text = self.text.split_off(offset);
        let mut tail_spans = Vec::new();
        let mut head_spans = Vec::with_capacity(self.spans.len());
        for span in self.spans.drain(..) {
            if span.end <= offset {
                head_spans.push(span);
            } else if span.start >= offset {
                tail_spans.push(Span {
                    start: span.start - offset,
                    end: span.end - offset,
                    style: span.style,
                });
            } else {
                head_spans.push(Span {
                    start: span.start,
                    end: offset,
                    style: span.style,
                });
                tail_spans.push(Span {
                    start: 0,
                    end: span.end - offset,
                    style: span.style,
                });
            }
        }
        self.spans = head_spans;
        let mut tail = TextBuffer::new();
        tail.text = tail_text;
        tail.spans = tail_spans;
        self.finish_edit();
        Ok(tail)
    }

    /// Cover `[start, end)` with a style id. Bounds are validated like
    /// edits; spans are rendered content, so this pushes history, marks
    /// views dirty, and advances the epoch like any content edit.
    pub fn add_span(&mut self, start: usize, end: usize, style: u32) -> Result<(), TextError> {
        self.check_offset(start)?;
        self.check_offset(end)?;
        if start > end {
            return Err(TextError::OutOfRange);
        }
        self.begin_edit();
        self.spans.push(Span { start, end, style });
        self.finish_edit();
        Ok(())
    }

    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    fn restore(&mut self, snapshot: Snapshot) {
        self.text = snapshot.text;
        self.spans = snapshot.spans;
        self.epoch += 1;
        for dirty in self.views.iter_mut() {
            *dirty = true;
        }
    }

    /// Restore content and styles one step back.
    pub fn undo(&mut self) -> bool {
        let Some(snapshot) = self.history.pop() else {
            return false;
        };
        self.future.push(Snapshot {
            text: std::mem::take(&mut self.text),
            spans: std::mem::take(&mut self.spans),
        });
        self.restore(snapshot);
        true
    }

    /// Replay one undone step. Any new edit clears the redo stack.
    pub fn redo(&mut self) -> bool {
        let Some(snapshot) = self.future.pop() else {
            return false;
        };
        self.history.push(Snapshot {
            text: std::mem::take(&mut self.text),
            spans: std::mem::take(&mut self.spans),
        });
        self.restore(snapshot);
        true
    }

    /// Register a view; it reads dirty until cleared.
    pub fn register_view(&mut self) -> ViewId {
        if let Some(index) = self.free_views.pop() {
            self.views[index] = true;
            ViewId(index)
        } else {
            self.views.push(true);
            ViewId(self.views.len() - 1)
        }
    }

    pub fn unregister_view(&mut self, view: ViewId) {
        if view.0 < self.views.len() {
            self.views[view.0] = false;
            self.free_views.push(view.0);
        }
    }

    pub fn view_dirty(&self, view: ViewId) -> bool {
        self.views.get(view.0).copied().unwrap_or(false)
    }

    pub fn clear_view_dirty(&mut self, view: ViewId) {
        if let Some(dirty) = self.views.get_mut(view.0) {
            *dirty = false;
        }
    }

    /// Content epoch, advanced by every content edit, undo, and redo.
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        TextBuffer::new()
    }
}

/// TXT-001 falsifier: plain-text extraction differs from the reference
/// fixture after a ported edit sequence.
#[cfg(test)]
#[test]
fn edits() {
    let mut buf = TextBuffer::from_text("hello world");
    buf.insert(5, ", brave").unwrap();
    assert_eq!("hello, brave world", buf.content());
    buf.delete(5, 7).unwrap();
    assert_eq!("hello world", buf.content());
    buf.insert(11, "!").unwrap();
    let tail = buf.split_off(6).unwrap();
    assert_eq!("hello ", buf.content());
    assert_eq!("world!", tail.content());
    // Multibyte text: offsets are bytes, splits never strand.
    let mut wide = TextBuffer::from_text("héllo→world");
    wide.insert(1, "X").unwrap();
    assert_eq!("hXéllo→world", wide.content());
    wide.delete(2, "é".len()).unwrap();
    assert_eq!("hXllo→world", wide.content());
    assert_eq!(TextError::OutOfRange, wide.insert(99, "?").unwrap_err());
    assert_eq!(TextError::OutOfRange, wide.delete(0, 99).unwrap_err());
    assert_eq!(TextError::NotCharBoundary, wide.insert(6, "?").unwrap_err());
}

/// TXT-002 falsifier: inserting text before a span leaves the span on
/// its old columns instead of shifting with its text.
#[cfg(test)]
#[test]
fn span_tracking() {
    let mut buf = TextBuffer::from_text("hello world");
    buf.add_span(6, 11, 1).unwrap();
    buf.insert(0, "well, ").unwrap();
    assert_eq!(
        [Span {
            start: 12,
            end: 17,
            style: 1
        }],
        buf.spans()
    );
    // At the span start the insert shifts it with its text.
    buf.insert(12, "wide ").unwrap();
    assert_eq!(
        [Span {
            start: 17,
            end: 22,
            style: 1
        }],
        buf.spans()
    );
    assert_eq!("well, hello wide world", buf.content());
    // Inside the span stretches it.
    buf.insert(19, "XX").unwrap();
    assert_eq!(
        [Span {
            start: 17,
            end: 24,
            style: 1
        }],
        buf.spans()
    );
    // Deleting across the span clamps it to the surviving text.
    buf.delete(10, 8).unwrap();
    assert_eq!("well, helloXXrld", buf.content());
    assert_eq!(
        [Span {
            start: 10,
            end: 16,
            style: 1
        }],
        buf.spans()
    );
    // Deleting the whole span drops it.
    buf.delete(10, 6).unwrap();
    assert!(buf.spans().is_empty());
}

/// TXT-003 falsifier: redo still applies after an intervening edit, or
/// undo restores content but drops styles.
#[cfg(test)]
#[test]
fn undo_redo() {
    let mut buf = TextBuffer::from_text("ab");
    buf.insert(2, "c").unwrap();
    buf.add_span(0, 3, 7).unwrap();
    buf.delete(0, 1).unwrap();
    assert_eq!("bc", buf.content());
    assert!(buf.can_undo());
    // Step back: delete undone first, content and styles together.
    assert!(buf.undo());
    assert_eq!("abc", buf.content());
    assert_eq!(
        [Span {
            start: 0,
            end: 3,
            style: 7
        }],
        buf.spans()
    );
    assert!(buf.undo());
    assert_eq!("abc", buf.content());
    assert!(buf.spans().is_empty());
    assert!(buf.undo());
    assert_eq!("ab", buf.content());
    assert!(!buf.can_undo());
    assert!(!buf.undo());
    // Replay in order.
    assert!(buf.redo());
    assert_eq!("abc", buf.content());
    assert!(buf.redo());
    assert_eq!(
        [Span {
            start: 0,
            end: 3,
            style: 7
        }],
        buf.spans()
    );
    assert!(buf.redo());
    assert_eq!("bc", buf.content());
    assert!(!buf.can_redo());
    // A new edit clears the redo stack.
    assert!(buf.undo());
    buf.insert(0, "z").unwrap();
    assert_eq!("zabc", buf.content());
    assert!(!buf.can_redo());
    assert!(!buf.redo());
}

/// TXT-004 falsifier: a registered view reads clean after an edit, or
/// the epoch is unchanged.
#[cfg(test)]
#[test]
fn view_dirty_tracking() {
    let mut buf = TextBuffer::from_text("ab");
    let first = buf.register_view();
    let second = buf.register_view();
    buf.clear_view_dirty(first);
    buf.clear_view_dirty(second);
    let epoch = buf.epoch();
    buf.insert(2, "c").unwrap();
    assert!(buf.epoch() > epoch);
    assert!(buf.view_dirty(first));
    assert!(buf.view_dirty(second));
    buf.clear_view_dirty(first);
    assert!(!buf.view_dirty(first));
    assert!(buf.view_dirty(second));
    // Undo and redo are content edits too.
    buf.clear_view_dirty(second);
    assert!(buf.undo());
    assert!(buf.view_dirty(first));
    assert!(buf.view_dirty(second));
    // Unregistered views read clean and never panic.
    buf.unregister_view(first);
    assert!(!buf.view_dirty(first));
    assert!(!buf.view_dirty(ViewId(999)));
    buf.clear_view_dirty(ViewId(999));
}

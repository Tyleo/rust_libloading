use LibTracked;
use LibUnsafe;
use std::sync::Arc;

/// A shared library which implements [LibTracked](struct.LibTracked.html) with atomic ref-counting to track its [Symbols](trait.Symbol.html).
pub type LibArc = LibTracked<Arc<LibUnsafe>>;

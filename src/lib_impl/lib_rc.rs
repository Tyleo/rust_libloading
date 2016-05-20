use LibTracked;
use LibUnsafe;
use std::rc::Rc;

/// A shared library which implements [LibTracked](struct.LibTracked.html) with atomic ref-counting to track its [Symbols](trait.Symbol.html).
pub type LibRc = LibTracked<Rc<LibUnsafe>>;

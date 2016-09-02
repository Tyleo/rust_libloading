//! Defines errors which may be returned by [sharedlib](index.html).

use std::path::PathBuf;

error_chain! {
    types { }

    links { }

    foreign_links { }

    errors {
        LibraryClose {
            description("A shared library failed to close.")
            display(
                "{}",
                "A shared library failed to close.",
            )
        }

        LibraryFindSymbol(symbol: String) {
            description("Failed to find a symbol in a shared library.")
            display(
                "{}{}{}",
                "The search for symbol, '",
                symbol,
                "', from a shared library failed.",
            )
        }

        LibraryOpen(path_to_lib: PathBuf) {
            description("A shared library failed to open.")
            display(
                "{}{}{}",
                "The shared library at path, '",
                path_to_lib.to_string_lossy(),
                "', failed to open.",
            )
        }

        OsError(cause: String, function_called: String) {
            description("A call to a native function failed.")
            display(
                "{}{}{}{}",
                "A call to the native function, '",
                function_called,
                "', failed. Cause\n",
                cause
            )
        }

        OsErrorFailure(function_called: String) {
            description("A call to a native function failed but the operating system reported success.")
            display(
                "{}{}{}",
                "A call to the native function, '",
                function_called,
                "', failed but the operating system reported success.",
            )
        }
    }
}

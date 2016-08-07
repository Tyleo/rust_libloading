//! Defines errors which may be returned by [sharedlib](index.html).

use std::io;
use std::path::PathBuf;
use string::error::library_close as library_close_string;
use string::error::library_find_symbol as library_find_symbol_string;
use string::error::library_open as library_open_string;
use string::error::os_error as os_error_string;
use string::error::os_error_failure as os_error_failure_string;

error_chain! {
    types { }

    links { }

    foreign_links {
        io::Error, IoError;
    }

    errors {
        LibraryClose {
            description(library_close_string::description())
            display(
                "{}",
                library_close_string::display_1()
            )
        }

        LibraryFindSymbol(symbol: String) {
            description(library_find_symbol_string::description())
            display(
                "{}{}{}",
                library_find_symbol_string::display_1(),
                symbol,
                library_find_symbol_string::display_2()
            )
        }

        LibraryOpen(path_to_lib: PathBuf) {
            description(library_open_string::description())
            display(
                "{}{}{}",
                library_open_string::display_1(),
                path_to_lib.to_string_lossy(),
                library_open_string::display_2()
            )
        }

        OsError(cause: String, function_called: String) {
            description(os_error_string::description())
            display(
                "{}{}{}{}",
                os_error_string::display_1(),
                function_called,
                os_error_string::display_2(),
                cause
            )
        }

        OsErrorFailure(function_called: String) {
            description(os_error_failure_string::description())
            display(
                "{}{}{}",
                os_error_failure_string::display_1(),
                function_called,
                os_error_failure_string::display_2()
            )
        }
    }
}

/// Unwraps in the ```Some``` case; returns in the ```None``` case.
///```
/// return_if_none!(expression)
///```
/// is shorthand for
///```
/// match expression {
///     Some(ret) => ret,
///     None => return
/// }
/// ```
#[macro_export]
macro_rules! return_if_none {
    ($e:expr) => {
        match { $e } {
            Some(ret) => ret,
            None => return,
        }
    };
}

/// Unwraps in the ```Some``` case; breaks in the ```None``` case.
///```
/// break_if_none!(expression)
///```
/// is shorthand for
///```
/// match expression {
///     Some(ret) => ret,
///     None => break
/// }
/// ```
#[macro_export]
macro_rules! break_if_none {
    ($e:expr) => {
        match { $e } {
            Some(ret) => ret,
            None => break,
        }
    };
}

/// Unwraps in the ```Ok``` case; returns in the ```Err``` case.
/// Discards anything contained within Err.
///```
/// return_if_err!(expression)
///```
/// is shorthand for
///```
/// match expression {
///     Ok(ret) => ret,
///     Err(_) => return
/// }
/// ```
#[macro_export]
macro_rules! return_if_err {
    ($e:expr) => {
        match { $e } {
            Ok(ret) => ret,
            Err(_) => return,
        }
    };
}

/// Unwraps in the ```Ok``` case; breaks in the ```Err``` case.
/// Discards anything contained within Err.
///```
/// break_if_err!(expression)
///```
/// is shorthand for
///```
/// match expression {
///     Ok(ret) => ret,
///     Err(_) => break
/// }
/// ```
#[macro_export]
macro_rules! break_if_err {
    ($e:expr) => {
        match { $e } {
            Ok(ret) => ret,
            Err(_) => break,
        }
    };
}

/// Unwraps in the ```Ok``` case; does something in the ```None``` case.
///
/// Usually you want ```unwrap_or_else``` from the standard library. Use this for cases where 
/// you need to do something simple before returning or breaking.
///
///```
/// do_if_none!(expression, do)
///```
/// is shorthand for
///```
/// match expression {
///     Some(ret) => ret,
///     None => {
///         do
///     }
/// }
/// ```
#[macro_export]
macro_rules! do_if_none {
    ($e:expr, $d:expr) => {
        match { $e } {
            Some(ret) => ret,
            None => $d
        }
    };
}

/// Unwraps in the ```Ok``` case; does something in the ```Err``` case.
/// Discards anything contained within Err.
///
/// Usually you want ```unwrap_or_else``` from the standard library. Use this for cases where 
/// you need to do something simple before returning or breaking.
///
///```
/// do_if_err!(expression, do)
///```
/// is shorthand for
///```
/// match expression {
///     Ok(ret) => ret,
///     Err(_) => {
///         do_if_err
///     }
/// }
/// ```
#[macro_export]
macro_rules! do_if_err {
    ($e:expr, $d:expr) => {
        match { $e } {
            Ok(ret) => ret,
            Err(_) => $d
        }
    };
}
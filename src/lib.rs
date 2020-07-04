/// Unwraps in the ```Some``` case; returns in the ```None``` case.
///```
/// unwrap_return!(expression)
///```
/// is shorthand for
///```
/// match expression {
///     Some(ret) => ret,
///     None => return
/// }
/// ```
#[macro_export]
macro_rules! unwrap_return {
    ($e:expr) => {
        match { $e } {
            Some(ret) => ret,
            None => return,
        }
    };
}

/// Unwraps in the ```Some``` case; breaks in the ```None``` case.
///```
/// unwrap_break!(expression)
///```
/// is shorthand for
///```
/// match expression {
///     Some(ret) => ret,
///     None => break
/// }
/// ```
#[macro_export]
macro_rules! unwrap_break {
    ($e:expr) => {
        match { $e } {
            Some(ret) => ret,
            None => break,
        }
    };
}

/// Unwraps in the ```Some``` case; does something in the ```None``` case.
///
/// Usually you want ```unwrap_or_else``` from the standard library. Use this for cases where 
/// you need to do something simple before returning or breaking, or you want to return a value 
/// other than ().
///
///```
/// unwrap_do!(expression, do)
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
macro_rules! unwrap_do {
    ($e:expr, $d:expr) => {
        match { $e } {
            Some(ret) => ret,
            None => $d
        }
    };
}

/// Unwraps in the ```Ok``` case; returns in the ```Err``` case.
///
/// Drops everything contained within Err.
///
///```
/// ok_unwrap_return!(expression)
///```
/// is shorthand for
///```
/// match expression {
///     Ok(ret) => ret,
///     Err(_) => return
/// }
/// ```
#[macro_export]
macro_rules! ok_unwrap_return {
    ($e:expr) => {
        match { $e } {
            Ok(ret) => ret,
            Err(_) => return,
        }
    };
}

/// Unwraps in the ```Ok``` case; breaks in the ```Result``` case.
///
/// Drops everything contained within Err.
///
///```
/// ok_unwrap_break!(expression)
///```
/// is shorthand for
///```
/// match expression {
///     Ok(ret) => ret,
///     Err(_) => break
/// }
/// ```
#[macro_export]
macro_rules! ok_unwrap_break {
    ($e:expr) => {
        match { $e } {
            Ok(ret) => ret,
            Err(_) => break,
        }
    };
}

/// Unwraps in the ```Ok``` case; does something in the ```Err``` case.
///
/// Drops everything contained within Err.
///
/// Usually you want ```unwrap_or_else``` from the standard library. Use this for cases where 
/// you need to do something simple before returning or breaking, or you want to return a value 
/// other than ().
///
///```
/// ok_unwrap_do!(expression, do)
///```
/// is shorthand for
///```
/// match expression {
///     Ok(ret) => ret,
///     Err(_) => {
///         do
///     }
/// }
/// ```
#[macro_export]
macro_rules! ok_unwrap_do {
    ($e:expr, $d:expr) => {
        match { $e } {
            Ok(ret) => ret,
            Err(_) => $d
        }
    };
}
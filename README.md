A set of macros for returns and breaks in the ```None``` and ```Err``` cases.
Replaces repetitive code similar to
```
match expression {
    Some(ret) => ret,
    None => return
}
```
with simpler macros such as
```
unwrap_return!(expression)
```
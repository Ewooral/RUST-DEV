```rs
  let mut guess = String::new();

```
The `:: syntax` in the `::new` line indicates that new is an `associated function`of the String type. An associated function is a function that’s implemented on a type, in this case String. This `new function` creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
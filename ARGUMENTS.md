# Ideas about arguments parsing

## Arguments types

### Short Options :

Example : *-a*

### Long Options  :

Example : *--all*

### Parameters :

Example : *toto*

## Parser main function

##### Prototype

```rust
fn parse_args(&self) => Result<Arguments, Err>;
```

This function should compare the current arguments of the program with the list of option defined for it.

If a required option is missing in the program's arguments, this function should raise an error.

If a given option were not specified for this program, this function should also raise an error.


# Postfix Calculator Desu

Simple stack based postfix calculator written in rust.

I was bored in class...

## Quickstart

```console
$ cargo r --release
```

### Supported operators

```
'+' => Addition (pop 2, push their sum)
'-' => Subtraction (pop 2, push their difference)
'*' => Multiplication (pop 2, push their multiple)
'/' => Divition (pop 2, push their division)
```

##### Note: Division divides penultimate elem by top elem

### Example:

```console
$ cargo r --release

calc-desu> 10
calc-desu> 2
calc-desu> 3
calc-desu> *
calc-desu> *
calc-desu> p
60
```

### License

Postfix Calculator Desu is provided under the GPLv2 license. See [LICENSE](LICENSE).

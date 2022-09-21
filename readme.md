## sphx

a cli token generator built with rust.

### usage

```bash
sphx

USAGE:
    sphx [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
    -l, --length <LENGTH>      [default: 8]
    -p, --pattern <PATTERN>    [default: Aa0!]
```

### examples

#### create a token with only uppercase or numeric characters, and a length of 5
```
sphx -l 5 -p A0
```

#### create a token with only lowercase and special characters
```
sphx -p a!
```

# rust_calculator

Expression calculator.

```
USAGE:
    expr_calc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -i, --interactive    Start interactive shell
    -l, --list           Supported operations list
    -V, --version        Prints version information

OPTIONS:
    -c, --calculate <expr>    Calculate expression. 
                              E.g "-10+sin(23)/(2e10 -1.3)"

```

## Example
```
# ./expr_calc --calculate="-2*(11+sin(pi^e)) + 2e-3"
-21.0957791914363347596
# ./expr_calc -i                                    
Type 'exit' for exit and 'list' for supported operations list.
> list
Supported operations
(x+y)
(x-y)
(x*y)
(x/y)
(x^y)
sqrt(x)
sin(x)
cos(x)
ln(x)
log2(x)
log10(x)
exp(x)
pi
e
> -2*(11+sin(pi^e)) + 2e-3
-21.0957791914363347596
> -2*(11+sin(pi^e)) + 2k-3
-2*(11+sin(pi^e)) + 2k-3
                    ^
[syntax error] '2k' at position 20
> exit
#
```
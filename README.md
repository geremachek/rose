<h1 align="center">rose 🌹</h1>

<p align="center">A dead simple prefix/polish notation calculator</p>
<br><br>

**Usage:**

```
rose 1.0
A simple prefix calculator

USAGE:
    rose [FLAGS] [EXPRESSION]

FLAGS:
    -e, --evaluate    Evaluate stdin line by line
    -f, --format      Don't format output
    -h, --help        Prints help information
    -q, --quiet       Don't print the version message on startup
    -s, --silent      Supress output
    -V, --version     Prints version information

ARGS:
    <EXPRESSION>    Expression to evaluate (requires the '-e' flag)
```

**Syntax:**

(starting rose in interactive mode by running without any arguments)

```
+ 5 6
  => 11
- 5 6
  => -1
* 5 6
  => 30
/ 5 6
  => 0.8333333333333334
^ 5 6
  => 15625
root 25 2
  => 5
pi
  => 3.141592653589793
e
  => 2.718281828459045
put
  => 5
memory
pi => 3.141592653589793
$ => 5
e => 2.718281828459045
format
+ 5 5
10
silent
+ $ 5
p
15
```

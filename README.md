<h1 align="center">rose 🌹</h1>

<p align="center">A dead simple prefix/polish notation calculator</p>
<br><br>

**Usage:**

```
rose 2.0
A simple (reverse) polish notation calculator

USAGE:
    rose [FLAGS] [EXPRESSION]

FLAGS:
    -e, --evaluate    Evaluate stdin line by line
    -f, --format      Don't format output
    -h, --help        Prints help information
    -r, --reverse     Enables RPN mode
    -s, --silent      Supress output
    -V, --version     Prints version information

ARGS:
    <EXPRESSION>    Expression to evaluate (requires the '-e' flag)
```

**Syntax:**

(starting rose in interactive mode by running without any arguments)

```
# Basic Operations

+ 5 6
  -> 11

- 5 6
  -> -1

* 5 6
  -> 30

/ 5 6
  -> 0.8333333333333334

^ 5 6
  -> 15625

# Functions

root 25 # By default root does square roots
  -> 5

root 125 3 # ...But you can do others
  -> 5

log 10 # Log does base 10 by default
  -> 1

log 5 5 # But again, you can do others
  -> 1

ln e # "e" is a variable in this context
  -> 1

# you can also do sin, cos, tan, asin acos, atan, etc. 

# Variables

pi # π
  -> 3.141592653589793

e
  -> 2.718281828459045

tau # τ 
  -> 6.283185307179586

Ans # result of the previous expression
  -> 6.283185307179586

# Commands

put
  -> 6.283185307179586

memory # list the variables stored in memory
Ans: 6.283185307179586
π: 3.141592653589793
e: 2.718281828459045
τ: 6.283185307179586
pi: 3.141592653589793
tau: 6.283185307179586

format # remove the "->" prefix from results

silent # don't echo answers, unless  you use put

reverse # enable reverse polish notation

# Other

+ (+ 5 5) 1
  -> 11
```

**Configuration:**

`ROSE_PROMPT="Your prompt"`
`ROSE_FORMAT_PREFIX="Format prefix"`
`ROSE_FORMAT_POSTFIX="Format prefix"`

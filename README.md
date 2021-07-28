# 🌹 rose

A dead simple prefix/polish notation evaluator and stack calculator

**Usage:**

```
A simple (reverse) polish notation calculator

USAGE:
    rose [FLAGS] [EXPRESSION]

FLAGS:
    -e, --evaluate    Evaluate stdin line by line
    -f, --format      Don't format output
    -h, --help        Prints help information
    -r, --reverse     Enables RPN mode
    -s, --silent      Supress output
    -S, --stack       Use a stack based calculator
    -V, --version     Prints version information

ARGS:
    <EXPRESSION>    Expression to evaluate (requires the '-e' flag)
```

**Syntax:**

(starting rose in interactive mode by running without any arguments)

**Evaluator:**

```
# Basic Operations

+ 5 6 1
  -> 12

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

! 5
  -> 120

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

$ # result of the previous expression
  -> 6.283185307179586

set X 5 # set a variable

X
  -> 5

set Y root 25 # set a variable to the result of an expression

Y
  -> 5

# Commands

put
  -> 6.283185307179586

memory # list the variables stored in memory
$: 6.283185307179586
e: 2.718281828459045
pi: 3.141592653589793
tau: 6.283185307179586

format # remove the "->" prefix from results

silent # don't echo answers, unless  you use put

reverse # enable reverse polish notation

# Other

+ (+ 5 5) 1
  -> 11
```

**Stack Calculator:**

```
# Same commands (excluding reverse and set) as evaluator

1 2 3 # add three numbers to the stack

+ # add the last two
  -> 5

stack # list everything on the stack
1 5

# 2 and 3 were removed after being added

125 3 root # you may also string commands together on 
  -> 5

clear # clear the stack

1 5 2 9 # put some random data on the stack

reverse # reverse the stack

stack
9 2 5 1

twirl # swap the last two values of the stack

stack
9 2 1 5

pop # "pop off" the last element

stack
9 2 1
```

**Configuration:**

`ROSE_PROMPT="Your prompt"`

`ROSE_FORMAT_PREFIX="Format prefix"`

`ROSE_FORMAT_POSTFIX="Format prefix"`

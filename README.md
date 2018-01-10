# Example Python Module That Contains Rust Code

## Purpose
This module exists to demonstrate a roadblock that I hit when was trying out the following [example](https://github.com/PyO3/pyo3/tree/master/examples/word-count). It is heavily based on that code.

# Steps to Reproduce
1. Clone this repository
2. `cd` into the folder
** Optionally, create a python virtualenv to manage the python packages
3. Run `python setup.py install`
4. From the same folder, run `python` and then in the REPL enter the following:
```
from pyrust_example import example_fn

example_fn('Length of this string is')
```

If that behaves as expected, the following error will appear:
```
Python 3.6.4 (default, Dec 23 2017, 19:07:07) 
[GCC 7.2.1 20171128] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyrust_example import example_fn
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
  File "/home/matt/code/rust/pyrust-example/pyrust_example/__init__.py", line 4, in <module>
    from ._pyrust_example import example_fn
ModuleNotFoundError: No module named 'pyrust_example._pyrust_example'
```

If you `cd` to `/tmp` and run the same python code in the REPL, it succeeds. It also succeeds if you run `python setup.py develop` after/instead of running `install`.

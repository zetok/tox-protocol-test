

Test layer between [hstox](https://github.com/TokTok/hstox) and
[zetox](https://github.com/zetok/tox).

Additional docs for writing tests are at
https://toktok.github.io/spec#test-protocol

Tests are ran by linking test executable under `hstox/test/` directory and
issuing `cabal test`.

Some setup for `hstox` and `zetox` is required. For more info, refer to their
docs.

Running all tests is tad slow, thus one can run a single test with:

```
./dist/build/test-tox/test-tox -m <test name>
```

TODO: add travis

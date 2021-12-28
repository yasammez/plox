# Plox

If you need to debug the TCP-interaction of two pieces of software but compliance won't let you install wireshark,
this is your next best option.

# Installation

```
cargo install --git https://github.com/yasammez/plox.git
```

# Usage

```
plox <listen addr> <forward addr>
```

Then start up your microservices, macroservices, frontends, backends or whatever piece of code doesn't behave well at
the moment. Just make sure that the connecting side doesn't directly connect to the binding side but uses the listen
address instead. In the same fashion, the binding service binds to the forward address. Now the two services should be
able to communicate normally, but additionally, every byte that's on wire gets outputted to stdout and stderr of plox.
This way you can easily find out, which part isn't sending or respecting that crucial HTTP-header.

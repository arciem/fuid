# Friendly Universal Identifier (FUID)

FUIDs are wrapped 128-bit unsigned integers, which gives them the same
resolution as a UUID. FUIDs are serialized to Base62, which is a sequence of
digits and alphanumerics. This makes them shorter and easier to handle than
normal UUID encoding, yet when generated randomly they use a UUID generation
algorithm and are therefore isomorphic with UUIDs. One advantage of using
FUIDs is that they can be converted from and to short strings that can stand
in as human-readable identifiers for testing purposes, but can be
full-length random numbers for production purposes.

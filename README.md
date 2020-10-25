# mgs

![Rust](https://github.com/peterallin/mgs/workflows/Rust/badge.svg)

msg - short for "multi git status" - is a small utility for getting an overview of the status of multiple git repositories. It takes a path as a command line parameter, and lists the status of any git repositories below that path. For each repository with changes, it will list:

* the number of added files (shown as the number followed by a + sign)
* the number of modified files (shown as the number followed my an M)
* the number of removed files (shown as the number followed by a - sign)
* the number of files with conflicts (shown as the number by a C)

It is planned to also show the number of commits the repositories are ahead of their origins, but this is not implemented yet.

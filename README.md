# sharedlib [![Travis CI][tcii]][tci] [![Appveyor CI][acii]][aci]

[tcii]: https://travis-ci.org/tyleo/sharedlib.svg?branch=master
[tci]: https://travis-ci.org/tyleo/sharedlib
[acii]: https://ci.appveyor.com/api/projects/status/95wp614fd08o8rus?svg=true
[aci]: https://ci.appveyor.com/project/tyleo/sharedlib

This project has been forked from rust_libloading. While rust_libloading provides a useful interface for loading shared libraries, it is inconvienient and provides incorrect safety gaurantees. This fork seeks to correct those gurantees by properly marking unsafe regions and by not requiring clients to transmute symbols.

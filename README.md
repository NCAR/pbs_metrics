## PBS bindings
- need to set LIBCLANG_PATH to llvm-project/build/lib
- need llvm-project/build/bin in PATH
  - put these libs in ~/bin
- also need to module load gnu
- /opt/pbs/lib needs to be in LD_LIBRARY_PATH
- will need to uncomment the bindings related sections in build.rs as well
  - these are commented out so clang isn't required to build, when bindings are already generated


## TODO
- figure out how to include bindgen bindings.rs for pbs headers from code, instead of having to copy it from `target/debug/build/pbs_metrics-*/out/bindings.rs` to src


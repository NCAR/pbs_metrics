- need to set LIBCLANG_PATH to llvm-project/build/lib
- need llvm-project/build/bin in PATH
  - put these libs in ~/bin
- also need to module load gnu/12.1.0
- /opt/pbs/lib needs to be in LD_LIBRARY_PATH


## TODO
- figure out how to include bindgen bindings.rs for pbs headers from code, instead of having to copy it from `target/debug/build/pbs_metrics-*/out/bindings.rs` to src

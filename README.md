template for a new command line app.

Libs:
- clap
  - cargo add clap --features derive
  - Create a struct Args
  - Create a get_args()
  - Create a run() function that takes the args
- include anyhow::Result
  - cargo add anyhow
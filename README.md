# jamq-prototype-fileio

Simple prototype for file io operations as a learning tool and a first step towards a larger project.

Prototype will create a new file in the specified path with the contents of "Hello, world!"

Files will be named with the timestamp value of duration_since(UNIX_EPOCH) in milliseconds.

Prototype is written using only std lib for all functionality, knowing many recommend using the Chrono crate for datetime values.
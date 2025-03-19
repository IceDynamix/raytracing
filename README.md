# raytracing

following the [ray tracer construction kit](https://matklad.github.io/2022/12/31/raytracer-construction-kit.html)


## features

raytracing implementation _from scratch_. this means _no_ dependencies (aside from std)!
yes, there's a handwritten implementation of _everything_

- image output as [.ppm](https://en.wikipedia.org/wiki/Netpbm) in `./artifacts`
- all the vector and matrix math derived by hand
  - ray intersection with spheres
  - ray intersection with infinite planes
  - (todo) ray intersection with meshes from .obj files
- light calculation with the phong reflection model
- optimization
  - (todo) multithreading for faster computation
  - (todo)
- my own handwritten "ababa" config language
  - tokenizing
  - AST
  - proc macro for structs (ok this is the only place i used dependencies LOL although tbf i could implement it by hand)

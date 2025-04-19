```
attempt at basic raytracing in order to learn about rust
* https://matklad.github.io/2022/12/31/raytracer-construction-kit.html
* https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html
    - https://www.desmos.com/3d/uzapppri9x

implementing add/sub/mult traits for Vector3D:
* https://stackoverflow.com/questions/76370125/why-does-rust-opsadd-want-me-to-implement-add-with-a-move
* many ways to impl add (types explicitly shown)
<<<
impl Add<Vector3D>  for Vector3D  { ... } // Vector3D + Vector3D   -> Output
impl Add<Vector3D>  for &Vector3D { ... } // Vector3D + &Vector3D  -> Output
impl Add<&Vector3D> for Vector3D  { ... } // &Vector3D + Vector3D  -> Output
impl Add<&Vector3D> for &Vector3D { ... } // &Vector3D + &Vector3D -> Output
>>>
* im probably just going to stick with using methods for Vector3D as it makes more sense to me
* see if there is a way to do (Vector3D * int) as a way to replace scalar

```

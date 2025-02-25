# lua-benchmark

This is a small CLI tool I put together in a few minutes to help visualize Big O time complexity for various algorithms. It benchmarks the code at various N values and exports them to a CSV file, which you can then graph with external software.

### Features
You can configure the type of lua being used by changing the feature of the `mlua` dependency to something else (i.e. `luajit`).


### Examples
![linear](doc/o(n).png)
![quadratic](doc/o(n^2).png)
# A Trig-less Line of Sight Algorithm in Two Dimensions

In many examples of 2D line-of-sight algorithms, expensive operations like trigonometry are used. Additionally, some methods have intentional inaccuracies in them for the sake of simplicity. Here, we give an algorithm which does not fudge the numbers, and uses only basic arithmetic: addition, subtraction, multiplication, and division. This is not intended to replace the existing algorithms, or even be more efficient in practice.

The algorithm is implemented in Rust. The repo contains a simple example application written using [ggez](https://crates.io/crates/ggez) in addition to the algorithm itself, which can be downloaded and run by cloning the repo and using cargo. `main.rs` contains the code for the application, `sight.rs` contains the line of sight algorithm, and `space.rs` contains the structures and helper methods necessary to make it work.

I encourage you to check out the book available here: [https://basstabs.github.io/2d-line-of-sight/](https://basstabs.github.io/2d-line-of-sight/)

It walks through the math behind making the algorithm work and includes plenty of visual diagrams to help explain it. Hopefully, it is more enlightening than just reading the code. It also includes references to some of the other excellent articles on this subject available on the web.

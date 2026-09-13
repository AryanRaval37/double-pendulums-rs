# Double Pendulums in Rust
Basically an experimental repo to play around with a bunch of stuff related to pendulums and related simulations.
Built using [Nannou](https://nannou.cc/)
Used help from this ancient [repo](https://github.com/MonliH/pendulum/tree/master)

#### Edit 13/9/26:
An evenings worth of work looking into animation in rust, bevy game engine, nannou docs and the github repo. Double pendulum simulation now working.

Todo 
Estimating motion using lagrangian neural networks 
That will likely involve going down the [Burn](https://docs.rs/burn/latest/burn/) rabbit hole which in turn will cause going down making burn run on apple silicon gpus rabbit hole.  
Eventually should get LNNs working  
Then move to balancing the pendulum on a cart. Inspired by [this](https://www.youtube.com/watch?v=fWQWX9-8_sA) though I predict more sophisticated algorithms will be required... Explore neuro evolution.  

I now realize how sloooooowwwwwwww Rust compiles. It took forever to compile Nannou with when I changed optimisation level to 3.

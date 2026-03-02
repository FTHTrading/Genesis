# The Cliff

There's a number buried in our sensitivity analysis that I keep thinking about.

At P_floor = 3 (our default collapse definition): **0% collapse** across 120 worlds.
At P_floor = 5: **5.8% collapse**.
At P_floor = 10: **97.5% collapse**.

That's not a gradual degradation. That's a cliff.

The system isn't robust to collapse in general. It's robust to collapse *under one specific operationalization of failure*. Change the definition from "sustained below 3 agents" to "sustained below 10 agents" and the zero-collapse result inverts almost entirely.

What does P_floor = 3 mean mechanically? The engine has a hard architectural constraint that prevents populations from reaching true extinction — an extinction floor that catches anything below 3 before it can reach zero. At P_floor = 3, that floor is doing the work. At P_floor = 10, populations enter the 3–10 agent band, sustain there for 50+ epochs, and now we're calling that collapse.

So the honest version of our headline result is:

> Zero collapses, given a collapse definition that permits populations to compress to a residual 3-agent band indefinitely without triggering the criterion.

That's documented. That's in the paper. But the cliff is the part that makes it interesting.

The boundary isn't somewhere out in unexplored parameter space. It's right here, at the definition layer. We measured it. It sits between floors 5 and 10.

What's between those two numbers? That's where the actual stability question lives.

---

[Genesis Protocol](https://github.com/FTHTrading/Genesis) — 6,820 worlds, 44 experiments, open source.

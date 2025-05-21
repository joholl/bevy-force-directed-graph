# Calculating Positions

You think you know how to do this - but trust me, you probably don't. Bear with
me...

## The Problem to Solve

The problem is simple: we want to simulate the position of bodies over time. The
input is **forces** which act on these bodies and the output is their
**positions** over
time.

The force is directly proportional to the acceleration. So in even easier terms:
**acceleration** in, **positions** out.

```math
\vec{F}(t) = m \cdot \vec{a}(t)
```

## The Simple Solution

The acceleration is the second integration of the position.

```math
\vec{a}(t) = \frac{d\vec{v}(t)}{dt} = \frac{d^2\vec{s}(t)}{dt^2}
```

Or the other way round, to get the position at time `t` (`s(t)`), we need to
integrate over the velocity over time. And to get a velocity at time `t` (`v(t)`),
we need to integrate over the acceleration over time.

```math
\displaylines{
    \vec{s}(t) = \vec{s}_0 + \int_0^t \vec{v}(\tau)d\tau \\[10pt]
    \vec{s}(t) = \vec{s}_0 + \int_0^t \left( \vec{v}_0 + \int_0^\tau \vec{a}(T)dT \right)d\tau
}
```

Anyone who has ever programmed, knows how simple this is:

```math
\displaylines{
    \textrm{given:} \quad \vec a_i = \frac{\vec F_i}{m} \\[10pt]

    \vec s_{i+1} = \vec s_{i} + \vec v_i Δt \\[10pt]
    \vec v_{i+1} = \vec v_{i} + \vec a_i Δt
}
```

This is called the [explicit Euler
integrator](https://en.wikipedia.org/wiki/Euler_method). And it's not good enough.

## Why Not Simple?

The problem with the Euler integrator is that whenever we integrate with
discrete time steps, we introduce errors. Let's assume a constant acceleration
`a = 1` in an ideal physical world.

For timestep `t = 0`, we have exact information:

```math
\displaylines{
    \textrm{given:} \quad a = 1\mathrm{\frac{m}{s^2}}, \quad Δt=1\mathrm{s}, \quad s_0=0\mathrm{m}, \quad v_0=0\mathrm{\frac{m}{s}} \\[10pt]


    s_1 = s_0 + v_0 Δt = 0\mathrm{m} + 0\mathrm{\frac{m}{s}} \cdot 1\mathrm{s} = 0\mathrm{m} \\[10pt]
    v_1 = v_0 + a Δt = 0\mathrm{\frac{m}{s}} + 1\mathrm{\frac{m}{s^2}} \cdot 1\mathrm{s} = 1\mathrm{\frac{m}{s}}
}
```

That is already quite incorrect. With continuous time, we have:

```math
\displaylines{
    s(t) = s_0 + \int_0^{t} v(\tau) d\tau = 0\mathrm{m} + \int_0^{t} \tau \mathrm{\frac{m}{s^2}} d\tau = \frac{1}{2} t^2 \mathrm{\frac{m}{s^2}} \qquad
    s(t=1\mathrm{s}) = \frac{1}{2}\mathrm{m} \\[10pt]
    v(t) = v_0 + \int_0^{t} a(\tau) d\tau = 0\mathrm{\frac{m}{s}} + \int_0^{t} 1\mathrm{\frac{m}{s^2}} d\tau = t \mathrm{\frac{m}{s^2}} \qquad
    v(t=1\mathrm{s}) = 1\mathrm{\frac{m}{s}} \\[10pt]
}
```

Even the first time step is way off. Why? Because we ignore the change in
velocity between the first and second time step and just assume that `v_0` stays
the same between `v_0` and `v_1` - while actually it increases constantly.



![Euler method](https://upload.wikimedia.org/wikipedia/commons/1/10/Euler_method.svg)


The acceleration `a` is the second derivative of the position `s` of a given
body (here a vector consisting of `x`, `y`).

```math
\vec{a}(t) = \frac{d\vec{v}(t)}{dt} = \frac{d^2\vec{s}(t)}{dt^2}
```

To make things worse, the Euler integrator is based on the forward difference.

![Forward difference](https://upload.wikimedia.org/wikipedia/commons/9/90/Finite_difference_method.svg)

This will lead to the overall velocity to increase compared to the real thing
(i.e. the values in the continuous time domain). That means things get faster,
balls jump higher than they started out etc. Needless to say, that is really bad
for numerical stability.

In other words, systems where e.g. multiple bodies are connected with
constraints like springs start oscillating more and more over time.


## The Actual Solution -  Verlet Integration

The acceleration `a` is the second derivative of the position `s` of a given
body (here a vector consisting of `x`, `y`).

```math
\vec{a}(t) = \frac{d\vec{v}(t)}{dt} = \frac{d^2\vec{s}(t)}{dt^2}
```

Let's discretize the time axis. Now the infinitesimal small time step `dt` becomes a more tangible `Δt`.

Also, let's introduce this less verbose notation more rigorously:

```math
\vec{a}_i = \vec{a}(i \cdot Δt)
```

```
        |                                                        .
        |                                                      ..
        |                                                     .
s_(i+1) +---------------------------------------------------X.
        |                                                 ..|
        |                                                .  |
        |                                              ..   |
        |                                            ..     |
        |                                          ..       |
        |                                       ...         |
        |                                     ..            |
        |                                   ..              |
        |                                ...                |
        |                             ...                   |
    s_i +--------------------------.X.                      |
        |                     ..... |                       |
        |                .....      |                       |
s_(i-1) +---X............           |                       |
        |...|                       |                       |
    ----+---+-----------------------+-----------------------+-------
        |   |                       |                       |
        |   t_(i-1)                t_i                 t_(i+1)
        |   |←         Δt        →|←         Δt        →|
```

The verlet integration is based on the [central
difference](https://en.wikipedia.org/wiki/Finite_difference#Basic_types)
approximation of the acceleration. That means we approximate the acceleration `a_i` as
the velocity half a timestep before (`v_(i - 1/2)`) and after (`v_(i + 1/2)`) devided by `Δt`:

```math
\displaylines{
    \vec{a}_i = \frac{Δ\vec{v}_i}{Δt} = \frac{Δ^2\vec{s}_i}{Δ t^2} \\[10pt]
    = \frac{\vec{v}_{i+\frac{1}{2}} - \vec{v}_{i-\frac{1}{2}}}{Δ t} \\[10pt]
    = \frac{\frac{\vec{s}_{i+1} - \vec{s}_i}{Δt} - \frac{\vec{s}_i - \vec{s}_{i-1}}{Δt}}{Δt} \\[10pt]
    = \frac{\vec{s}_{i+1} - 2 \vec{s}_i + \vec{s}_{i-1}}{Δt^2}
}
```

To get the verlet integration equation, let's just solve for `x_(i+1)`:

```math
\vec{s}_{i+1} = 2 \vec{s}_i - \vec{s}_{i-1} + \vec{a}_i Δt^2
```

### Non-constant Time Steps

If we have a non-constant time step `Δt_i`.

```math
t_{i+1} = t_i + Δt_i
```


```
        |                                                        .
        |                                                      ..
        |                                                     .
s_(i+1) +---------------------------------------------------X.
        |                                                 ..|
        |                                                .  |
        |                                              ..   |
        |                                            ..     |
        |                                          ..       |
        |                                       ...         |
        |                                     ..            |
        |                                   ..              |
        |                                ...                |
        |                             ...                   |
    s_i +--------------------------.X.                      |
        |                     ..... |                       |
        |                .....      |                       |
s_(i-1) +---X............           |                       |
        |...|                       |                       |
    ----+---+-----------------------+-----------------------+-------
        |   |                       |                       |
        |   t_(i-1)                t_i                 t_(i+1)
        |   |←     Δt_(i-1)      →|←        Δti        →|
```

Let's start again, this time with `Δt_i`:

```math
\displaylines{
    \vec{a}_i = \frac{Δ\vec{v}_i}{\frac{1}{2}Δt_{i-1} + \frac{1}{2}Δt_{i}} \\[10pt]
    = \frac{\vec{v}_{i+\frac{1}{2}} - \vec{v}_{i-\frac{1}{2}}}{\frac{1}{2}(Δt_{i-1} + Δt_{i})} \\[10pt]
    = \frac{\frac{\vec{s}_{i+1} - \vec{s}_i}{Δt_i} - \frac{\vec{s}_i - \vec{s}_{i-1}}{Δt_{i-1}}}{\frac{1}{2}(Δt_{i-1} + Δt_{i})}
}
```

To get the verlet integration equation, let's just solve for `s_(i+1)`:

```math
\vec{s}_{i+1} = \vec{s}_i + (\vec{s}_i - \vec{s}_{i-1}) \frac{Δ t_i}{Δ t_{i-1}} + \vec{a}_i\,\frac{Δ t_{i} + Δ t_{i-1}}2\,Δ t_i
```

Given a resulting force `F` on a body with mass `m`:

```math
\vec{s}_{i+1} = \vec{s}_i + (\vec{s}_i - \vec{s}_{i-1}) \frac{Δ t_i}{Δ t_{i-1}} + \frac{\vec{F}_i}m \frac{Δ t_{i} + Δ t_{i-1}}2\,Δ t_i
```

### Implementation

The following three values are all stored in the same piece of memory (as
subsequent calculation results). This saves memory which is important for large
simulations.

```math
\vec{s}_i \qquad \vec{s}_{i+1} \ ' \qquad \vec{s}_{i+1}
```

We decouple the addition of the velocity term from the application of our force.

Step 1: Adding the velocity term:

```math
\vec{s}_{i+1} \ ' = \vec{s}_i + (\vec{s}_i - \vec{s}_{i-1}) \frac{Δ t_i}{Δ t_{i-1}}
```

Step 2: Adding the resulting force:

```math
\vec{s}_{i+1} = \vec{s}_{i+1} \ ' + \frac{\vec{F}_i}m \frac{Δ t_{i} + Δ t_{i-1}}2\,Δ t_i
```

Actually, step 2 is in turn multiple steps, since multiple forces are at play:

```math
\displaylines{
    \vec{s}_{i+1} \ '' = \vec{s}_{i+1} \ ' + \frac{\vec{F}_{0,i}}m \frac{Δ t_{i} + Δ t_{i-1}}2\,Δ t_i \\[10pt]
    \vec{s}_{i+1} \ ''' = \vec{s}_{i+1} \ '' + \frac{\vec{F}_{1,i}}m \frac{Δ t_{i} + Δ t_{i-1}}2\,Δ t_i \\[10pt]
    ... \\[10pt]
    \vec{s}_{i+1} = \vec{s}_{i+1} \ ''''' + \frac{\vec{F}_{n,i}}m \frac{Δ t_{i} + Δ t_{i-1}}2\,Δ t_i \\[10pt]
}
```

The order of addition of the forces is irrelevant. However, we must apply step 1
and step 2 in order.

# rc-sim

Simple numerical simulation of an ideal LCR Tank Oscillator using Euler numerical methods (for Fun!)

There are also simulations of Charging and Discharging a Capacitor (RC).

This was an attempt to simulate the circuits using first principles and fundemental techniques for fun to see how much I could remember from my University electronics and numerical methods lectures, I am fairly happy with the results, the LCR simulation is stable at low frequencies but it shows problems at audio frequencies, I probably need to refine it using a more sophisticated numerical method like RK2 or RK4.

<img width="702" height="390" alt="image" src="https://github.com/user-attachments/assets/f5c47f89-c9fa-44e6-8e50-7b134940cfca" />

The second order ODE for the LCR circuit is broken into two first order ODEs and solved iteratively:

```
// Kirchhoff's Current Law
// Il = Ir = Ic = I

// Kirchhoff’s Voltage Law
// Vr + vl +  Vc = 0

// Derivative wrt t
// dVr/dt + dVl/dt + dvVc/dt = 0

// Capacitor and Inductor
// Vl = L * dI/dt
// Ic = C * dVc/dt
// Vr = I * R

// Discrete time Simulation 
// In = In-1 + dI/dtN-1 * DT
// Vrn = InR
// Vln = Vln-1 + dVl/dtn-1 * DT
// dI/dtn = Vln/L
// Vcn = -Vrn - Vln
// dVl/dtn = -In/C - dI/dtn * R
```

Each simulation function outputs to a CSV which can then be loaded into Octave for analysis and visualisation.

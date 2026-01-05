# rc-sim

Simple numerical simulation of an ideal RCL Tank Oscillator using Euler numerical methods (for Fun!)

There are also simulations of Charging and Discharging a Capacitor (RC)

<img width="933" height="452" alt="image" src="https://github.com/user-attachments/assets/dc64b1c7-db99-4dd4-ae26-0a50e800ee6e" />

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

This simulation is stable at low frequencies but showing problems at audio frequencies, I probably need to refine it using a more sophisticated numerical method like RK2 or RK4.



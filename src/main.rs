//!
//! Some simple simulations of ideal C, R and L circuits using basic
//! Euler method.  The LCR oscillator works ok for ~10Hz, but at audio
//! frequencies it doesn't work - probbaly need to use a more refinded 
//! technique for higher frequencies.
//! 
//! This work is just for fun to see how much I can remember of college
//! electronics and working with ODEs
//! 
//! The simulation functions write their output to a CSV.
//! 
use std::fs::File;
use std::io::{Write, Result};

/// Simulation of a capacitor charging through a resistor.
/// 
fn rc_charge_sim() -> Result<()> {
    println!("RC Sim!");

    // Source voltage
    const VS: f32 = 5.0;

    // Expect full charge after about 800ms
    const C: f32 = 100E-6;
    const R: f32 = 1.6E3;

    // RC * dV/dt + Vc = Vsource
    // At the start Vc = 0, so
    // RC * dV/dt = Vsource
    // dV/dt = Vsource / RC
    let mut dvdt: f32 = 0.0;
    const RC: f32 = R * C;
    
    let mut vc = 0.0;

    // Write t and Vc to a CSV file
    let mut file = File::create("rc_charge.csv")?;

    let step:f32 = 1e-2;
    let mut t: f32 = 0.0;

    while t <= 1.0 {
        vc += dvdt * step as f32;
        dvdt = (VS - vc) / RC;

        writeln!(file, "{},{}", t, vc)?;

        t += step;
    }

    Ok(())
}

/// Simulation of a charged capacitor discharging through a resistor.
fn rc_discharge_sim() -> Result<()> {
    println!("RC Sim!");

    // Source voltage
    const VS: f32 = 5.0;

    const C: f32 = 100E-6;
    const R: f32 = 1.6E3;

    // dv/dt = - Vc/RC
    // At the start Vc = 5
    let mut dvdt: f32;
    const RC: f32 = R * C;
    
    let mut vc = VS;

    let mut file = File::create("rc_discharge.csv")?;

    let step:f32 = 1e-2;
    let mut t: f32 = 0.0;

    dvdt = vc /  RC;

    while t <= 1.0 {
        vc += dvdt * step as f32;
        dvdt = -vc / RC;

        writeln!(file, "{},{}", t, vc)?;

        t += step;
    }

    Ok(())
}


/// Simulation of an LC Tank Oscillator
fn lc_sim() -> Result<Vec<f32>> {
    println!("LC Sim!");

    // Vl(t) = L * dIl(t)/dt
    // Ic(t) = C * dVc/dt
    // Ic = Il
    // Vc + Vl = 0, Vc = -Vl

    // Vc -> Vl -> di/dt -> Ic -> dv/dt -> Vc(N+1)

    let mut waveform = Vec::with_capacity(10000);

    // Source voltage
    const VS: f32 = 5.0;

    let mut vc = VS;
    let mut vl: f32;

    let mut file = File::create("lc_sim.csv")?;

    let step:f32 = 1e-3;
    let mut dvdt: f32;
    let mut didt: f32;

    // Expect 10Hz oscillation.
    const C: f32 = 2533E-6;
    const L: f32 = 0.1;

    let mut t: f32 = 0.0;
    let mut i: f32 = 0.0;

    while t <= 1.0 {
        vl = -vc;
        didt = vl / L;
        i += didt * step;
        dvdt = i / C;
        vc += dvdt * step;

        writeln!(file, "{},{}", t, vc)?;
        waveform.push(vc);
        t += step;
    }

    Ok(waveform)
}


///
/// Simple simulation of an ideal RCL oscillator.
/// 
fn lcr_sim() -> Result<Vec<f32>> {
    println!("LCR Sim!");

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

    let mut waveform = Vec::with_capacity(10000);

    // Write t and Vc to a csv file.
    let mut file = File::create("lcr_sim.csv")?;

    let dt:f32 = 1e-3;

    // Source voltage
    const VS: f32 = 5.0;

    const C: f32 = 2533E-6;
    const L: f32 = 0.1;
    const R: f32 = 1.5;

    let mut t: f32 = 0.0;
    let mut i: f32 = 0.0;

    // at t = 0, I = 0,
    // Vc = Vs
    // Vr = 0
    // Vl = -Vc
    // dI/dt = Vl / L
    // dVl/dt = 0

    let mut vc = VS;
    let mut vl: f32 = -vc;
    let mut vr: f32;

    let mut dvl_dt: f32 = 0.0;
    let mut di_dt: f32 = vl / L;

    while t <= 1.0 {
        i = i + di_dt * dt;
        vr = i * R;
        vl = vl + dvl_dt * dt;
        di_dt = vl / L;
        vc = -vr - vl;
        dvl_dt = -i/C - di_dt * R;

        writeln!(file, "{},{}", t, vc)?;
        waveform.push(vc);
        t += dt;
    }

    Ok(waveform)
}

fn main() -> Result<()> {
    rc_charge_sim()?;
    rc_discharge_sim()?;
    
    lc_sim()?;
    lcr_sim()?;

    Ok(())
}

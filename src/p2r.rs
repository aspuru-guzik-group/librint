use crate::scf::{integral1e, integral2e, RHF, energy, denergy};

use pyo3::prelude::*;
use pyo3::types::PyList;

#[pymodule]
fn librint(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(integral1ep, m)?)?;
    m.add_function(wrap_pyfunction!(integral2ep, m)?)?;
    m.add_function(wrap_pyfunction!(RHFp, m)?)?;
    m.add_function(wrap_pyfunction!(energyp, m)?)?;
    m.add_function(wrap_pyfunction!(grad, m)?)?;
    Ok(())
}

#[no_mangle]
#[pyfunction]
pub fn integral1ep(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atml: &PyList,
    basl: &PyList,
    envl: &PyList,
    coord: i32,
    typec: i32,
) -> PyResult<Vec<f64>> {
    let mut atm: Vec<i32> = atml.extract()?;
    let mut bas: Vec<i32> = basl.extract()?;
    let mut env: Vec<f64> = envl.extract()?;

    let mut R: Vec<f64> = integral1e(natm, nbas, nshells, &mut atm, &mut bas, &mut env, coord, typec);

    Ok(R)
}

#[no_mangle]
#[pyfunction]
pub fn integral2ep(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atml: &PyList,
    basl: &PyList,
    envl: &PyList,
    coord: i32,
) -> PyResult<Vec<f64>> {
    let mut atm: Vec<i32> = atml.extract()?;
    let mut bas: Vec<i32> = basl.extract()?;
    let mut env: Vec<f64> = envl.extract()?;

    let mut R: Vec<f64> = integral2e(natm, nbas, nshells, &mut atm, &mut bas, &mut env, coord);

    Ok(R)    
}

#[no_mangle]
#[pyfunction]
pub fn RHFp(
    natm: usize,
    nbas: usize,
    nelec: usize,
    nshells: usize,
    patm: &PyList,
    pbas: &PyList,
    penv: &PyList,
    imax: i32,
    conv: f64,
) -> PyResult<Vec<f64>> {
    let mut atm: Vec<i32> = patm.extract()?;
    let mut bas: Vec<i32> = pbas.extract()?;
    let mut env: Vec<f64> = penv.extract()?;

    let P: Vec<f64> = RHF(natm, nbas, nelec, nshells, &mut atm, &mut bas, &mut env, imax, conv);

    Ok(P)
}

#[no_mangle]
#[pyfunction]
pub fn energyp(
    natm: usize,
    nbas: usize,
    nshells: usize,
    patm: &PyList,
    pbas: &PyList,
    penv: &PyList,
    Pl: &PyList,
) -> PyResult<f64> {
    let mut atm: Vec<i32> = patm.extract()?;
    let mut bas: Vec<i32> = pbas.extract()?;
    let mut env: Vec<f64> = penv.extract()?;
    let mut P: Vec<f64> = Pl.extract()?;

    let E: f64 = energy(natm, nbas, nshells, &mut atm, &mut bas, &mut env, &mut P);

    Ok(E)
}

#[no_mangle]
#[pyfunction]
pub fn grad(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atml: &PyList,
    basl: &PyList,
    envl: &PyList,
    Pl: &PyList,
) -> PyResult<Vec<f64>> {
    let mut atm: Vec<i32> = atml.extract()?;
    let mut bas: Vec<i32> = basl.extract()?;
    let mut env: Vec<f64> = envl.extract()?;
    let mut P: Vec<f64> = Pl.extract()?;

    let mut denv: Vec<f64> = vec![0.0; 1000];

    // let _ = denergy(natm, nbas, nshells, &mut atm, &mut bas, &mut env, &mut denv, &mut P, 1.0); // ENZYME REALLOC ERROR

    Ok(denv)
}
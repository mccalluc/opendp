use std::ffi::c_char;

use crate::{
    core::{FfiResult, IntoAnyMeasurementFfiResultExt},
    domains::{AtomDomain, VectorDomain},
    ffi::{
        any::{AnyDomain, AnyMeasurement, AnyMetric, AnyObject, Downcast},
        util::{to_str, Type},
    },
    measurements::{make_base_discrete_exponential, Optimize},
    metrics::LInfDiffDistance,
    traits::{
        samplers::{CastInternalRational, SampleUniform},
        CheckNull, Float, InfCast, Number, RoundCast,
    },
};

#[no_mangle]
pub extern "C" fn opendp_measurements__make_base_discrete_exponential(
    input_domain: *const AnyDomain,
    input_metric: *const AnyMetric,
    temperature: *const AnyObject,
    optimize: *const c_char,
    QO: *const c_char,
) -> FfiResult<*mut AnyMeasurement> {
    let input_domain = try_as_ref!(input_domain);
    let input_metric = try_as_ref!(input_metric);
    let TIA = try_!(input_domain.type_.get_atom());
    let temperature = try_as_ref!(temperature);

    let optimize = match try_!(to_str(optimize)) {
        i if i.to_lowercase().starts_with("min") => Optimize::Min,
        i if i.to_lowercase().starts_with("max") => Optimize::Max,
        _ => return err!(FFI, "optimize must start with \"min\" or \"max\"").into(),
    };
    let QO = try_!(Type::try_from(QO));

    fn monomorphize<TIA, QO>(
        input_domain: &AnyDomain,
        input_metric: &AnyMetric,
        temperature: &AnyObject,
        optimize: Optimize,
    ) -> FfiResult<*mut AnyMeasurement>
    where
        TIA: Clone + CheckNull + Number + CastInternalRational,
        QO: 'static + InfCast<TIA> + RoundCast<TIA> + Float + SampleUniform + CastInternalRational,
    {
        let input_domain =
            try_!(input_domain.downcast_ref::<VectorDomain<AtomDomain<TIA>>>()).clone();
        let input_metric = try_!(input_metric.downcast_ref::<LInfDiffDistance<TIA>>()).clone();
        let temperature = *try_!(temperature.downcast_ref::<QO>());
        make_base_discrete_exponential::<TIA, QO>(input_domain, input_metric, temperature, optimize)
            .into_any()
    }

    dispatch!(monomorphize, [
        (TIA, [u32, u64, i32, i64, usize, f32, f64]),
        (QO, @floats)
    ], (input_domain, input_metric, temperature, optimize))
}

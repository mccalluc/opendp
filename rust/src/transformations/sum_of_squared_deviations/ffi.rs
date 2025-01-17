use std::convert::TryFrom;
use std::os::raw::c_char;

use crate::core::{FfiResult, IntoAnyTransformationFfiResultExt};
use crate::domains::{AtomDomain, VectorDomain};
use crate::err;
use crate::ffi::any::{AnyDomain, AnyMetric, AnyTransformation, Downcast};
use crate::ffi::util::Type;
use crate::metrics::SymmetricDistance;
use crate::traits::Float;
use crate::transformations::{make_sum_of_squared_deviations, Pairwise, Sequential, UncheckedSum};

#[no_mangle]
pub extern "C" fn opendp_transformations__make_sum_of_squared_deviations(
    input_domain: *const AnyDomain,
    input_metric: *const AnyMetric,
    S: *const c_char,
) -> FfiResult<*mut AnyTransformation> {
    fn monomorphize<T>(
        input_domain: &AnyDomain,
        input_metric: &AnyMetric,
        S: Type,
    ) -> FfiResult<*mut AnyTransformation>
    where
        T: 'static + Float,
    {
        fn monomorphize2<S>(
            input_domain: &AnyDomain,
            input_metric: &AnyMetric,
        ) -> FfiResult<*mut AnyTransformation>
        where
            S: UncheckedSum,
            S::Item: 'static + Float,
        {
            let input_domain =
                try_!(input_domain.downcast_ref::<VectorDomain<AtomDomain<S::Item>>>()).clone();
            let input_metric = try_!(input_metric.downcast_ref::<SymmetricDistance>()).clone();
            make_sum_of_squared_deviations::<S>(input_domain, input_metric).into_any()
        }
        dispatch!(monomorphize2, [(S, [Sequential<T>, Pairwise<T>])], (input_domain, input_metric))
    }
    let input_domain = try_as_ref!(input_domain);
    let input_metric = try_as_ref!(input_metric);
    let S = try_!(Type::try_from(S));
    let T = try_!(S.get_atom());
    dispatch!(monomorphize, [
        (T, @floats)
    ], (input_domain, input_metric, S))
}

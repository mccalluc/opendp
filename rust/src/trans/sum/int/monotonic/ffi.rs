use std::convert::TryFrom;
use std::os::raw::{c_char, c_uint};

use num::Zero;

use crate::core::{FfiResult, IntoAnyTransformationFfiResultExt};

use crate::dist::IntDistance;
use crate::err;
use crate::ffi::any::{AnyObject, AnyTransformation, Downcast};
use crate::ffi::util::Type;
use crate::traits::{AlertingAbs, CheckNull, DistanceConstant, InfSub, SaturatingAdd};
use crate::trans::{
    make_bounded_int_monotonic_sum, make_sized_bounded_int_monotonic_sum, AddIsExact, IsMonotonic,
};

#[no_mangle]
pub extern "C" fn opendp_trans__make_bounded_int_monotonic_sum(
    bounds: *const AnyObject,
    T: *const c_char,
) -> FfiResult<*mut AnyTransformation> {
    fn monomorphize<T>(bounds: *const AnyObject) -> FfiResult<*mut AnyTransformation>
    where
        T: DistanceConstant<IntDistance>
            + CheckNull
            + Zero
            + AlertingAbs
            + SaturatingAdd
            + AddIsExact
            + IsMonotonic,
    {
        let bounds = try_!(try_as_ref!(bounds).downcast_ref::<(T, T)>()).clone();
        make_bounded_int_monotonic_sum::<T>(bounds).into_any()
    }
    let T = try_!(Type::try_from(T));
    dispatch!(monomorphize, [
        (T, @integers)
    ], (bounds))
}

#[no_mangle]
pub extern "C" fn opendp_trans__make_sized_bounded_int_monotonic_sum(
    size: c_uint,
    bounds: *const AnyObject,
    T: *const c_char,
) -> FfiResult<*mut AnyTransformation> {
    fn monomorphize<T>(size: usize, bounds: *const AnyObject) -> FfiResult<*mut AnyTransformation>
    where
        T: DistanceConstant<IntDistance>
            + InfSub
            + CheckNull
            + Zero
            + SaturatingAdd
            + AddIsExact
            + IsMonotonic,
    {
        let bounds = try_!(try_as_ref!(bounds).downcast_ref::<(T, T)>()).clone();
        make_sized_bounded_int_monotonic_sum::<T>(size, bounds).into_any()
    }
    let size = size as usize;
    let T = try_!(Type::try_from(T));
    dispatch!(monomorphize, [(T, @integers)], (size, bounds))
}

#[cfg(test)]
mod tests {
    use crate::core;
    use crate::error::Fallible;
    use crate::ffi::any::{AnyObject, Downcast};
    use crate::ffi::util;
    use crate::ffi::util::ToCharP;

    use super::*;

    #[test]
    fn test_make_bounded_int_monotonic_sum_ffi() -> Fallible<()> {
        let transformation = Result::from(opendp_trans__make_bounded_int_monotonic_sum(
            util::into_raw(AnyObject::new((0i32, 10))),
            "i32".to_char_p(),
        ))?;
        let arg = AnyObject::new_raw(vec![1i32, 2, 3]);
        let res = core::opendp_core__transformation_invoke(&transformation, arg);
        let res: i32 = Fallible::from(res)?.downcast()?;
        assert_eq!(res, 6);
        Ok(())
    }

    #[test]
    fn test_make_sized_bounded_int_monotonic_sum_ffi() -> Fallible<()> {
        let transformation = Result::from(opendp_trans__make_sized_bounded_int_monotonic_sum(
            3 as c_uint,
            util::into_raw(AnyObject::new((0i32, 10))),
            "i32".to_char_p(),
        ))?;
        let arg = AnyObject::new_raw(vec![1i32, 2, 3]);
        let res = core::opendp_core__transformation_invoke(&transformation, arg);
        let res: i32 = Fallible::from(res)?.downcast()?;
        assert_eq!(res, 6);
        Ok(())
    }
}
use crate::internal::*;
use crate::ops::cnn::{KernelFormat, PaddingSpec};
use crate::ops::nn::DataFormat;

// NCHW OIHW rank=4 valid, no-stride, no-dil, no-bias, no-group, f32

#[derive(Clone, Debug, new, Hash)]
pub struct DeconvUnary {
    pub data_format: DataFormat,
    pub kernel_format: KernelFormat,
    pub padding: PaddingSpec,
    pub kernel: Arc<Tensor>,
}

impl DeconvUnary {
    pub fn output_shape<D: DimLike>(&self, x_shape: &[D]) -> TractResult<TVec<D>> {
        super::output_shape(
            &self.data_format,
            &self.kernel_format,
            &self.padding,
            &self.kernel.shape(),
            x_shape,
        )
    }
}

impl_dyn_hash!(DeconvUnary);

impl Op for DeconvUnary {
    fn name(&self) -> Cow<str> {
        "DeconvUnary".into()
    }
    op_core_mir!();
    op_as_typed_op!();
}

impl EvalOp for DeconvUnary {
    fn is_stateless(&self) -> bool {
        true
    }

    fn eval(&self, mut inputs: TVec<Arc<Tensor>>) -> TractResult<TVec<Arc<Tensor>>> {
        let input = args_1!(inputs);
        let output_shape = self.output_shape(input.shape())?;
        let mut tensor = Tensor::zero_dt(input.datum_type(), &output_shape)?;
        /*
        let input = input.to_array_view::<f32>()?.into_dimensionality()?;
        let mut output = tensor.to_array_view_mut::<f32>()?.into_dimensionality()?;
        let kernel = self.kernel.to_array_view::<f32>()?.into_dimensionality()?;
        for n in 0..input.shape()[0] {
        for co in 0..output.shape()[1] {
        for ci in 0..input.shape()[1] {
        for hi in 0..input.shape()[2] {
        for wi in 0..input.shape()[3] {
        for hk in 0..self.kernel.shape()[2] {
        for wk in 0..self.kernel.shape()[3] {
        output[(n, co, hi + hk, wi + wk)] +=
        input[(n, ci, hi, wi)] * kernel[(ci, co, hk, wk)];
        }
        }
        }
        }
        }
        }
        }
        */
        Ok(tvec!(tensor.into_arc_tensor()))
    }
}

impl TypedOp for DeconvUnary {
    fn output_facts(&self, inputs: &[&TypedFact]) -> TractResult<TVec<TypedFact>> {
        let x_fact = inputs[0];
        let output_shape = self.output_shape(&*x_fact.shape)?;
        Ok(tvec!(TypedFact::dt_shape(x_fact.datum_type, &output_shape)))
    }

    as_op!();
}

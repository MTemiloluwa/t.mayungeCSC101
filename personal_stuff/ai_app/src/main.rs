use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{Linear, Module, VarBuilder, VarMap};

fn main() -> Result <()>{
    // Select device: CPU or GPU
    let device = Device::Cpu;


    //Create random input tensor [batch_size: 1, features: 4]
    let input = Tensor::randn(0.0f32, 1.0f32, (1, 4), &device)?;

    // Define model weights (in real projects, load form file or train via Varmap)
    let weights = Tensor::randn(0.0f32, 1.0f32, (4, 2) &device, /* &Device */)?;
    let bias = Tensor::zeros((2,), DType::F32, &device)?;
    let linear = Linear::new(weights, Some(bias));


    // Forward pass 
    let output = linear.forward(&input)?;
    println!("Output Tensor:\n{}", output);


    Ok(())
} 



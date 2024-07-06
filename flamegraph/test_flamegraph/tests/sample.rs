use candle_core::{Device, Result, Tensor};

struct Model {
    first: Tensor,
    second: Tensor,
}

impl Model {
    fn forward(&self, image: &Tensor) -> Result<Tensor> {
        let x = image.matmul(&self.first)?;
        let x = x.relu()?;
        x.matmul(&self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let device = Device::Cpu;
        let first = Tensor::randn(0f32, 1.0, (784, 100), &device).unwrap();
        let second = Tensor::randn(0f32, 1.0, (100, 10), &device).unwrap();
        let model = Model { first, second };

        let dummy_image = Tensor::randn(0f32, 1.0, (1, 784), &device).unwrap();

        let _ = model.forward(&dummy_image).unwrap();
    }
}

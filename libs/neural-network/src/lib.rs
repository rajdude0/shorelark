struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

pub struct Network {
    layers: Vec<Layer>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

impl Network {
    pub fn propagte(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut inputs = inputs;

        for layers in &self.layers {
            inputs = layer.propagate(inputs);
        }

        inputs;
    }
}

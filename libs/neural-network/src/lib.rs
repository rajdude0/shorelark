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

pub struct LayerTopology {
    pub neurons: usize,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs.iter()
                            .zip(&self.weights)
                            .map(|(input, weight)| input * weight)
                            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pun fn random(output_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }


}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(
        input_neurons: usize, 
        output_neurons: usize
        ) -> Self {
            let neurons = (0..output_neurons)
                .map(|_| Neuron::random(input_neurons))
                .collect();
        
            Self { neurons } 
    }
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers
        }
    }
    
    pub fn random(layers: &[LayerTopology]) -> Self {
            assert!(layers.len() > 1);
            let layers = layers
                .windows(2)
                .map(|adjacentLayers| {
                    Layer::random(adjacentLayers[0].neurons, adjacentLayers[1].neurons)
                })
                .collect();

            Self { layers } 
        }
    }

    pub fn propagte(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

fn main() {
}

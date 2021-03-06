
use kongodjan::{
    maths::types::MatrixD,
    utils::{synthetic_data_mat},
    layers::multi_layers::FcLayer,
    activators::non_linear::{logsig, logsig_deriv/*, tansig, tansig_deriv*/},
    network_arch::PerceptronNetwork,
    loss_functions::{squared_loss, squared_loss_gradient},
    neural_traits::NetworkT,
    optimizers::sgd
};


#[test]
fn linear_regression() {

    // rows are neurons and columns are inputs
    // 7 inputs for 3 neurons
    let true_w = MatrixD::<f64>::from_row_slice(3, 7, &[
        2.0, -3.4, 2.0, -3.4, 2.0, -3.4, 1.0,
        2.0, -3.4, 2.0, -3.4, 2.0, -3.4, 1.0,
        2.0, -3.4, 2.0, -3.4, 2.0, -3.4, 1.0
    ]);

    let true_b = MatrixD::<f64>::from_row_slice(3, 1, &[
        2.0,
        -3.4,
        1.0
    ]);
    
    // the network is one layer with one neuron which receives two inputs
    // then features are two rows(two inputs) and more columns
    // labels one row and more columns
    let (features, labels) = synthetic_data_mat(&true_w, true_b, 1000);
    
    // build layer
    let n_neurons = 4;
    let n_inputs = 7; 
    let layer1 = FcLayer::new(n_neurons, n_inputs, logsig, Some(logsig_deriv), 1);
    let layer2 = FcLayer::new(3, 4, logsig, Some(logsig_deriv), 2);

    let layers = vec![layer1, layer2];

    // build the network
    let mut network = PerceptronNetwork::new(features, layers, labels);

    // train the networ
    network.train(0.1, Some(3), (squared_loss, squared_loss_gradient,sgd), 10);

    //let pred = network.predict(&test_x);
    //println!("output: {:?}", pred);
}

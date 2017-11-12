extern crate nn_rust;
extern crate time;
extern crate rand;

use nn_rust::FeedForward;
use nn_rust::data::{DataSet, Extractable};

use rand::distributions::IndependentSample;
use rand::distributions::range::Range;
use rand::distributions::normal::Normal;

use nn_rust::activators;
use nn_rust::activators::tanh;
use nn_rust::activators::Type::Tanh;
use nn_rust::activators::Type::Sigmoid;
use nn_rust::estimators;


#[test]
fn xor(){
    let allowed_error = 0.08; // Max allowed error is 8%
    let mut nn = FeedForward::new(&[2, 2, 1]);
    let sc = &[
        (&[0f64, 0f64], &[0f64]),
        (&[1f64, 0f64], &[1f64]),
        (&[0f64, 1f64], &[1f64]),
        (&[1f64, 1f64], &[0f64]),
    ];
    let mut k;
    let rnd_range = Range::new(0, sc.len());
    let prev = time::now_utc();

    for _ in 0..30_000{
        k = rnd_range.ind_sample(&mut rand::thread_rng());
        nn.fit(sc[k].0, sc[k].1);
    }

    let mut res;
    for v in sc{
        res = nn.calc(v.0)[0];
        println!("for [{:.3}, {:.3}], [{:.3}] -> [{:.3}]",
                 v.0[0], v.0[1], v.1[0], res);

        if (res - v.1[0]).abs() > allowed_error{
            assert!(false);
        }
    }

    println!("\nSpend time: {:.5}", (time::now_utc() - prev));
    assert!(true);
}

#[test]
fn xor_through_data_set_and_train(){
    const allowed_error: f64 = 0.08; // Max allowed error is 8%
    let mut nn = FeedForward::new(&[2, 2, 1]);
    let mut data = DataSet::new();

    data.push(&[0f64, 0f64], &[0f64]);
    data.push(&[1f64, 0f64], &[1f64]);
    data.push(&[0f64, 1f64], &[1f64]);
    data.push(&[1f64, 1f64], &[0f64]);

    nn.activation(activators::Type::Tanh)
        .learning_rate(0.1)
        .momentum(0.15)
        .train(&data, 30_000);

    let mut res;
    let mut d;
    for i in 0..data.len(){
        res = nn.calc(data.get(i).0)[0];
        d = data.get(i);
        println!("for [{:.3}, {:.3}], [{:.3}] -> [{:.3}]", d.0[0], d.0[1], d.1[0], res);
        if (res - data.get(i).1[0]).abs() > allowed_error{
            assert!(false);
        }
    }
}

#[test]
fn binding(){
    let allowed_error = 0.08; // Max allowed error is 8%
    let mut nn = FeedForward::new(&[6, 4, 4, 2, 1]);

    println!("{}", nn);

    nn.unbind(1, 0);
    println!("{}", nn);

    nn.bind(1, 0);
    println!("{}", nn);
}

#[test]
#[ignore]
fn widrows(){
    let w = estimators::widrows(&[2, 1], 0.1);
    assert_eq!(w, 90f64);
}
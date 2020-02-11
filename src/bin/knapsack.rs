use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item {
    name: &'static str,
    weight: usize,
    value: f32,
}

fn knapsack(items: Vec<Item>, max_capacity: usize) -> Vec<Item> {
    // build up dynamic programming table
    let zero_vec = vec![0.0; max_capacity + 1];
    let mut table: Vec<Vec<f32>> = vec![zero_vec.clone(); items.len() + 1];
    for (i, item) in items.iter().enumerate() {
        for capacity in 1..(max_capacity + 1) {
            let previous_items_value: f32 = table[i][capacity];
            if capacity >= item.weight {
                // item fits in knapsack
                let value_freeing_weight_for_item: f32 = table[i][capacity - item.weight];
                // only take if more valuable than previous item
                table[i + 1][capacity] =
                    previous_items_value.max(value_freeing_weight_for_item + item.value);
            } else {
                // no room for this item
                table[i + 1][capacity] = previous_items_value;
            }
        }
    }
    // figure out solution from table
    let mut solution: Vec<Item> = Vec::new();
    let mut capacity = max_capacity;
    for i in (0..items.len()).rev() {
        // work backwards
        // was this item used?
        if table[i - 1][capacity] != table[i][capacity] {
            solution.push(items[i - 1]);
            // if the item was used, remove its weight
            capacity -= items[i - 1].weight
        }
    }
    solution
}

fn main() {
    let items: Vec<Item> = vec![
        Item {
            name: "television",
            weight: 50,
            value: 500.0,
        },
        Item {
            name: "candlesticks",
            weight: 2,
            value: 300.0,
        },
        Item {
            name: "stereo",
            weight: 35,
            value: 400.0,
        },
        Item {
            name: "laptop",
            weight: 3,
            value: 1000.0,
        },
        Item {
            name: "food",
            weight: 15,
            value: 50.0,
        },
        Item {
            name: "clothing",
            weight: 20,
            value: 800.0,
        },
        Item {
            name: "jewelry",
            weight: 1,
            value: 4000.0,
        },
        Item {
            name: "books",
            weight: 100,
            value: 300.0,
        },
        Item {
            name: "printer",
            weight: 18,
            value: 30.0,
        },
        Item {
            name: "refrigerator",
            weight: 200,
            value: 700.0,
        },
        Item {
            name: "painting",
            weight: 10,
            value: 1000.0,
        },
    ];
    println!("{:?}", knapsack(items, 75));
}

#[derive(Debug)]
struct Asset<T> {
    name: String,
    asset_type: T,
}

#[derive(Debug)]
enum AssetType {
    Stock,
    Bond,
    Option,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Stock {
    ticker: String,
    price: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Bond {
    issuer: String,
    face_value: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Option {
    underlying: String,
    strike: f64,
}

fn main() {
    let stock = Asset {
        name: "Apple Inc".to_string(),
        asset_type: AssetType::Stock,
    };

    let bond = Asset {
        name: "US Treasuty Bond".to_string(),
        asset_type: AssetType::Bond,
    };

    let option = Asset {
        name: "Call Option on Goole".to_string(),
        asset_type: AssetType::Option,
    };

    println!("Asset 1: {}({:?}", stock.name, stock.asset_type);
    println!("Asset 2: {}({:?}", bond.name, bond.asset_type);
    println!("Asset 3: {}({:?}", option.name, option.asset_type);
}

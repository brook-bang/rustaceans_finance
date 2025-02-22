enum AssetType {
    Stock,
    Bond,
    RealEstate,
}

struct Asset {
    name: String,
    asset_type: AssetType,
    price: f64,
}

struct Portfolio {
    assets: Vec<Asset>,
}

impl Portfolio {
    fn calculate_total_value(&self) -> f64 {
        let mut total_value = 0.0;
        for asset in &self.assets {
            total_value += asset.price;
        }
        total_value
    }
}

fn main() {
    let stock1 = Asset {
        name: String::from("AAPL"),
        asset_type: AssetType::Stock,
        price: 150.0,
    };

    let bond1 = Asset {
        name: String::from("Government Bond"),
        asset_type: AssetType::Bond,
        price: 1000.0,
    };

    let real_estate1 = Asset {
        name: String::from("Commercial Property"),
        asset_type: AssetType::RealEstate,
        price: 500000.0,
    };

    let mut portfolio = Portfolio {
        assets: Vec::new(),
    };

    portfolio.assets.push(stock1);
    portfolio.assets.push(bond1);
    portfolio.assets.push(real_estate1);

    let total_value = portfolio.calculate_total_value();

    println!("投资组合总价值：${}",total_value);

}

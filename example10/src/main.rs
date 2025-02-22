fn main() {
    let mut futures_contracts: Vec<String> = vec![
        "AU2012".to_string(),
        "IF2110".to_string(),
        "C2109".to_string(),
    ];

    futures_contracts.push("IH2110".to_string());

    println!("当前期货合约列表：{:?}",futures_contracts);

    let popped_contract = futures_contracts.pop();
    println!("移除的最后一个期货合约：{:?}",popped_contract);

    futures_contracts.insert(1, "IC2110".to_string());
    println!("插入新期货合约后的列表：{:?}",futures_contracts);

    let removed_contract = futures_contracts.remove(2);
    println!("移除的第三个期货合约：{:?}",removed_contract);

    println!("最终期货合约列表：{:?}",futures_contracts);

}

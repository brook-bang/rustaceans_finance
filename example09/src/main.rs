#[derive(Debug)]
enum FinancialInstrument {
    Stock,
    Option,
    Future,
}

enum OrderType {
    Market,
    Limit(f64),
    Stop(f64),
}

struct Order {
    instrument: FinancialInstrument,
    order_type: OrderType,
    quantity: i32,
}

impl Order {
    fn execute(&self) {
        match &self.order_type {
            OrderType::Market => println!("执行市价订单：{:?} x {}",self.instrument,self.quantity),
            OrderType::Limit(price) =>{
                println!("执行限价订单：{:?} x {} (价格限制：${})",self.instrument,self.quantity,price)
            }
            OrderType::Stop(trigger_price) => {
                println!("执行止损订单：{:?} x {} (触发价格：${})",self.instrument,self.quantity,trigger_price)
            }
        }
    }
}

fn main(){
    let market_order = Order {
        instrument: FinancialInstrument::Stock,
        order_type: OrderType::Market,
        quantity: 100,
    };

    let limit_order = Order {
        instrument: FinancialInstrument::Option,
        order_type: OrderType::Limit(50.0),
        quantity: 50,
    };

    let stop_order = Order {
        instrument: FinancialInstrument::Future,
        order_type: OrderType::Stop(4900.0),
        quantity: 10,
    };

    market_order.execute();
    limit_order.execute();
    stop_order.execute();
}
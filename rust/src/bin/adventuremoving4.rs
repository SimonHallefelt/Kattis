use std::io;

fn main() {
    let n = read_line().parse::<i128>().unwrap();
    let mut gas_stations = Vec::new();
    loop{
        let gas_station = read_line();
        if gas_station.len() < 3 { // must have, they send a blank line
            break;
        }
        let gas_station = gas_station.split(" ").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<i128>>();
        let gas_station = (gas_station[0], gas_station[1]);
        gas_stations.push(gas_station);
        if gas_station.0 >= n {
            break;
        }
    }

    if gas_stations.len() == 0{
        println!("Impossible");
    }else if gas_stations[0].0 > 100 {
        println!("Impossible");
    }else if n <= 100 {
        let mut min = 1000000000;
        for gas_station in gas_stations {
            if gas_station.1 < min && gas_station.0 <= n{
                min = gas_station.1;
            }
        }
        println!("{}", min*n);
    }else {
        run(n, gas_stations);
    }
}

fn run(n: i128, gas_stations: Vec<(i128, i128)>) {
    let mut min_price = 0;
    let mut current = n;
    let mut need = 100;
    let mut at_gas_station;

    if n <= gas_stations[gas_stations.len()-1].0{
        at_gas_station = gas_stations[gas_stations.len()-1];
    }else {
        at_gas_station = (0, 100000000000);
    }

    let mut last = 0;
    for i in 0..gas_stations.len() {
        if gas_stations[i].0 <= last+200 {
            last = gas_stations[i].0;
        }
    }
    if last+100 < n {
        println!("Impossible");
        return;
    }

    loop {
        if current <= 100 {
            let mut temp: i128 = 1000000000000;
            for gas_station in gas_stations.clone() {
                if gas_station.0 <= current {
                    if gas_station.1 < temp {
                        temp = gas_station.1;
                    }
                }else {
                    break;
                }
            }
            min_price += temp * (need - (100 - current));
            break;
        }

        let mut lowest_price: i128 = 1000000000000;
        let mut new_need = need;
        let mut new_current = current;
        let mut new_at_gas_station = at_gas_station;
        for gas_station in gas_stations.clone() {
            if gas_station.0 >= current-200 && gas_station.0 < current{
                let mut price = 0;
                let new_need2;
                let mut max = 200;
                max -= current - gas_station.0;
                
                if at_gas_station.1 < gas_station.1 {
                    price += at_gas_station.1 * need;
                    new_need2 = current - gas_station.0;
                }else if need > max {
                    price += at_gas_station.1 * (need - max);
                    new_need2 = 200;
                }else {
                    new_need2 = current - gas_station.0 + need;
                }

                if price <= lowest_price {
                    lowest_price = price;
                    new_need = new_need2;
                    new_current = gas_station.0;
                    new_at_gas_station = gas_station;
                }
            }else if gas_station.0 >= current{
                break;
            }
        }
        min_price += lowest_price;
        need = new_need;
        current = new_current;
        at_gas_station = new_at_gas_station;
    }

    println!("{}", min_price);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

/* 
Input:    tankar:   i_tank
500      |        | 100
100 999  | 50     | 50
150 888  | 50     | 50
200 777  | 200    | 200
300 999  | 100    | 200
400 1009 | 100    | 200
450 1019 | 0      | 150
500 1399 | 0      | 100
 */
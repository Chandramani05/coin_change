pub fn greed_coin_change(amount :u32) -> Vec<u32>{

    let mut coins = vec![1, 5, 10, 25];
    coins.sort();
    coins.reverse();

    let mut change = vec![];
    let mut remaining = amount;

    for coin in coins {
        while remaining >= coin {
            change.push(coin);
            remaining -= coin;
        }
    }

    change


}
use anyhow::{Context, Result};
use coingecko_rs::CoinGeckoClient;
use ethers::{
    abi::Abi,
    prelude::{Address, Contract, Http, *},
};
use std::{collections::HashMap, fmt};
// use serde_json;

pub mod ethers_helpers {
    use crate::*;

    #[derive(Debug, PartialEq)]
    pub struct Unit {
        integer: u64,
        point: u64,
        decimal_places: usize,
    }

    impl Unit {
        pub fn new(integer: u64, point: u64, decimal_places: usize) -> Unit {
            Unit {
                integer,
                point,
                decimal_places,
            }
        }

        pub fn to_precise_str(&self, precision: usize) -> String {
            format!("{:.precision$}", f64::from(self), precision = precision)
        }
    }

    impl Default for Unit {
        fn default() -> Self {
            Unit {
                integer: 0,
                point: 0,
                decimal_places: 2,
            }
        }
    }

    impl From<f64> for Unit {
        fn from(f: f64) -> Self {
            let fstr = f.to_string();
            let mut iter = fstr.split('.');
            let integer = iter.next().unwrap();
            let point = iter.next().unwrap();
            let decimal_places = point.len();
            Unit {
                integer: integer.parse().unwrap(),
                point: point.parse().unwrap(),
                decimal_places,
            }
        }
    }

    impl From<&Unit> for f64 {
        fn from(u: &Unit) -> Self {
            let integer = u.integer;
            let dividend = f64::from(10).powf(u.decimal_places as f64);
            let point = u.point as f64 / dividend;
            integer as f64 + point
        }
    }

    impl fmt::Display for Unit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}.{:0>width$}",
                self.integer,
                self.point,
                width = self.decimal_places
            )
        }
    }

    pub fn get_provider(rpc_url: &str) -> Provider<Http> {
        Provider::<Http>::try_from(rpc_url).unwrap()
    }

    pub fn get_contract(
        address: &str,
        abi: &str,
        provider: Provider<Http>,
    ) -> Result<Contract<Provider<Http>>> {
        let address: Address = address.parse().context("error parsing contract address")?;
        let abi: Abi = serde_json::from_str(abi).context("error parsing contract ABI")?;

        Ok(Contract::new(address, abi, provider))
    }

    pub fn u256_to_unit(num: U256, decimal_places: usize) -> Unit {
        let dividend: U256 = U256::from(10).pow(U256::from(decimal_places));
        let (integer, point) = num.div_mod(dividend);
        Unit::new(integer.as_u64(), point.as_u64(), decimal_places)
    }

    pub fn calc_price(units: &Unit, price: f64) -> f64 {
        let units: f64 = (units).into();
        units * price
    }

    pub async fn get_prices(token_addresses: Vec<&str>) -> Result<HashMap<String, f64>> {
        let client = CoinGeckoClient::default();
        let result = client
            .token_price(
                "fantom",
                token_addresses,
                vec!["usd"],
                false,
                false,
                false,
                false,
            )
            .await
            .unwrap();

        let result: HashMap<String, f64> = result
            .iter()
            .map(|(k, v)| (k.to_string(), v.usd.unwrap()))
            .collect();
        Ok(result)
    }
}

pub mod fantom;
pub use fantom::tomb_finance;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn u256_to_units() {
        let testint = U256::from(10045);
        let units = ethers_helpers::u256_to_unit(testint, 4);
        assert_eq!("1.0045", units.to_string());
    }

    #[test]
    fn calc_price() {
        let unit = ethers_helpers::Unit::new(2, 5, 2);
        let price = ethers_helpers::calc_price(&unit, 10.0);
        assert_eq!(20.5, price);
    }

    #[test]
    fn f64_to_units() {
        let num = 2.5;
        let unit = ethers_helpers::Unit::new(2, 5, 1);
        assert_eq!(ethers_helpers::Unit::from(num), unit);
    }
}

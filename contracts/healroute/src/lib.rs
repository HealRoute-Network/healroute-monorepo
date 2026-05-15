#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct HealRouteContract;

#[contractimpl]
impl HealRouteContract {
    /// Store a route record keyed by patient ID.
    pub fn set_route(env: Env, patient_id: Symbol, route: Symbol) {
        env.storage().instance().set(&patient_id, &route);
    }

    /// Retrieve a route record by patient ID.
    pub fn get_route(env: Env, patient_id: Symbol) -> Symbol {
        env.storage()
            .instance()
            .get(&patient_id)
            .unwrap_or(symbol_short!("none"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn test_set_and_get_route() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HealRouteContract);
        let client = HealRouteContractClient::new(&env, &contract_id);

        let patient = symbol_short!("p001");
        let route = symbol_short!("routeA");

        client.set_route(&patient, &route);
        assert_eq!(client.get_route(&patient), route);
    }
}

#![no_std]


use soroban_sdk::{contract, contractimpl,  Env, symbol_short, Symbol};
const RESULTADO: Symbol = symbol_short!("RESULTADO");
#[contract]
pub struct Contract;


#[contractimpl]
impl Contract {
    
    pub fn sumar(env: Env, a:i128, b:i128) -> i128 {
      //Implementar función que sume dos números
    }

    pub fn resultado_anterior(env: Env) -> i128 {
           //Implementar función que retorne el valor anterior
    }
}

mod test;
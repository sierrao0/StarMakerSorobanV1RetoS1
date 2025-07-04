#![no_std]


use soroban_sdk::{contract, contractimpl,  Env, symbol_short, Symbol};
const RESULTADO: Symbol = symbol_short!("RESULTADO");
#[contract]
pub struct Contract;


#[contractimpl]
impl Contract {
    
    pub fn sumar(env: Env, a:i128, b:i128) -> i128 {
      //Implementar función que sume dos números
      let resultado: i128 = i128::from(a+b);
      // Guardar el resultado en el almacenamiento persistente
      env.storage().persistent().set(&RESULTADO, &resultado);
      return resultado;
    }

    pub fn resultado_anterior(env: Env) -> i128 {
      //Implementar función que retorne el valor anterior
      let res: i128 = env.storage().persistent().get(&RESULTADO).unwrap_or(0);
      return res;
    }
}

mod test;

//this is a contract I formed from a venmo clone I found online.
//for front-end this should be linked up to the charge button and open a prompt window.
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{env, setup_alloc, near_bindgen, AccountId, Promise};

setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Charge{
    memo: LookupMap<String, Vec<String>>
}
//giving us our default value or balance to fallback on
impl Default for Charge {
    fn default() -> Self {
        Self{
            memo: LookupMap::new(b"memo".to_vec())
        }
    }
} 

#[near_bindgen]
impl Charge{
    ///adding memo and price functionallity 
    pub fn add_memo(&mut self, memo_text: String, price:String){

        let account_id=env::signer_account_id();
        let contains_user=self.memo.contains_key(&account_id);

        if contains_user{
            let mut temp_list=match self.memo.get(&account_id){
                Some(x)=>x,   // x is vector of memos
                None=>vec![] //else this will return an empty vector
            };

            temp_list.push(memo_text + " || " + &price + "Aerx");
            self.memo.insert(&account_id, &temp_list);

            }else{
                let fresh_vec=vec![memo_text + " || " + &price+ "Aerx"];
                self.memo.insert(&account_id, &fresh_vec);
            }
    }
    //transfer function
    pub fn transfer_money(&mut self, account_id: AccountId, amount:f64){
        Promise::new(account_id).transfer(amount as u128);
    }

    //View Methods 
    pub fn get_memos(self, user:String) -> Vec<String>{
        match self.memo.get(&user){
            Some(x)=>x, // vector that contains all memos
            None=>vec![] //else this will return an empty vector
        }
    }
}


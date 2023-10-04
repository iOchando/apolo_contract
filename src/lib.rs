use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault, serde_json::json,
};



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
//#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    owner_id: AccountId
}


#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id: owner_id,
        }
    }

    
    pub fn set_wallet(
        &mut self, 
        wallet: AccountId,
    ) {
        assert_eq!(env::predecessor_account_id(), self.owner_id.clone(), "no autorizado");
        
        env::log_str(
            &json!({
                "wallet": wallet.to_string(),
            })
            .to_string(),
        );
    }

    

}







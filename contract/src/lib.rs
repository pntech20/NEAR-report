/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
    reports: UnorderedMap<usize, Report>,
    owner: AccountId,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Report {
    id: usize,
    author: AccountId,
    done_today: String,
    goal_tomorrow: String,
    blocker: String,
    word_appreciation: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            message: DEFAULT_MESSAGE.to_string(),
            reports: UnorderedMap::new(b"reports".to_vec()),
            owner: env::signer_account_id(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    // add a report
    pub fn add_report(
        &mut self,
        done_today: String,
        goal_tomorrow: String,
        blocker: String,
        word_appreciation: String,
    ) -> usize {
        let report = Report {
            id: self.reports.len() as usize,
            author: env::signer_account_id(),
            done_today,
            goal_tomorrow,
            blocker,
            word_appreciation,
        };
        self.reports.insert(&report.id, &report);
        report.id
    }

    // get a report
    pub fn get_report(&self, id: usize) -> Report {
        self.reports.get(&id).unwrap()
    }

    // update a report
    pub fn update_report(
        &mut self,
        id: usize,
        done_today: String,
        goal_tomorrow: String,
        blocker: String,
        word_appreciation: String,
    ) {
        let report = Report {
            id,
            author: env::signer_account_id(),
            done_today,
            goal_tomorrow,
            blocker,
            word_appreciation,
        };

        self.reports.insert(&id, &report);
        // self.reports.remove(&id);
        // self.reports.insert(&id, &report);
    }

    // delete a report
    pub fn delete_report(&mut self, id: usize) {
        // Check if current user is NOT author
        let user = env::signer_account_id();
        assert_eq!(user, self.owner, "Only author can delete post");

        self.reports.remove(&id);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello".to_string());
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy".to_string());
    }

    #[test]
    fn add_report() {
        let mut contract = Contract::default();
        let report_id = contract.add_report(
            "done today".to_string(),
            "goal tomorrow".to_string(),
            "blocker".to_string(),
            "word appreciation".to_string(),
        );
        assert_eq!(report_id, 0);
        assert_eq!(contract.reports.len(), 1);
    }

    #[test]
    fn get_report() {
        let mut contract = Contract::default();
        let report_id = contract.add_report(
            "done today".to_string(),
            "goal tomorrow".to_string(),
            "blocker".to_string(),
            "word appreciation".to_string(),
        );
        let report = contract.get_report(report_id);
        assert_eq!(report.id, 0);
        assert_eq!(report.author, env::signer_account_id());
        assert_eq!(report.done_today, "done today".to_string());
        assert_eq!(report.goal_tomorrow, "goal tomorrow".to_string());
        assert_eq!(report.blocker, "blocker".to_string());
        assert_eq!(report.word_appreciation, "word appreciation".to_string());
    }

    #[test]
    fn update_report() {
        let mut contract = Contract::default();
        let report_id = contract.add_report(
            "done today".to_string(),
            "goal tomorrow".to_string(),
            "blocker".to_string(),
            "word appreciation".to_string(),
        );
        assert_eq!(contract.reports.len(), 1);
        contract.update_report(
            report_id,
            "updated done today".to_string(),
            "updated goal tomorrow".to_string(),
            "updated blocker".to_string(),
            "updated word appreciation".to_string(),
        );
        let report = contract.get_report(report_id);
        assert_eq!(report.id, 0);
        assert_eq!(report.author, env::signer_account_id());
        assert_eq!(report.done_today, "updated done today".to_string());
        assert_eq!(report.goal_tomorrow, "updated goal tomorrow".to_string());
        assert_eq!(report.blocker, "updated blocker".to_string());
        assert_eq!(
            report.word_appreciation,
            "updated word appreciation".to_string()
        );
    }

    #[test]
    fn delete_report() {
        let mut contract = Contract::default();
        let report_id = contract.add_report(
            "done today".to_string(),
            "goal tomorrow".to_string(),
            "blocker".to_string(),
            "word appreciation".to_string(),
        );
        assert_eq!(contract.reports.len(), 1);
        contract.delete_report(report_id);
        assert_eq!(contract.reports.len(), 0);
    }
}

extern crate serde_json as json;
extern crate stripe;

use std::env;

fn customer_create_and_delete(client: &stripe::Client) {
    let customer_params = stripe::CustomerParams {
        account_balance: None,
        business_vat_id: None,
        default_source: None,
        coupon: None,
        description: None,
        email: None,
        metadata: None,
        shipping: None,
        source: None,
    };
    let customer = stripe::Customer::create(client, customer_params).unwrap();
    let result = stripe::Customer::delete(client, &customer.id);
    match result {
        Ok(deleted) => assert!(deleted.deleted, "Customer wasn't deleted"),
        Err(err) => assert!(false, format!("{}", err)),
    }

}

#[test]
fn customer_create_and_delete_without_account() {
    let sk = env::var("STRIPE_SK").unwrap();
    let client = stripe::Client::new(sk);
    customer_create_and_delete(&client)
}

#[test]
fn customer_create_and_delete_with_account() {
    let sk = env::var("STRIPE_SK").unwrap();
    let account = env::var("STRIPE_ACCOUNT").unwrap();
    let params = stripe::Params { stripe_account: Some(account) };
    let client = stripe::Client::new(sk).with(params);
    customer_create_and_delete(&client)
}

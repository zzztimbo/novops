use std::collections::HashMap;
use serde::Deserialize;
use crate::bitwarden;

#[derive(Debug, Deserialize)]
pub struct NovopsConfig {
    pub environments: HashMap<String, NovopsEnvironment>
}

#[derive(Debug, Deserialize)]
pub struct NovopsEnvironment {
    pub variables: HashMap<String, NovopsValue>,
    pub files: HashMap<String, NovopsFile>
}

#[derive(Debug, Deserialize)]
pub struct NovopsFile {
    pub dest: String,
    pub content: NovopsValue
}

/**
 * A Novops value is it's core: in can be a string, a secret, or something else
 */
#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[enum_dispatch(ResolvableNovopsValue)]
pub enum NovopsValue {
    String(String), 
    StringValue(StringValue),
    BitwardenItem(BitwardenItem)
}

#[enum_dispatch]
pub trait ResolvableNovopsValue {
    fn resolve(&self) -> String;
}

impl ResolvableNovopsValue for String {
    fn resolve(&self) -> String {
        return self.clone()
    }
}

/**
 * A string set with 'value' key such as 
 * 
 * myvar:
 *   value: foo
 */

#[derive(Debug, Deserialize)]
pub struct StringValue{
    value: String
}

impl ResolvableNovopsValue for StringValue {
    fn resolve(&self) -> String {
        return self.value.clone();
    }
}

/**
 * A BitWarden secret such as
 * 
 * myvar:
 *   bitwarden:
 *     entry: wordpress_prod
 *     field: login.password
 */

#[derive(Debug, Deserialize)]
pub struct BitwardenItem {
    bitwarden: BitwardenValue,
}

impl ResolvableNovopsValue for BitwardenItem {
    fn resolve(&self) -> String {
        let json_value = bitwarden::get_item(&self.bitwarden.entry).expect(&String::from("Error fetching Bitwarden entry"));

        // Novops config let use specify a string like "login.password"
        // we need to retrieve this field nexted in our JSON (or fail if not found)
        let fields = self.bitwarden.field.split(".").map(|s| String::from(s)).collect();
        let val = bitwarden::get_string_in_value(&json_value, fields);
        
        return val.expect("Couldn't get value from Bitwarden entry").to_string();
    }
}


#[derive(Debug, Deserialize)]
pub struct BitwardenValue {
    entry: String,
    field: String
}
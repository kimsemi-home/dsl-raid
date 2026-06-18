mod gate;
mod profile;
mod reliability;
mod sensitive;

use profile::ProducerProfile;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let producer = ProducerProfile::new(value);
    gate::push_issues(&producer, issues);
    reliability::push_issues(value, &producer, issues);
    sensitive::push_issues(&producer, issues);
}

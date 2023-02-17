use serde::Deserialize;

macro_rules! healthchecks {
    ($( $healthcheck_name:ident { $( $field:ident: $field_type:ty ),* }, $config_name:ident ),*) => {
        // Define structs for each healthcheck
        $(
            #[derive(Debug, Deserialize)]
            pub struct $healthcheck_name {
                name: Option<String>,
                $( $field: $field_type, )*
            }
        )*

        // Define a Configuration struct that includes each healthcheck
        #[derive(Debug, Deserialize)]
        pub struct Configuration {
            name: Option<String>,
            interval: Option<u32>,
            handlers: Option<Vec<Handler>>,

            $( pub $config_name: Option<Vec<$healthcheck_name>>, )*
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct Handler {
    name: Option<String>,
    command: String,
    state: HealthCheckState,
    timeout: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub enum HealthCheckState {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "failed")]
    Failed,
}

healthchecks!(
    Http {
        url: String,
        timeout: Option<u32>
    }, http
);

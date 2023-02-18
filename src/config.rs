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

        // generate getters for name, interval, handlers, and all healthchecks
        impl Configuration {
            pub fn name(&self) -> Option<&String> {
                self.name.as_ref()
            }

            pub fn interval(&self) -> Option<u32> {
                self.interval
            }

            pub fn handlers(&self) -> Option<&Vec<Handler>> {
                self.handlers.as_ref()
            }

            $(
                pub fn $config_name(&self) -> Option<&Vec<$healthcheck_name>> {
                    self.$config_name.as_ref()
                }
            )*
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

impl Handler {
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn command(&self) -> &str {
        self.command.as_ref()
    }

    pub fn state(&self) -> &HealthCheckState {
        &self.state
    }

    pub fn timeout(&self) -> Option<u32> {
        self.timeout
    }
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

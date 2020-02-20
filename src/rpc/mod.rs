mod client;

pub use client::Client;

pub enum RPCType {
    JsonRPC,
    MsgPack,
}

macro_rules! gen_rpc {
    ($client:ident, $mime:expr) => {
        pub struct $client {
            pub id: u64,
            pub endpoint: String,
        }

        impl $client {
            pub fn mimetype() -> String {
                $mime
            }

            pub fn build(mut self) -> Self {
                self.id = 1;
                self.endpoint = String::from("http://localhost:5993/rpc");
                return self;
            }

            pub fn with_endpoint(mut self, endpoint: String) -> Self {
                self.endpoint = endpoint;
                return self;
            }
        }
    };
}

gen_rpc!(JsonRPC, "application/json".into());
gen_rpc!(MsgPack, "application/x-msgpack".into());

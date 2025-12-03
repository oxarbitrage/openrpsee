use std::{
    borrow::Cow,
};

use documented::Documented;
use jsonrpsee::core::{JsonValue, RpcResult};
use schemars::{JsonSchema, Schema, SchemaGenerator, generate::SchemaSettings};
use serde::Serialize;


/// Response to an `rpc.discover` RPC request.
pub type Response = RpcResult<ResultType>;
pub type ResultType = OpenRpc;

/// Static information about a Zallet JSON-RPC method.
pub struct RpcMethod {
    pub description: &'static str,
    pub params: fn(&mut Generator) -> Vec<ContentDescriptor>,
    pub result: fn(&mut Generator) -> ContentDescriptor,
    pub deprecated: bool,
}

impl RpcMethod {
    pub fn generate(&self, generator: &mut Generator, name: &'static str) -> Method {
        let description = self.description.trim();

        Method {
            name,
            summary: description
                .split_once('\n')
                .map(|(summary, _)| summary)
                .unwrap_or(description),
            description,
            params: (self.params)(generator),
            result: (self.result)(generator),
            deprecated: self.deprecated,
        }
    }
}

/// An OpenRPC document generator.
pub struct Generator {
    inner: SchemaGenerator,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            inner: SchemaSettings::draft07()
                .with(|s| {
                    s.definitions_path = "#/components/schemas/".into();
                })
                .into_generator(),
        }
    }

    /// Constructs the descriptor for a JSON-RPC method parameter.
    pub fn param<T: JsonSchema>(
        &mut self,
        name: &'static str,
        description: &'static str,
        required: bool,
    ) -> ContentDescriptor {
        ContentDescriptor {
            name,
            summary: description
                .split_once('\n')
                .map(|(summary, _)| summary)
                .unwrap_or(description),
            description,
            required,
            schema: self.inner.subschema_for::<T>(),
            deprecated: false,
        }
    }

    /// Constructs the descriptor for a JSON-RPC method's result type.
    pub fn result<T: Documented + JsonSchema>(
        &mut self,
        name: &'static str,
    ) -> ContentDescriptor {
        ContentDescriptor {
            name,
            summary: T::DOCS
                .split_once('\n')
                .map(|(summary, _)| summary)
                .unwrap_or(T::DOCS),
            description: T::DOCS,
            required: false,
            schema: self.inner.subschema_for::<T>(),
            deprecated: false,
        }
    }

    pub fn into_components(mut self) -> Components {
        Components {
            schemas: self.inner.take_definitions(false),
        }
    }
}

/// An OpenRPC document.
#[derive(Clone, Debug, Serialize, Documented)]
pub struct OpenRpc {
    pub openrpc: &'static str,
    pub info: Info,
    pub methods: Vec<Method>,
    pub components: Components,
}

impl JsonSchema for OpenRpc {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("OpenRPC Schema")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::new_ref(
            "https://raw.githubusercontent.com/open-rpc/meta-schema/master/schema.json".into(),
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub title: &'static str,
    pub description: &'static str,
    pub version: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct Method {
    name: &'static str,
    summary: &'static str,
    description: &'static str,
    params: Vec<ContentDescriptor>,
    result: ContentDescriptor,
    #[serde(skip_serializing_if = "is_false")]
    deprecated: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct ContentDescriptor {
    name: &'static str,
    summary: &'static str,
    description: &'static str,
    #[serde(skip_serializing_if = "is_false")]
    required: bool,
    schema: Schema,
    #[serde(skip_serializing_if = "is_false")]
    deprecated: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Components {
    schemas: serde_json::Map<String, JsonValue>,
}

fn is_false(b: &bool) -> bool {
    !b
}

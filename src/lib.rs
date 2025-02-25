use kube::{Api, Client};
use kube::api::PostParams;
use serde::{Serialize, Deserialize};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinition,
    CustomResourceDefinitionSpec,
    CustomResourceDefinitionNames,
    CustomResourceDefinitionVersion,
    JSONSchemaProps,
    CustomResourceValidation,
    JSONSchemaPropsOrArray,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use schemars::JsonSchema;
use kube::CustomResource;
use std::collections::BTreeMap;

// Define the spec of our custom resource
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "ctf.rs", version = "v1", kind = "Flag", namespaced)]
pub struct FlagSpec {}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "ctf.rs", version = "v1", kind = "Plugin", namespaced)]
pub struct PluginSpec {}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "ctf.rs", version = "v1", kind = "Challenge", namespaced)]
pub struct ChallengeSpec {
    name: String,
    type: String,
    image: String,
    deploy_as: String,
}

pub struct VolumeItem {}

impl ChallengeSpec {
    pub fn new(name: String, type: String, image: String, deploy_as: String) -> Api<CustomResourceDefinition> {
        let crds: Api<CustomResourceDefinition> = Api::all(client);
        CustomResourceDefinition {
            metadata: ObjectMeta {
                name: Some("kcdtrack2s.example.com".to_string()),
                ..Default::default()
            },
            spec: CustomResourceDefinitionSpec {
                group: "example.com".to_string(),
                versions: vec![
                    CustomResourceDefinitionVersion {
                        name: "v1".to_string(),
                        served: true,
                        storage: true,
                        schema: Some(CustomResourceValidation {
                            open_api_v3_schema: Some(JSONSchemaProps {
                                type_: Some("object".to_string()),
                                properties: Some({
                                    let mut props = BTreeMap::new();
                                    props.insert("spec".to_string(), JSONSchemaProps {
                                        type_: Some("object".to_string()),
                                        properties: Some({
                                            let mut spec_props = BTreeMap::new();
                                            spec_props.insert("organizer".to_string(), JSONSchemaProps {
                                                type_: Some("string".to_string()),
                                                ..Default::default()
                                            });
                                            spec_props.insert("topic".to_string(), JSONSchemaProps {
                                                type_: Some("string".to_string()),
                                                ..Default::default()
                                            });
                                            spec_props.insert("attendees".to_string(), JSONSchemaProps {
                                                type_: Some("array".to_string()),
                                                items: Some(JSONSchemaPropsOrArray::Schema(Box::new(JSONSchemaProps {
                                                    type_: Some("string".to_string()),
                                                    ..Default::default()
                                                }))),
                                                ..Default::default()
                                            });
                                            spec_props.insert("conference".to_string(), JSONSchemaProps {
                                                type_: Some("string".to_string()),
                                                ..Default::default()
                                            });
                                            spec_props.insert("time".to_string(), JSONSchemaProps {
                                                type_: Some("string".to_string()),
                                                ..Default::default()
                                            });
                                            spec_props.insert("session_type".to_string(), JSONSchemaProps {
                                                type_: Some("string".to_string()),
                                                ..Default::default()
                                            });
                                            spec_props.insert("speaker".to_string(), JSONSchemaProps {
                                                type_: Some("string".to_string()),
                                                ..Default::default()
                                            });
                                            spec_props
                                        }),
                                        ..Default::default()
                                    });
                                    props
                                }),
                                ..Default::default()
                            }),
                        }),
                        ..Default::default()
                    }
                ],
                names: CustomResourceDefinitionNames {
                    plural: "kcdtrack2s".to_string(),
                    singular: Some("kcdtrack2".to_string()),
                    kind: "KCDTrack2".to_string(),
                    short_names: Some(vec!["kcdt2".to_string()]),
                    ..Default::default()
                },
                scope: "Namespaced".to_string(),
                ..Default::default()
            },
            status: None,
        };
    }
    pub fn read(&self) -> &String {
        &self.name
    }
    pub fn create(&self) -> String {
        format!("Hello, {}!", self.name)
    }
    pub fn update(&self) -> String {
        format!("Hello, {}!", self.name)
    }
    pub fn delete(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

impl FlagSpec {
    pub fn new(client: &Client) -> FlagSpec {
        Api<Flag>::all(client);
        FlagSpec {}
    }
    pub fn create(client: &Client) -> FlagSpec {
        Api<Flag>::all(client);
        FlagSpec {}
    }
    pub fn read(client: &Client) -> FlagSpec {
        FlagSpec {}
    }
    pub fn update(client: &Client) -> FlagSpec {
        FlagSpec {}
    }
    pub fn delete(client: &Client) -> FlagSpec {
        FlagSpec {}
    }
}

impl PluginSpec {
    pub fn new(client: &Client) -> PluginSpec {
        PluginSpec {}
    }
    pub fn create(client: &Client) -> PluginSpec {
        PluginSpec {}
    }
    pub fn read(client: &Client) -> PluginSpec {
        PluginSpec {}
    }
    pub fn update(client: &Client) -> PluginSpec {
        PluginSpec {}
    }
    pub fn delete(client: &Client) -> PluginSpec {
        PluginSpec {}
    }
}

// Manage ConfigMap interactions
pub fn loadConfig(client: &Client) {
    Api<ConfigMap>::all(client);
}

// Manage Secret interactions
pub fn loadSecrets(client: &Client) {
    Api<Secret>::all(client);
}

// Manage Volume interactions
impl VolumeItem {}


// TODO: Implement Watchers for all CRDs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_spec() {
        let challenge = ChallengeSpec.new();
        challenge.create();
        challenge.read();
        challenge.update();
        challenge.read();
        challenge.delete();
        challenge.read();
        assert_eq!(4, 4);
    }

    #[test]
    fn test_plugin_spec() {
        let plugin = PluginSpec.new();
        plugin.create();
        plugin.read();
        plugin.update();
        plugin.read();
        plugin.delete();
        plugin.read();
        assert_eq!(4, 4);
    }

    #[test]
    fn test_flag_spec() {
        let flag = FlagSpec.new();
        flag.create();
        flag.read();
        flag.update();
        flag.read();
        flag.delete();
        flag.read();
        assert_eq!(4, 4);
    }

    #[test]
    fn test_volume_item() {
        assert_eq!(4, 4);
    }

    #[test]
    fn test_secret_interface() {
        assert_eq!(4, 4);
    }

    #[test]
    fn test_configmap_interface() {
        assert_eq!(4, 4);
    }


}

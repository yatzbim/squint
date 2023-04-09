// Contains traits specifying database behaviors, and contains submodules implementing those behaviors in specific integrations
// The plan is to start with an integration to local BonsaiDB and maybe one day migrate to cloud hosting
// TODO:  implement

#[path = "bonsai-local.rs"]
mod bonsai_local;

//! Factory semantic core, authority and execution-intelligence boundaries.

pub mod authority;
pub mod build;
pub mod build_cognitive;
pub mod build_provider;
pub mod core;
pub mod execution_intelligence;
pub mod journey;
pub mod project_development;
pub mod structural_ground;

impl AsRef<core::identity::Ref> for journey::JourneyRef {
    fn as_ref(&self) -> &core::identity::Ref {
        journey::JourneyRef::as_ref(self)
    }
}

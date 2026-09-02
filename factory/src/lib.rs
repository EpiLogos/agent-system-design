//! Factory semantic core, authority and execution-intelligence boundaries.

pub mod authority;
pub mod build;
pub mod build_cognitive;
pub mod build_provider;
pub mod core;
pub mod execution_intelligence;
pub mod git_development;
// JourneyRef implements the standard AsRef trait below. Keep the legacy inherent
// accessor during this additive contract tranche without weakening any other lint.
#[allow(clippy::should_implement_trait)]
pub mod journey;
pub mod journey_commission;
pub mod project_development;
pub mod structural_ground;

impl AsRef<core::identity::Ref> for journey::JourneyRef {
    fn as_ref(&self) -> &core::identity::Ref {
        journey::JourneyRef::as_ref(self)
    }
}

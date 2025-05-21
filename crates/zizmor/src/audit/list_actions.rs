use anyhow::Result;
use github_actions_models::common::Uses;

use super::{Audit, AuditLoadError, audit_meta};
use crate::finding::{Confidence, Finding, Severity};
use crate::models::{CompositeStep, Step, StepCommon};
use crate::state::AuditState;

pub(crate) struct ListActions {}

audit_meta!(
    ListActions,
    "list-actions",
    "list all actions used in workflows"
);

impl Audit for ListActions {
    fn new(_state: &AuditState<'_>) -> Result<Self, AuditLoadError> {
        Ok(Self {})
    }

    fn audit_step<'doc>(&self, step: &Step<'doc>) -> Result<Vec<Finding<'doc>>> {
        let mut findings = vec![];
        
        if let Some(Uses::Repository(repo_uses)) = step.uses() {
            let action_ref = format!("{}/{}", repo_uses.owner, repo_uses.repo);
            let action_ref = if let Some(subpath) = &repo_uses.subpath {
                format!("{}/{}", action_ref, subpath)
            } else {
                action_ref
            };
            let action_ref = if let Some(git_ref) = &repo_uses.git_ref {
                format!("{}@{}", action_ref, git_ref)
            } else {
                action_ref
            };

            findings.push(
                Self::finding()
                    .confidence(Confidence::High)
                    .severity(Severity::Informational)
                    .add_location(
                        step.location()
                            .primary()
                            .with_keys(&["uses".into()])
                            .annotated(action_ref),
                    )
                    .build(step.workflow())?
            );
        }

        Ok(findings)
    }

    fn audit_composite_step<'doc>(&self, step: &CompositeStep<'doc>) -> Result<Vec<Finding<'doc>>> {
        let mut findings = vec![];

        if let Some(Uses::Repository(repo_uses)) = step.uses() {
            let action_ref = format!("{}/{}", repo_uses.owner, repo_uses.repo);
            let action_ref = if let Some(subpath) = &repo_uses.subpath {
                format!("{}/{}", action_ref, subpath)
            } else {
                action_ref
            };
            let action_ref = if let Some(git_ref) = &repo_uses.git_ref {
                format!("{}@{}", action_ref, git_ref)
            } else {
                action_ref
            };

            findings.push(
                Self::finding()
                    .confidence(Confidence::High)
                    .severity(Severity::Informational)
                    .add_location(
                        step.location()
                            .primary()
                            .with_keys(&["uses".into()])
                            .annotated(action_ref)
                    )
                    .build(step.action())?
            );
        }

        Ok(findings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AuditState;
    use crate::test_utils::*;

    #[test]
    fn test_list_actions_basic() {
        let state = AuditState::test();
        let audit = ListActions::new(&state).unwrap();
        
        let workflow = test_workflow(
            r#"
            on: push
            jobs:
              test:
                runs-on: ubuntu-latest
                steps:
                  - uses: actions/checkout@v2
                  - uses: actions/setup-node@v1
            "#,
        );

        let findings = audit.audit_workflow(&workflow).unwrap();
        assert_eq!(findings.len(), 2);
    }
}
// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[doc(hidden)]
#[macro_export]
macro_rules! decl_recruit_catalog_entry {
  ($building:ident) => {
    paste::paste! {
      #[derive(Clone, Debug, Deserialize, Serialize)]
      #[serde(tag = "kind", rename_all = "kebab-case")]
      pub enum [<$building RecruitCatalogEntry>] {
        /// Unit is available for recruitment.
        Available { recipe: Box<[<$building RecruitCatalogRecipe>]> },
        /// City does not meet the requirements for recruitment.
        Unmet {
          requirements: InfrastructureRequirements,
        },
      }

      impl [<$building RecruitCatalogEntry>] {
        fn new(unit: &dyn Unit, infrastructure: &Infrastructure) -> Self {
          let infra_req = unit.infrastructure_requirements();
          if !infra_req.has_required_levels(infrastructure) {
            return Self::Unmet { requirements: infra_req.clone() };
          }

          let chunk = unit.chunk();
          let recipe = Box::new([<$building RecruitCatalogRecipe>] {
            resources: chunk.resources(),
            maintenance: chunk.maintenance(),
            workforce: chunk.workforce(),
            requirements: infra_req.clone(),
          });

          Self::Available { recipe }
        }
      }

      #[derive(Clone, Debug, Deserialize, Serialize)]
      #[serde(rename_all = "camelCase")]
      pub struct [<$building RecruitCatalogRecipe>] {
        resources: Resources,
        maintenance: Maintenance,
        workforce: Workforce,
        requirements: InfrastructureRequirements,
      }
    }
  };
}

use std::collections::HashSet;

use fj_math::{Point, Scalar};

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{stores::Stores, Handle};

pub trait Validate {
    fn validate(
        &self,
        min_distance: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError>;
}

impl Validate for Point<3> {
    fn validate(&self, _: Scalar, _: &Stores) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Curve {
    fn validate(&self, _: Scalar, _: &Stores) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Surface {
    fn validate(&self, _: Scalar, _: &Stores) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Vertex {
    /// Validate the vertex
    ///
    /// # Implementation note
    ///
    /// In the future, this method is likely to validate more than it already
    /// does. See documentation of [`crate::kernel`] for some context on that.
    fn validate(
        &self,
        min_distance: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        if !stores.points.contains(&self.point) {
            return Err(StructuralIssues::default().into());
        }
        for existing in stores.vertices.iter() {
            let distance = (existing.get().point() - self.point()).magnitude();

            if distance < min_distance {
                return Err(ValidationError::Uniqueness);
            }
        }

        Ok(())
    }
}

impl Validate for Edge {
    fn validate(
        &self,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        let mut missing_curve = None;
        let mut missing_vertices = HashSet::new();

        if !stores.curves.contains(&self.curve) {
            missing_curve = Some(self.curve.clone());
        }
        for vertices in &self.vertices {
            for vertex in vertices {
                if !stores.vertices.contains(vertex) {
                    missing_vertices.insert(vertex.clone());
                }
            }
        }

        if missing_curve.is_some() || !missing_vertices.is_empty() {
            return Err(StructuralIssues {
                missing_curve,
                missing_vertices,
                ..StructuralIssues::default()
            }
            .into());
        }

        Ok(())
    }
}

impl Validate for Cycle {
    /// Validate the cycle
    ///
    /// # Implementation note
    ///
    /// The validation of the cycle should be extended to cover more cases:
    /// - That those edges form a cycle.
    /// - That the cycle is not self-overlapping.
    /// - That there exists no duplicate cycle, with the same edges.
    fn validate(
        &self,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        let mut missing_edges = HashSet::new();
        for edge in &self.edges {
            if !stores.edges.contains(edge) {
                missing_edges.insert(edge.clone());
            }
        }

        if !missing_edges.is_empty() {
            return Err(StructuralIssues {
                missing_edges,
                ..StructuralIssues::default()
            }
            .into());
        }

        Ok(())
    }
}

impl Validate for Face {
    fn validate(
        &self,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        if let Face::Face {
            surface,
            exteriors,
            interiors,
            ..
        } = self
        {
            let mut missing_surface = None;
            let mut missing_cycles = HashSet::new();

            if !stores.surfaces.contains(surface) {
                missing_surface = Some(surface.clone());
            }
            for cycle in exteriors.iter().chain(interiors) {
                if !stores.cycles.contains(cycle) {
                    missing_cycles.insert(cycle.clone());
                }
            }

            if missing_surface.is_some() || !missing_cycles.is_empty() {
                return Err(StructuralIssues {
                    missing_surface,
                    missing_cycles,
                    ..StructuralIssues::default()
                }
                .into());
            }
        }

        Ok(())
    }
}

/// Returned by the various `add_` methods of the [`Shape`] API
pub type ValidationResult<T> = Result<Handle<T>, ValidationError>;

/// An error that can occur during a validation
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Structural validation failed
    ///
    /// Structural validation verifies, that all the object that an object
    /// refers to are already part of the shape.
    #[error("Structural validation failed")]
    Structural(StructuralIssues),

    /// Uniqueness validation failed
    ///
    /// Uniqueness validation checks, that an object is unique. Uniqueness is
    /// only required for topological objects, as there's no harm in geometric
    /// objects being duplicated.
    #[error("Uniqueness validation failed")]
    #[allow(unused)]
    Uniqueness,

    /// Geometric validation failed
    ///
    /// Geometric validation checks, that various geometric constraints of an
    /// object are upheld. For example, edges or faces might not be allowed to
    /// intersect.
    #[error("Geometric validation failed")]
    #[allow(unused)]
    Geometric,
}

impl ValidationError {
    /// Indicate whether validation found a missing curve
    #[cfg(test)]
    pub fn missing_curve(&self, curve: &Handle<Curve>) -> bool {
        if let Self::Structural(StructuralIssues { missing_curve, .. }) = self {
            return missing_curve.as_ref() == Some(curve);
        }

        false
    }

    /// Indicate whether validation found a missing vertex
    #[cfg(test)]
    pub fn missing_vertex(&self, vertex: &Handle<Vertex>) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_vertices, ..
        }) = self
        {
            return missing_vertices.contains(vertex);
        }

        false
    }

    /// Indicate whether validation found a missing edge
    #[cfg(test)]
    pub fn missing_edge(&self, edge: &Handle<Edge>) -> bool {
        if let Self::Structural(StructuralIssues { missing_edges, .. }) = self {
            return missing_edges.contains(edge);
        }

        false
    }

    /// Indicate whether validation found a missing surface
    #[cfg(test)]
    pub fn missing_surface(&self, surface: &Handle<Surface>) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_surface, ..
        }) = self
        {
            return missing_surface.as_ref() == Some(surface);
        }

        false
    }

    /// Indicate whether validation found a missing cycle
    #[cfg(test)]
    pub fn missing_cycle(&self, cycle: &Handle<Cycle>) -> bool {
        if let Self::Structural(StructuralIssues { missing_cycles, .. }) = self
        {
            return missing_cycles.contains(cycle);
        }

        false
    }
}

impl From<StructuralIssues> for ValidationError {
    fn from(issues: StructuralIssues) -> Self {
        Self::Structural(issues)
    }
}

/// Structural issues found during validation
///
/// Used by [`ValidationError`].
#[derive(Debug, Default)]
pub struct StructuralIssues {
    /// Missing curve found in edge validation
    pub missing_curve: Option<Handle<Curve>>,

    /// Missing vertices found in edge validation
    pub missing_vertices: HashSet<Handle<Vertex>>,

    /// Missing edges found in cycle validation
    pub missing_edges: HashSet<Handle<Edge>>,

    /// Missing surface found in face validation
    pub missing_surface: Option<Handle<Surface>>,

    /// Missing cycles found in face validation
    pub missing_cycles: HashSet<Handle<Cycle>>,
}

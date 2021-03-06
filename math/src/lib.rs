extern crate cgmath;
extern crate num_traits;

pub mod prelude {
    pub use super::InnerSpaceExt as MathPreludeInnerSpaceExt;
    pub use cgmath::prelude::{Angle as MathPreludeAngle, Array as MathPreludeArray,
                              ElementWise as MathPreludeElementWise,
                              EuclideanSpace as MathPreludeEuclideanSpace,
                              InnerSpace as MathPreludeInnerSpace, Matrix as MathPreludeMatrix,
                              MetricSpace as MathPreludeMetricSpace,
                              Rotation as MathPreludeRotation, Rotation2 as MathPreludeRotation2,
                              Rotation3 as MathPreludeRotation3,
                              SquareMatrix as MathPreludeSquareMatrix,
                              Transform as MathPreludeTransform,
                              Transform2 as MathPreludeTransform2,
                              Transform3 as MathPreludeTransform3,
                              VectorSpace as MathPreludeVectorSpace, Zero as MathPreludeZero};
}

pub use cgmath::{Angle, Array, ElementWise, EuclideanSpace, InnerSpace, Matrix, MetricSpace,
                 Rotation, Rotation2, Rotation3, SquareMatrix, Transform, Transform2, Transform3,
                 VectorSpace, Decomposed, Deg, Rad, BaseFloat, BaseNum, ortho, frustum,
                 perspective, vec2, vec3, vec4, Euler, ApproxEq};
pub use num_traits::{NumCast, Float};

pub mod contact;
pub mod line;
pub mod sphere;

pub use self::contact::ContactInfo;
pub use self::line::Line2;
pub use self::sphere::Sphere;

pub trait InnerSpaceExt: InnerSpace
where
    Self::Scalar: BaseFloat,
{
    fn normalize_or_zero(self) -> Self {
        self / self.magnitude().max(Self::Scalar::default_epsilon())
    }

    fn normalize_or_zero_self(&mut self) {
        *self = *self / self.magnitude().max(Self::Scalar::default_epsilon())
    }
}

impl<T: InnerSpace> InnerSpaceExt for T
where
    Self::Scalar: BaseFloat,
{
}

pub type Vec2<T> = cgmath::Vector2<T>;
pub type Vec3<T> = cgmath::Vector3<T>;
pub type Vec4<T> = cgmath::Vector4<T>;

pub type Pnt2<T> = cgmath::Point2<T>;
pub type Pnt3<T> = cgmath::Point3<T>;

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

pub type Pnt2f = Pnt2<f32>;
pub type Pnt3f = Pnt3<f32>;

pub type Mat4 = cgmath::Matrix4<f32>;
pub type Quat = cgmath::Quaternion<f32>;

pub type Trans3 = cgmath::Decomposed<Vec3f, Quat>;
pub type Line2f = Line2<f32>;

pub type Degf = Deg<f32>;
pub type Radf = Rad<f32>;
